//! SDL2 Simulator Display Driver
//!
//! Allows running LVGL on desktop for development/testing.
//! Uses SDL2 for window management and rendering.

/// SDL2 window wrapper for LVGL simulation
pub struct SimulatorDisplay {
    width: u32,
    height: u32,
    scale: u32,
    canvas: sdl2::render::Canvas<sdl2::video::Window>,
    event_pump: sdl2::EventPump,
    framebuffer: Vec<u8>,
    mouse_x: i32,
    mouse_y: i32,
    mouse_pressed: bool,
    quit_requested: bool,
}

impl SimulatorDisplay {
    /// Create a new simulator display
    ///
    /// # Arguments
    /// * `title` - Window title
    /// * `width` - Display width in pixels
    /// * `height` - Display height in pixels
    /// * `scale` - Scale factor (2 = 2x window size)
    pub fn new(title: &str, width: u32, height: u32, scale: u32) -> Result<Self, String> {
        let sdl_context = sdl2::init()?;
        let video_subsystem = sdl_context.video()?;

        let window = video_subsystem
            .window(title, width * scale, height * scale)
            .position_centered()
            .build()
            .map_err(|e| e.to_string())?;

        let canvas = window
            .into_canvas()
            .accelerated()
            .present_vsync()
            .build()
            .map_err(|e| e.to_string())?;

        let event_pump = sdl_context.event_pump()?;

        // RGB565 framebuffer (2 bytes per pixel)
        let framebuffer = vec![0u8; (width * height * 2) as usize];

        Ok(Self {
            width,
            height,
            scale,
            canvas,
            event_pump,
            framebuffer,
            mouse_x: 0,
            mouse_y: 0,
            mouse_pressed: false,
            quit_requested: false,
        })
    }

    /// Get display width
    pub fn width(&self) -> u32 {
        self.width
    }

    /// Get display height
    pub fn height(&self) -> u32 {
        self.height
    }

    /// Check if quit was requested (window closed)
    pub fn quit_requested(&self) -> bool {
        self.quit_requested
    }

    /// Get current mouse state
    pub fn mouse_state(&self) -> (i32, i32, bool) {
        (self.mouse_x, self.mouse_y, self.mouse_pressed)
    }

    /// Process SDL events (call this in your main loop)
    pub fn poll_events(&mut self) {
        use sdl2::event::Event;
        use sdl2::mouse::MouseButton;

        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => {
                    self.quit_requested = true;
                }
                Event::MouseMotion { x, y, .. } => {
                    self.mouse_x = x / self.scale as i32;
                    self.mouse_y = y / self.scale as i32;
                }
                Event::MouseButtonDown {
                    mouse_btn: MouseButton::Left,
                    x,
                    y,
                    ..
                } => {
                    self.mouse_x = x / self.scale as i32;
                    self.mouse_y = y / self.scale as i32;
                    self.mouse_pressed = true;
                }
                Event::MouseButtonUp {
                    mouse_btn: MouseButton::Left,
                    ..
                } => {
                    self.mouse_pressed = false;
                }
                _ => {}
            }
        }
    }

    /// Flush a region to the simulated display (for LVGL)
    pub fn flush(&mut self, x1: i32, y1: i32, x2: i32, y2: i32, data: &[u8]) {
        let width = (x2 - x1 + 1) as usize;
        let height = (y2 - y1 + 1) as usize;

        // Copy data to framebuffer
        for row in 0..height {
            let src_offset = row * width * 2;
            let dst_y = y1 as usize + row;
            let dst_offset = (dst_y * self.width as usize + x1 as usize) * 2;

            if dst_offset + width * 2 <= self.framebuffer.len()
                && src_offset + width * 2 <= data.len()
            {
                self.framebuffer[dst_offset..dst_offset + width * 2]
                    .copy_from_slice(&data[src_offset..src_offset + width * 2]);
            }
        }
    }

    /// Render the framebuffer to the window
    pub fn render(&mut self) {
        use sdl2::pixels::PixelFormatEnum;
        use sdl2::rect::Rect;

        let texture_creator = self.canvas.texture_creator();

        // Create texture from framebuffer
        let mut texture = texture_creator
            .create_texture_streaming(PixelFormatEnum::RGB565, self.width, self.height)
            .expect("Failed to create texture");

        // Update texture with framebuffer data
        texture
            .update(None, &self.framebuffer, (self.width * 2) as usize)
            .expect("Failed to update texture");

        // Clear and draw
        self.canvas.clear();
        self.canvas
            .copy(
                &texture,
                None,
                Some(Rect::new(
                    0,
                    0,
                    self.width * self.scale,
                    self.height * self.scale,
                )),
            )
            .expect("Failed to copy texture");
        self.canvas.present();
    }

    /// Fill the entire display with a color (RGB565)
    pub fn clear(&mut self, color: u16) {
        let hi = (color >> 8) as u8;
        let lo = (color & 0xFF) as u8;
        for i in (0..self.framebuffer.len()).step_by(2) {
            self.framebuffer[i] = hi;
            self.framebuffer[i + 1] = lo;
        }
    }
}

/// RGB565 color helpers
pub mod color {
    pub const fn rgb565(r: u8, g: u8, b: u8) -> u16 {
        ((r as u16 & 0xF8) << 8) | ((g as u16 & 0xFC) << 3) | (b as u16 >> 3)
    }

    pub const BLACK: u16 = 0x0000;
    pub const WHITE: u16 = 0xFFFF;
    pub const RED: u16 = rgb565(255, 0, 0);
    pub const GREEN: u16 = rgb565(0, 255, 0);
    pub const BLUE: u16 = rgb565(0, 0, 255);
}
