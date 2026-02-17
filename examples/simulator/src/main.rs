//! LVGL Desktop Simulator
//!
//! Demonstrates a variety of LVGL widgets wrapped in safe Rust.
//! Uses SDL2 for window rendering and mouse input.
//!
//! Build and run:
//!   cargo run

mod simulator_display;

use std::thread;
use std::time::{Duration, Instant};

use lvgl::display::{Display, RenderMode};
use lvgl::input::{InputDevice, InputType};
use lvgl::widgets::*;
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
// Demo UI — Tabview with 3 tabs
// =============================================================================

fn create_demo_ui() -> Result<(), lvgl::LvglError> {
    let screen = lvgl::screen_active().expect("No active screen");

    // Screen padding (keep default light theme)
    let bg_style = Box::leak(Box::new(Style::new()));
    screen.add_style(bg_style, 0);

    // Tabview — 3 pages
    let tabview = Tabview::create(&screen)?;
    tabview.set_size(320, 240);
    tabview.set_tab_bar_size(28);

    // Remove dark background from tab bar so it uses default theme
    let tab_bar = tabview.get_tab_bar();
    let tab_bar_style = Box::leak(Box::new(Style::new()));
    tab_bar_style.set_pad_all(0);
    tab_bar.add_style(tab_bar_style, 0);

    let tab1 = tabview.add_tab(c"Controls");
    let tab2 = tabview.add_tab(c"Data");
    let tab3 = tabview.add_tab(c"Inputs");

    // Tab content padding
    let tab_style = Box::leak(Box::new(Style::new()));
    tab_style.set_pad_all(8);
    tab_style.set_pad_row(6);
    tab1.add_style(tab_style, 0);
    tab2.add_style(tab_style, 0);
    tab3.add_style(tab_style, 0);

    create_controls_tab(&tab1)?;
    create_data_tab(&tab2)?;
    create_inputs_tab(&tab3)?;

    Ok(())
}

// =============================================================================
// Tab 1: Controls — Button, Slider, Switch, Checkbox, LED
// =============================================================================

fn create_controls_tab(tab: &Obj) -> Result<(), lvgl::LvglError> {
    set_flex_flow(tab, lvgl::sys::LV_FLEX_FLOW_COLUMN);
    set_flex_align(
        tab,
        lvgl::sys::LV_FLEX_ALIGN_START,
        lvgl::sys::LV_FLEX_ALIGN_CENTER,
        lvgl::sys::LV_FLEX_ALIGN_CENTER,
    );

    // Button with LED indicator
    let btn_row = create_row(tab)?;
    set_pad_column(&btn_row, 12);

    let led = Led::create(&btn_row)?;
    led.set_size(20, 20);
    led.set_color(Color::hex(0x00ff88));
    led.off();

    let btn = Button::create(&btn_row)?;
    btn.set_size(120, 36);
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
    let slider_row = create_row(tab)?;
    set_pad_column(&slider_row, 8);

    let slider_val = Label::create(&slider_row)?;
    slider_val.set_text(c"50");
    slider_val.set_text_color(Color::hex(0x0077b6));
    slider_val.set_width(28);

    let slider = Slider::create(&slider_row)?;
    slider.set_size(200, 10);
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

    // Switch + Checkbox row
    let toggle_row = create_row(tab)?;
    set_pad_column(&toggle_row, 16);

    let sw_label = Label::create(&toggle_row)?;
    sw_label.set_text(c"WiFi");
    sw_label.set_text_color(Color::hex(0x555555));

    let sw = Switch::create(&toggle_row)?;
    sw.set_checked(true);

    let cb = Checkbox::create(&toggle_row)?;
    cb.set_text(c"Dark mode");

    // Arc gauge with percentage
    let arc_row = create_row(tab)?;
    set_pad_column(&arc_row, 20);

    let arc = Arc::create(&arc_row)?;
    arc.set_size(80, 80);
    arc.set_range(0, 100);
    arc.set_value(65);
    arc.set_bg_angles(135, 45);

    let arc_label = Label::create(&arc)?;
    arc_label.set_text(c"65%");
    arc_label.center();
    arc_label.set_text_color(Color::hex(0x2e7d32));

    let arc_ptr = arc.raw();
    let arc_label_ptr = arc_label.raw();
    arc.add_event_cb(Event::ValueChanged, move || unsafe {
        let val = lvgl::sys::lv_arc_get_value(arc_ptr);
        let mut buf = [0u8; 8];
        let text = format_int_percent(&mut buf, val);
        lvgl::sys::lv_label_set_text(arc_label_ptr, text.as_ptr() as *const _);
    });

    let spinner = Spinner::create(&arc_row)?;
    spinner.set_size(50, 50);
    spinner.set_anim_params(1000, 270);

    Ok(())
}

// =============================================================================
// Tab 2: Data — Chart, Bar, Table
// =============================================================================

