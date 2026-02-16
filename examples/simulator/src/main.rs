//! LVGL Desktop Simulator
//!
//! Run your LVGL UI on desktop for rapid development and testing.
//! Uses SDL2 for window rendering and mouse input.
//!
//! Build and run:
//!   cargo run
//!
//! Or with release optimizations:
//!   cargo run --release

mod simulator_display;

use std::thread;
use std::time::{Duration, Instant};

use lvgl::display::{Display, RenderMode};
use lvgl::input::{InputDevice, InputType};
use lvgl::widgets::{Arc, Bar, Button, Label, Slider, Spinner, Switch};
use lvgl::{Color, Event, LvglObj, Obj, Style};

use simulator_display::SimulatorDisplay;

// =============================================================================
// Configuration
// =============================================================================

const DISPLAY_WIDTH: u32 = 320;
const DISPLAY_HEIGHT: u32 = 240;
const WINDOW_SCALE: u32 = 2;
const BUFFER_LINES: u32 = 24;

#[repr(C, align(4))]
struct AlignedBuf([u8; (DISPLAY_WIDTH * BUFFER_LINES * 2) as usize]);
static mut DISPLAY_BUF: AlignedBuf = AlignedBuf([0u8; (DISPLAY_WIDTH * BUFFER_LINES * 2) as usize]);

static mut SIMULATOR: Option<SimulatorDisplay> = None;

static mut MOUSE_X: i32 = 0;
static mut MOUSE_Y: i32 = 0;
static mut MOUSE_PRESSED: bool = false;

// =============================================================================
// LVGL Callbacks
// =============================================================================

unsafe extern "C" fn flush_cb(
    disp: *mut lvgl::sys::lv_display_t,
    area: *const lvgl::sys::lv_area_t,
    px_map: *mut u8,
) {
    let area = &*area;
    let x1 = area.x1;
    let y1 = area.y1;
    let x2 = area.x2;
    let y2 = area.y2;

    let width = (x2 - x1 + 1) as usize;
    let height = (y2 - y1 + 1) as usize;
    let len = width * height * 2;

    if let Some(ref mut sim) = SIMULATOR {
        let data = std::slice::from_raw_parts(px_map, len);
        sim.flush(x1, y1, x2, y2, data);
    }

    lvgl::sys::lv_display_flush_ready(disp);
}

unsafe extern "C" fn touch_read_cb(
    _indev: *mut lvgl::sys::lv_indev_t,
    data: *mut lvgl::sys::lv_indev_data_t,
) {
    (*data).point.x = MOUSE_X;
    (*data).point.y = MOUSE_Y;
    (*data).state = if MOUSE_PRESSED {
        lvgl::sys::LV_INDEV_STATE_PRESSED
    } else {
        lvgl::sys::LV_INDEV_STATE_RELEASED
    };
}

// =============================================================================
// Helpers for raw LVGL calls not yet wrapped
// =============================================================================

fn set_flex_flow(obj: &impl LvglObj, flow: u32) {
    unsafe { lvgl::sys::lv_obj_set_flex_flow(obj.raw(), flow) }
}

fn set_flex_align(obj: &impl LvglObj, main: u32, cross: u32, track: u32) {
    unsafe { lvgl::sys::lv_obj_set_flex_align(obj.raw(), main, cross, track) }
}

fn remove_style_all(obj: &impl LvglObj) {
    unsafe { lvgl::sys::lv_obj_remove_style_all(obj.raw()) }
}

fn remove_flag(obj: &impl LvglObj, flag: u32) {
    unsafe { lvgl::sys::lv_obj_remove_flag(obj.raw(), flag) }
}

fn pct(v: i32) -> i32 {
    unsafe { lvgl::sys::lv_pct(v) }
}

