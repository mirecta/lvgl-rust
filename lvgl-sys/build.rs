//! Build script for lvgl-sys
//!
//! This script:
//! 1. Downloads LVGL C sources if not present
//! 2. Compiles LVGL C sources
//! 3. Generates Rust FFI bindings via bindgen
//!
//! Platform-independent: works on desktop, ESP32, and any target with a C compiler.
//! Cross-compilation is handled via standard env vars (CC, CFLAGS, BINDGEN_EXTRA_CLANG_ARGS).
//!
//! LVGL source resolution order:
//! 1. `LVGL_PATH` env var (explicit path to LVGL source)
//! 2. `lvgl/` directory next to the workspace root (for development)
//! 3. Auto-download from GitHub into OUT_DIR (for dependency usage)

use std::env;
use std::path::PathBuf;
use std::process::Command;

const LVGL_VERSION: &str = "v9.2.2";
const LVGL_REPO: &str = "https://github.com/lvgl/lvgl.git";

fn main() {
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());

    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
    let is_simulator = env::var("CARGO_FEATURE_SIMULATOR").is_ok();

    // Resolve LVGL source path (auto-downloads if needed)
    let lvgl_path = resolve_lvgl_path(&manifest_dir, &out_path);

    // Select appropriate config file
    let config_path = if is_simulator {
        let sim_config = manifest_dir.join("lv_conf_simulator.h");
        if sim_config.exists() {
            manifest_dir.clone()
        } else {
            PathBuf::from(
                env::var("DEP_LV_CONFIG_PATH")
                    .unwrap_or_else(|_| manifest_dir.to_string_lossy().into_owned()),
            )
        }
    } else {
        PathBuf::from(
            env::var("DEP_LV_CONFIG_PATH")
                .unwrap_or_else(|_| manifest_dir.to_string_lossy().into_owned()),
        )
    };

    // For simulator, copy simulator config to lv_conf.h in OUT_DIR
    let lv_conf_name = if is_simulator && config_path.join("lv_conf_simulator.h").exists() {
        let src = config_path.join("lv_conf_simulator.h");
        let dst = out_path.join("lv_conf.h");
        std::fs::copy(&src, &dst).expect("Failed to copy lv_conf_simulator.h");
        out_path.clone()
    } else {
        config_path.clone()
    };

    println!("cargo:rerun-if-changed=wrapper.h");
    println!("cargo:rerun-if-changed=lv_conf.h");
    println!("cargo:rerun-if-changed=lv_conf_simulator.h");
    println!("cargo:rerun-if-env-changed=LVGL_PATH");
    println!("cargo:rerun-if-env-changed=DEP_LV_CONFIG_PATH");

    // Collect LVGL source files
    let lvgl_sources: Vec<PathBuf> = glob::glob(&format!("{}/src/**/*.c", lvgl_path.display()))
        .expect("Failed to glob LVGL sources")
        .filter_map(|e| e.ok())
        .collect();

    if lvgl_sources.is_empty() {
        panic!(
            "No LVGL sources found at {}. This should not happen if auto-download succeeded.",
            lvgl_path.display()
        );
    }

    // Compile LVGL
    // The cc crate automatically picks up the correct compiler from env vars:
    // CC, CC_<target>, TARGET_CC, CFLAGS, etc.
    let mut build = cc::Build::new();

    build
        .files(&lvgl_sources)
        .include(&lvgl_path)
        .include(&lvgl_path.join("src"))
        .include(&lv_conf_name)
        .include(&config_path)
        .define("LV_CONF_INCLUDE_SIMPLE", None)
        .warnings(false)
        .extra_warnings(false)
        .flag_if_supported("-Wno-unused-parameter")
        .flag_if_supported("-Wno-missing-field-initializers")
        .flag_if_supported("-Wno-type-limits");

    // Windows-specific
    if target_os == "windows" {
        build.flag_if_supported("/W0");
    }

    build.compile("lvgl");

    println!("cargo:rustc-link-lib=static=lvgl");

    // Generate bindings
    // Bindgen picks up cross-compilation args from BINDGEN_EXTRA_CLANG_ARGS
    // and BINDGEN_EXTRA_CLANG_ARGS_<target> env vars.
    let mut bindgen_builder = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_arg(format!("-I{}", lvgl_path.display()))
        .clang_arg(format!("-I{}", lvgl_path.join("src").display()))
        .clang_arg(format!("-I{}", lv_conf_name.display()))
        .clang_arg(format!("-I{}", config_path.display()))
        .clang_arg("-DLV_CONF_INCLUDE_SIMPLE")
        .allowlist_type("lv_.*")
        .allowlist_function("lv_.*")
        .allowlist_var("LV_.*")
        .layout_tests(false)
        .generate_comments(true)
        .prepend_enum_name(false)
        .derive_default(true)
        .size_t_is_usize(true);

    // Use core types unless building with simulator (std) feature
    if !is_simulator {
        bindgen_builder = bindgen_builder.use_core();

        // When cross-compiling, help bindgen find system headers by querying
        // the cross-compiler for its sysroot
        let host = env::var("HOST").unwrap_or_default();
        let target = env::var("TARGET").unwrap_or_default();
        if host != target {
            if let Some(sysroot) = find_cross_sysroot(&target) {
                bindgen_builder = bindgen_builder.clang_arg(format!("--sysroot={}", sysroot));
            }
        }
    }

    let bindings = bindgen_builder
        .generate()
        .expect("Failed to generate bindings");

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Failed to write bindings");
}