fn create_data_tab(tab: &Obj) -> Result<(), lvgl::LvglError> {
    set_flex_flow(tab, lvgl::sys::LV_FLEX_FLOW_COLUMN);
    set_flex_align(
        tab,
        lvgl::sys::LV_FLEX_ALIGN_START,
        lvgl::sys::LV_FLEX_ALIGN_CENTER,
        lvgl::sys::LV_FLEX_ALIGN_CENTER,
    );

    // Line chart
    let chart = Chart::create(tab)?;
    chart.set_size(290, 90);
    chart.set_type(ChartType::Line);
    chart.set_point_count(12);
    chart.set_range(ChartAxis::PrimaryY, 0, 100);
    chart.set_div_line_count(3, 5);

    let chart_style = Box::leak(Box::new(Style::new()));
    chart_style.set_bg_color(Color::hex(0xf0f0f0));
    chart_style.set_radius(6);
    chart_style.set_border_width(0);
    chart.add_style(chart_style, 0);

    let series1 = chart.add_series(Color::hex(0x0077b6), ChartAxis::PrimaryY);
    let series2 = chart.add_series(Color::hex(0xd32f2f), ChartAxis::PrimaryY);

    // Populate with sample data
    let data1 = [20, 35, 50, 45, 70, 60, 80, 75, 90, 85, 65, 55];
    let data2 = [50, 40, 30, 55, 45, 35, 60, 50, 40, 70, 55, 45];
    for i in 0..12 {
        chart.set_next_value(&series1, data1[i]);
        chart.set_next_value(&series2, data2[i]);
    }

    // Progress bars with labels
    let bar_row1 = create_row(tab)?;
    set_pad_column(&bar_row1, 8);
    let lbl1 = Label::create(&bar_row1)?;
    lbl1.set_text(c"CPU");
    lbl1.set_text_color(Color::hex(0x555555));
    lbl1.set_width(36);
    let bar1 = Bar::create(&bar_row1)?;
    bar1.set_size(220, 10);
    bar1.set_range(0, 100);
    bar1.set_value(72, true);

    let bar_row2 = create_row(tab)?;
    set_pad_column(&bar_row2, 8);
    let lbl2 = Label::create(&bar_row2)?;
    lbl2.set_text(c"RAM");
    lbl2.set_text_color(Color::hex(0x555555));
    lbl2.set_width(36);
    let bar2 = Bar::create(&bar_row2)?;
    bar2.set_size(220, 10);
    bar2.set_range(0, 100);
    bar2.set_value(45, true);

    let bar_row3 = create_row(tab)?;
    set_pad_column(&bar_row3, 8);
    let lbl3 = Label::create(&bar_row3)?;
    lbl3.set_text(c"Disk");
    lbl3.set_text_color(Color::hex(0x555555));
    lbl3.set_width(36);
    let bar3 = Bar::create(&bar_row3)?;
    bar3.set_size(220, 10);
    bar3.set_range(0, 100);
    bar3.set_value(88, true);

    Ok(())
}

// =============================================================================
// Tab 3: Inputs — Dropdown, Roller, Textarea
// =============================================================================

fn create_inputs_tab(tab: &Obj) -> Result<(), lvgl::LvglError> {
    set_flex_flow(tab, lvgl::sys::LV_FLEX_FLOW_COLUMN);
    set_flex_align(
        tab,
        lvgl::sys::LV_FLEX_ALIGN_START,
        lvgl::sys::LV_FLEX_ALIGN_CENTER,
        lvgl::sys::LV_FLEX_ALIGN_CENTER,
    );

    // Dropdown
    let dd_row = create_row(tab)?;
    set_pad_column(&dd_row, 8);

    let dd_label = Label::create(&dd_row)?;
    dd_label.set_text(c"Theme");
    dd_label.set_text_color(Color::hex(0x555555));
    dd_label.set_width(46);

    let dd = Dropdown::create(&dd_row)?;
    dd.set_width(180);
    dd.set_options(c"Dark\nLight\nBlue\nGreen\nOcean");

    // Roller
    let roller_row = create_row(tab)?;
    set_pad_column(&roller_row, 8);

    let roller_label = Label::create(&roller_row)?;
    roller_label.set_text(c"Baud");
    roller_label.set_text_color(Color::hex(0x555555));
    roller_label.set_width(36);

    let roller = Roller::create(&roller_row)?;
    roller.set_options(c"9600\n19200\n38400\n57600\n115200", RollerMode::Normal);
    roller.set_visible_row_count(3);
    roller.set_selected(4, false);

    // Textarea
    let ta = Textarea::create(tab)?;
    ta.set_size(270, 50);
    ta.set_placeholder_text(c"Type something...");
    ta.set_text(c"LVGL + Rust");

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

/// Format an integer with null terminator (no % suffix).
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
