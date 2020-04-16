mod life;
use crate::life::Loc;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::{SystemTime, UNIX_EPOCH};

const MILLIS_PER_FRAME: u128 = 20;

struct View {
    pub cell_size: i32,
    pub width: u32,
    pub height: u32,
    pub center_row: i32,
    pub center_col: i32,
}
impl View {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            cell_size: 4,
            width,
            height,
            center_row: 0,
            center_col: 0,
        }
    }
    pub fn max_cols(&self) -> i32 {
        (self.width as i32) / self.cell_size
    }
    pub fn max_rows(&self) -> i32 {
        (self.height as i32) / self.cell_size
    }
    pub fn zoom_in(&mut self) {
        self.cell_size = (self.cell_size + 1).min(20);
    }
    pub fn zoom_out(&mut self) {
        self.cell_size = (self.cell_size - 1).max(1);
    }
    pub fn visible(&self, loc: &Loc) -> bool {
        let max_rows = self.max_rows();
        let max_cols = self.max_cols();
        (loc.row as i32 + self.center_row) >= -max_rows / 2
            && (loc.col as i32 + self.center_col) >= -max_cols / 2
            && (loc.row as i32 + self.center_row) < max_rows / 2
            && (loc.col as i32 + self.center_col) < max_cols / 2
    }
    pub fn to_screen(&self, loc: &Loc) -> sdl2::rect::Rect {
        sdl2::rect::Rect::new(
            ((loc.col as i32) + self.max_cols() / 2 + self.center_col) * self.cell_size,
            ((loc.row as i32) + self.max_rows() / 2 + self.center_row) * self.cell_size,
            self.cell_size as u32,
            self.cell_size as u32,
        )
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Usage: rust_of_life configuration");
    } else {
        let sdl_ctx = sdl2::init().unwrap();
        let sdl_video = sdl_ctx.video().unwrap();
        let window = sdl_video
            .window("Rust of Life", 1280, 720)
            .build()
            .map_err(|e| e.to_string())
            .unwrap();
        let mut canvas = window
            .into_canvas()
            .target_texture()
            .present_vsync()
            .build()
            .map_err(|e| e.to_string())
            .unwrap();
        println!("Using SDL with renderer {}", canvas.info().name);
        let mut event_pump = sdl_ctx.event_pump().unwrap();

        let (width, height) = canvas.output_size().unwrap();
        let mut view = View::new(width, height);

        let config_path = format!("worlds/{}.txt", args[1]);
        let original_world = crate::life::World::from_configuration(
            &std::fs::read_to_string(std::path::Path::new(&config_path)).unwrap(),
            '.',
            '*',
        )
        .unwrap();
        let mut world = original_world.clone();
        let mut previous_update = UNIX_EPOCH;
        'running: loop {
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Q),
                        ..
                    } => {
                        break 'running;
                    }
                    Event::KeyDown {
                        keycode: Some(Keycode::R),
                        ..
                    } => {
                        world = original_world.clone();
                    }
                    Event::KeyDown {
                        keycode: Some(Keycode::Minus),
                        ..
                    }
                    | Event::MouseWheel { y: -1, .. } => {
                        view.zoom_out();
                    }
                    Event::KeyDown {
                        keycode: Some(Keycode::Equals),
                        keymod: sdl2::keyboard::Mod::LSHIFTMOD,
                        ..
                    }
                    | Event::MouseWheel { y: 1, .. } => {
                        view.zoom_in();
                    }
                    Event::KeyDown {
                        keycode: Some(Keycode::Up), ..}
                    => {
                        view.center_row += 2;
                    }
                    Event::KeyDown {
                        keycode: Some(Keycode::Down), ..}
                    => {
                        view.center_row -= 2;
                    }
                    Event::KeyDown {
                        keycode: Some(Keycode::Right), ..}
                    => {
                        view.center_col -= 2;
                    }
                    Event::KeyDown {
                        keycode: Some(Keycode::Left), ..}
                    => {
                        view.center_col += 2;
                    }


                    _ => {}
                }
            }
            // Update
            if previous_update
                .elapsed()
                .map(|d| d.as_millis())
                .unwrap_or(0)
                > MILLIS_PER_FRAME
            {
                world.step();
                previous_update = SystemTime::now();
            }

            // Clear
            canvas.set_draw_color(sdl2::pixels::Color::RGB(0x11, 0x33, 0x66));
            canvas.clear();

            // Draw
            canvas.set_draw_color(sdl2::pixels::Color::RGB(0xcc, 0xcc, 0xcc));
            for loc in world.current_buffer().keys() {
                if view.visible(loc) {
                    if world.get(loc) {
                        canvas.fill_rect(view.to_screen(loc)).unwrap();
                    }
                }
            }

            canvas.present();
        }
    }
}
