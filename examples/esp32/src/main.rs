//! LVGL + ST7789 Example for ESP32
//!
//! Demonstrates LVGL widgets on a real display with safe Rust wrappers.
//! Configured for LILYGO T-Display-S3 (170x320), easily adaptable.
//!
//! Pin configuration for T-Display-S3:
//! - MOSI: GPIO11
//! - SCLK: GPIO12
//! - CS:   GPIO10
//! - DC:   GPIO13
//! - RST:  GPIO9
//! - BL:   GPIO14

use esp_idf_hal::delay::FreeRtos;
use esp_idf_hal::gpio::{Gpio10, Gpio11, Gpio13, Gpio9, PinDriver};
use esp_idf_hal::peripherals::Peripherals;
use esp_idf_hal::spi::{
    config::Config as SpiConfig, config::DriverConfig, SpiDeviceDriver, SpiDriver,
};
use esp_idf_hal::units::FromValueType;
use esp_idf_svc::log::EspLogger;
use log::info;

mod drivers;

use drivers::st7789::{St7789, St7789Config};
use lvgl::display::{Display, RenderMode};
use lvgl::input::{InputDevice, InputState, InputType, TouchPoint};
use lvgl::widgets::*;
use lvgl::{Color, Event, LvglObj, Obj, Style};

// =============================================================================
// Configuration - Adjust for your board!
// =============================================================================

const DISPLAY_WIDTH: u32 = 170;
const DISPLAY_HEIGHT: u32 = 320;
const BUFFER_LINES: u32 = 32;

static mut DISPLAY_BUF1: [u8; (DISPLAY_WIDTH * BUFFER_LINES * 2) as usize] =
    [0u8; (DISPLAY_WIDTH * BUFFER_LINES * 2) as usize];

// =============================================================================
// Global State (needed for C callbacks)
// =============================================================================

static mut DISPLAY_DRIVER: Option<St7789<'static, Gpio13, Gpio9>> = None;

static mut TOUCH_POINT: TouchPoint = TouchPoint {
    x: 0,
    y: 0,
    pressed: false,
};

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

    if let Some(ref mut driver) = DISPLAY_DRIVER {
        let data = core::slice::from_raw_parts(px_map, len);
        let _ = driver.flush(x1, y1, x2, y2, data);
    }

    lvgl::sys::lv_display_flush_ready(disp);
}

unsafe extern "C" fn touch_read_cb(
    _indev: *mut lvgl::sys::lv_indev_t,
    data: *mut lvgl::sys::lv_indev_data_t,
) {
    (*data).point.x = TOUCH_POINT.x;
    (*data).point.y = TOUCH_POINT.y;
    (*data).state = if TOUCH_POINT.pressed {
        InputState::Pressed as u32
    } else {
        InputState::Released as u32
    };
}

// =============================================================================
// Layout helpers
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

fn set_pad_column(obj: &impl LvglObj, pad: i32) {
    unsafe { lvgl::sys::lv_obj_set_style_pad_column(obj.raw(), pad, 0) }
}

/// Create a transparent container row
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
    esp_idf_svc::sys::link_patches();
    EspLogger::initialize_default();

    info!("LVGL + ST7789 Demo Starting...");

    let peripherals = Peripherals::take()?;

    // SPI pins for T-Display-S3
    let spi = peripherals.spi2;
    let sclk = peripherals.pins.gpio12;
    let mosi = peripherals.pins.gpio11;
    let cs = peripherals.pins.gpio10;

    let dc = PinDriver::output(peripherals.pins.gpio13)?;
    let rst = PinDriver::output(peripherals.pins.gpio9)?;
    let mut bl = PinDriver::output(peripherals.pins.gpio14)?;

    let spi_driver = SpiDriver::new(spi, sclk, mosi, None::<Gpio11>, &DriverConfig::default())?;

    let spi_config = SpiConfig::default()
        .baudrate(60.MHz().into())
        .write_only(true);

    let spi_device = SpiDeviceDriver::new(&spi_driver, Some(cs), &spi_config)?;

    let config = St7789Config::t_display_s3();
    let mut display_driver = St7789::new(spi_device, dc, Some(rst), config);

    info!("Initializing ST7789 display...");
    display_driver.init()?;
    bl.set_high()?;

    unsafe {
        DISPLAY_DRIVER = Some(core::mem::transmute(display_driver));
    }

    // Initialize LVGL
    lvgl::init()?;

    let display = Display::create(DISPLAY_WIDTH, DISPLAY_HEIGHT)?;
    unsafe {
        display.set_buffers(&mut DISPLAY_BUF1, None, RenderMode::Partial);
    }
    display.set_flush_cb(flush_cb);

    let indev = InputDevice::create()?;
    indev.set_type(InputType::Pointer);
    indev.set_read_cb(touch_read_cb);

    info!("Creating UI...");
    create_demo_ui()?;

    info!("UI created, entering main loop...");

    loop {
        let delay_ms = lvgl::task_handler();
        FreeRtos::delay_ms(core::cmp::min(delay_ms, 5));
    }
}

// =============================================================================
// Demo UI — Scrollable vertical layout for tall narrow screen
// =============================================================================