/// Create a transparent container row (no background, no border, no scroll)
fn create_row(parent: &impl LvglObj) -> Result<Obj, lvgl::LvglError> {
    let row = Obj::create(parent)?;
    remove_style_all(&row);
    remove_flag(&row, lvgl::sys::LV_OBJ_FLAG_SCROLLABLE);
    row.set_width(pct(100));
    set_flex_flow(&row, lvgl::sys::LV_FLEX_FLOW_ROW);
    set_flex_align(
        &row,
        lvgl::sys::LV_FLEX_ALIGN_CENTER,
        lvgl::sys::LV_FLEX_ALIGN_CENTER,
        lvgl::sys::LV_FLEX_ALIGN_CENTER,
    );
    Ok(row)
}

// =============================================================================
// Main
// =============================================================================

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!(
        "LVGL Simulator - {}x{} ({}x scale)",
        DISPLAY_WIDTH, DISPLAY_HEIGHT, WINDOW_SCALE
    );
    println!("Close window or Ctrl+C to exit");

    let simulator = SimulatorDisplay::new(
        "LVGL Simulator",
        DISPLAY_WIDTH,
        DISPLAY_HEIGHT,
        WINDOW_SCALE,
    )?;

    unsafe {
        SIMULATOR = Some(std::mem::transmute(simulator));
    }

    lvgl::init()?;

    let display = Display::create(DISPLAY_WIDTH, DISPLAY_HEIGHT)?;
    unsafe {
        display.set_buffers(&mut DISPLAY_BUF.0, None, RenderMode::Partial);
    }
    display.set_flush_cb(flush_cb);

    let indev = InputDevice::create()?;
    indev.set_type(InputType::Pointer);
    indev.set_read_cb(touch_read_cb);

    create_demo_ui()?;

    let start_time = Instant::now();
    let mut last_tick = 0u32;

    loop {
        let elapsed = start_time.elapsed().as_millis() as u32;
        if elapsed > last_tick {
            lvgl::tick_inc(elapsed - last_tick);
            last_tick = elapsed;
        }

        let sim = unsafe { SIMULATOR.as_mut().unwrap() };
        sim.poll_events();

        if sim.quit_requested() {
            break;
        }

        let (mx, my, pressed) = sim.mouse_state();
        unsafe {
            MOUSE_X = mx;
            MOUSE_Y = my;
            MOUSE_PRESSED = pressed;
        }

        let delay_ms = lvgl::task_handler();
        sim.render();
        thread::sleep(Duration::from_millis(delay_ms.min(16) as u64));
    }

    Ok(())
}

// =============================================================================
// Demo UI
// =============================================================================