/// Find the sysroot for a cross-compiler by querying the CC compiler.
/// Uses the CC_<target> env var or falls back to common toolchain prefixes.
fn find_cross_sysroot(target: &str) -> Option<String> {
    // Build the CC env var name: CC_xtensa_esp32s3_espidf
    let target_underscored = target.replace('-', "_");
    let cc = env::var(format!("CC_{}", target_underscored))
        .or_else(|_| env::var("TARGET_CC"))
        .or_else(|_| env::var("CC"))
        .ok()?;

    // Ask the compiler for its sysroot
    if let Ok(output) = Command::new(&cc).arg("-print-sysroot").output() {
        if output.status.success() {
            let sysroot = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if !sysroot.is_empty() && PathBuf::from(&sysroot).exists() {
                return Some(sysroot);
            }
        }
    }

    None
}

/// Resolve the LVGL C source path.
///
/// Priority:
/// 1. LVGL_PATH env var
/// 2. `lvgl/` directory next to the workspace root (for local development)
/// 3. Auto-download into OUT_DIR
fn resolve_lvgl_path(manifest_dir: &PathBuf, out_path: &PathBuf) -> PathBuf {
    // 1. Explicit LVGL_PATH env var
    if let Ok(path) = env::var("LVGL_PATH") {
        let p = PathBuf::from(&path);
        if p.join("src").exists() {
            return p;
        }
    }

    // 2. Check relative to this crate's parent (workspace root)
    if let Some(workspace_root) = manifest_dir.parent() {
        let local = workspace_root.join("lvgl");
        if local.join("src").exists() {
            return local;
        }
    }

    // 3. Auto-download into OUT_DIR
    let lvgl_dir = out_path.join("lvgl");
    if lvgl_dir.join("src").exists() {
        return lvgl_dir;
    }

    println!(
        "cargo:warning=LVGL source not found. Downloading {} from GitHub...",
        LVGL_VERSION
    );

    let status = Command::new("git")
        .args([
            "clone",
            "--depth",
            "1",
            "-b",
            LVGL_VERSION,
            LVGL_REPO,
            &lvgl_dir.to_string_lossy(),
        ])
        .status()
        .expect("Failed to run git. Is git installed?");

    if !status.success() {
        panic!(
            "Failed to clone LVGL {}. Ensure git is installed and you have internet access.\n\
             Alternatively, set LVGL_PATH to point to an existing LVGL source directory.",
            LVGL_VERSION
        );
    }

    lvgl_dir
}