fn create_demo_ui() -> Result<(), lvgl::LvglError> {
    let screen = lvgl::screen_active().expect("No active screen");

    // Dark background with vertical flex
    let bg_style = Box::leak(Box::new(Style::new()));
    bg_style.set_bg_color(Color::hex(0x1a1a2e));
    bg_style.set_bg_opa(255);
    bg_style.set_pad_all(8);
    bg_style.set_pad_row(8);
    screen.add_style(bg_style, 0);

    set_flex_flow(&screen, lvgl::sys::LV_FLEX_FLOW_COLUMN);
    set_flex_align(
        &screen,
        lvgl::sys::LV_FLEX_ALIGN_START,
        lvgl::sys::LV_FLEX_ALIGN_CENTER,
        lvgl::sys::LV_FLEX_ALIGN_CENTER,
    );

    // Title
    let title = Label::create(&screen)?;
    title.set_text(c"LVGL + ESP32");
    title.set_text_color(Color::hex(0x00d4ff));

    // LED + Button row
    let btn_row = create_row(&screen)?;
    set_pad_column(&btn_row, 10);

    let led = Led::create(&btn_row)?;
    led.set_size(18, 18);
    led.set_color(Color::hex(0x00ff88));
    led.off();

    let btn = Button::create(&btn_row)?;
    btn.set_size(110, 34);
    let btn_style = Box::leak(Box::new(Style::new()));
    btn_style.set_bg_color(Color::hex(0x0077b6));
    btn_style.set_radius(8);
    btn.add_style(btn_style, 0);
    let btn_label = Label::create(&btn)?;
    btn_label.set_text(c"Toggle LED");
    btn_label.center();

    let led_ptr = led.raw();
    btn.add_event_cb(Event::Clicked, move || unsafe {
        lvgl::sys::lv_led_toggle(led_ptr);
    });

    // Slider with live value
    let slider_row = create_row(&screen)?;
    set_pad_column(&slider_row, 8);

    let slider_val = Label::create(&slider_row)?;
    slider_val.set_text(c"50");
    slider_val.set_text_color(Color::hex(0x00d4ff));
    slider_val.set_width(28);

    let slider = Slider::create(&slider_row)?;
    slider.set_size(110, 10);
    slider.set_range(0, 100);
    slider.set_value(50, false);

    let slider_ptr = slider.raw();
    let slider_val_ptr = slider_val.raw();
    slider.add_event_cb(Event::ValueChanged, move || unsafe {
        let val = lvgl::sys::lv_slider_get_value(slider_ptr);
        let mut buf = [0u8; 8];
        let text = format_int(&mut buf, val);
        lvgl::sys::lv_label_set_text(slider_val_ptr, text.as_ptr() as *const _);
    });

    // Switch + Checkbox
    let toggle_row = create_row(&screen)?;
    set_pad_column(&toggle_row, 12);

    let sw_label = Label::create(&toggle_row)?;
    sw_label.set_text(c"WiFi");
    sw_label.set_text_color(Color::hex(0xaaaaaa));

    let sw = Switch::create(&toggle_row)?;
    sw.set_checked(true);

    let cb = Checkbox::create(&toggle_row)?;
    cb.set_text(c"Auto");

    // Dropdown
    let dd = Dropdown::create(&screen)?;
    dd.set_width(150);
    dd.set_options(c"115200\n57600\n38400\n19200\n9600");

    // Progress bars
    let bar_row1 = create_row(&screen)?;
    set_pad_column(&bar_row1, 6);
    let bl1 = Label::create(&bar_row1)?;
    bl1.set_text(c"CPU");
    bl1.set_text_color(Color::hex(0xaaaaaa));
    bl1.set_width(30);
    let b1 = Bar::create(&bar_row1)?;
    b1.set_size(110, 8);
    b1.set_range(0, 100);
    b1.set_value(72, true);

    let bar_row2 = create_row(&screen)?;
    set_pad_column(&bar_row2, 6);
    let bl2 = Label::create(&bar_row2)?;
    bl2.set_text(c"RAM");
    bl2.set_text_color(Color::hex(0xaaaaaa));
    bl2.set_width(30);
    let b2 = Bar::create(&bar_row2)?;
    b2.set_size(110, 8);
    b2.set_range(0, 100);
    b2.set_value(45, true);

    // Arc + Spinner
    let bottom_row = create_row(&screen)?;
    set_pad_column(&bottom_row, 16);

    let arc = Arc::create(&bottom_row)?;
    arc.set_size(65, 65);
    arc.set_range(0, 100);
    arc.set_value(65);
    arc.set_bg_angles(135, 45);

    let arc_label = Label::create(&arc)?;
    arc_label.set_text(c"65%");
    arc_label.center();
    arc_label.set_text_color(Color::hex(0x00ff88));

    let arc_ptr = arc.raw();
    let arc_label_ptr = arc_label.raw();
    arc.add_event_cb(Event::ValueChanged, move || unsafe {
        let val = lvgl::sys::lv_arc_get_value(arc_ptr);
        let mut buf = [0u8; 8];
        let text = format_int_percent(&mut buf, val);
        lvgl::sys::lv_label_set_text(arc_label_ptr, text.as_ptr() as *const _);
    });

    let spinner = Spinner::create(&bottom_row)?;
    spinner.set_size(40, 40);
    spinner.set_anim_params(1000, 270);

    Ok(())
}

// =============================================================================
// Helpers
// =============================================================================

/// Format an integer as "N%" with null terminator.
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
    for i in 0..len {
        buf[i] = tmp[len - 1 - i];
    }
    buf[len] = b'%';
    buf[len + 1] = 0;
    &buf[..len + 2]
}

/// Format an integer with null terminator.
fn format_int(buf: &mut [u8; 8], val: i32) -> &[u8] {
    let negative = val < 0;
    let mut n = if negative { (-val) as u32 } else { val as u32 };
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
    let mut pos = 0;
    if negative {
        buf[0] = b'-';
        pos = 1;
    }
    for i in 0..len {
        buf[pos + i] = tmp[len - 1 - i];
    }
    buf[pos + len] = 0;
    &buf[..pos + len + 1]
}