fn create_demo_ui() -> Result<(), lvgl::LvglError> {
    let screen = lvgl::screen_active().expect("No active screen");

    // Dark background
    let mut bg_style = Style::new();
    bg_style.set_bg_color(Color::hex(0x1a1a2e));
    bg_style.set_bg_opa(255);
    bg_style.set_pad_all(12);
    bg_style.set_pad_row(8);
    screen.add_style(&bg_style, 0);

    // Vertical flex layout on screen
    set_flex_flow(&screen, lvgl::sys::LV_FLEX_FLOW_COLUMN);
    set_flex_align(
        &screen,
        lvgl::sys::LV_FLEX_ALIGN_START,
        lvgl::sys::LV_FLEX_ALIGN_CENTER,
        lvgl::sys::LV_FLEX_ALIGN_CENTER,
    );

    // ---- Title ----
    let title = Label::create(&screen)?;
    title.set_text(c"LVGL Rust Simulator");
    title.set_text_color(Color::hex(0x00d4ff));

    // ---- Button ----
    let btn = Button::create(&screen)?;
    btn.set_size(200, 40);

    let mut btn_style = Style::new();
    btn_style.set_bg_color(Color::hex(0x0077b6));
    btn_style.set_radius(8);
    btn_style.set_shadow_width(4);
    btn_style.set_shadow_color(Color::hex(0x000000));
    btn_style.set_shadow_opa(100);
    btn.add_style(&btn_style, 0);

    let btn_label = Label::create(&btn)?;
    btn_label.set_text(c"Click Me!");
    btn_label.center();

    static mut CLICK_COUNT: u32 = 0;
    btn.add_event_cb(Event::Clicked, || unsafe {
        CLICK_COUNT += 1;
        println!("Button clicked! Count: {}", CLICK_COUNT);
    });

    // ---- Slider row: label + slider ----
    let slider_row = create_row(&screen)?;
    slider_row.set_style_pad_all(4, 0);

    let slider_lbl = Label::create(&slider_row)?;
    slider_lbl.set_text(c"Slider");
    slider_lbl.set_text_color(Color::hex(0xaaaaaa));
    slider_lbl.set_width(60);

    let slider = Slider::create(&slider_row)?;
    slider.set_size(200, 12);
    slider.set_range(0, 100);
    slider.set_value(50, false);

    // ---- Progress bar row: label + bar ----
    let bar_row = create_row(&screen)?;
    bar_row.set_style_pad_all(4, 0);

    let bar_lbl = Label::create(&bar_row)?;
    bar_lbl.set_text(c"Progress");
    bar_lbl.set_text_color(Color::hex(0xaaaaaa));
    bar_lbl.set_width(60);

    let bar = Bar::create(&bar_row)?;
    bar.set_size(200, 12);
    bar.set_range(0, 100);
    bar.set_value(75, true);

    // ---- Switch row: label + switch ----
    let switch_row = create_row(&screen)?;
    switch_row.set_style_pad_all(4, 0);

    let switch_lbl = Label::create(&switch_row)?;
    switch_lbl.set_text(c"Enable");
    switch_lbl.set_text_color(Color::hex(0xaaaaaa));
    switch_lbl.set_width(60);

    let switch = Switch::create(&switch_row)?;
    switch.set_checked(true);

    switch.add_event_cb(Event::ValueChanged, || {
        println!("Switch toggled");
    });

    // ---- Bottom row: Arc + Spinner ----
    let bottom_row = create_row(&screen)?;
    bottom_row.set_style_pad_all(4, 0);

    // Arc gauge
    let arc = Arc::create(&bottom_row)?;
    arc.set_size(80, 80);
    arc.set_range(0, 100);
    arc.set_value(65);
    arc.set_bg_angles(135, 45);

    let arc_label = Label::create(&arc)?;
    arc_label.set_text(c"65%");
    arc_label.center();
    arc_label.set_text_color(Color::hex(0x00ff88));

    // Update label when arc value changes
    let arc_ptr = arc.raw();
    let arc_label_ptr = arc_label.raw();
    arc.add_event_cb(Event::ValueChanged, move || unsafe {
        let val = lvgl::sys::lv_arc_get_value(arc_ptr);
        let mut buf = [0u8; 8];
        let text = format_int_percent(&mut buf, val);
        lvgl::sys::lv_label_set_text(arc_label_ptr, text.as_ptr() as *const _);
    });

    // Spacer between arc and spinner
    let spacer = Obj::create(&bottom_row)?;
    remove_style_all(&spacer);
    spacer.set_size(40, 1);

    // Spinner
    let spinner = Spinner::create(&bottom_row)?;
    spinner.set_size(50, 50);
    spinner.set_anim_params(1000, 270);

    Ok(())
}

/// Format an integer as "N%" into a buffer, returning a null-terminated slice.
fn format_int_percent(buf: &mut [u8; 8], val: i32) -> &[u8] {
    let mut n = if val < 0 { 0 } else { val as u32 };
    let mut tmp = [0u8; 6];
    let mut len = 0;
    if n == 0 {
        tmp[0] = b'0';
        len = 1;
    } else {
        while n > 0 {
            tmp[len] = b'0' + (n % 10) as u8;
            n /= 10;
            len += 1;
        }
    }
    // Reverse digits into buf
    for i in 0..len {
        buf[i] = tmp[len - 1 - i];
    }
    buf[len] = b'%';
    buf[len + 1] = 0; // null terminator
    &buf[..len + 2]
}
