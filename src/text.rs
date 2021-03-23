use crate::types::Vec2i;
use crate::Rect;
use crate::Texture;
use image::{Rgb, RgbaImage};
use std::path::Path;
use std::rc::Rc;

const CHAR_SIZE: usize = 8;
const ROWS: usize = 14;
const COLUMNS: usize = 16;

pub struct Font {
    image: Rc<Texture>,
}

impl Font {
    pub fn char_to_pos(&self, c: char) -> Rect {
        let mut x = ((c as u32) - 31) as usize % COLUMNS * CHAR_SIZE - CHAR_SIZE;
        let y = ((c as u32) - 31) as usize / ROWS * CHAR_SIZE;

        if x < 0 {
            x += 128;
        };

        Rect {
            x: x as i32,
            y: y as i32,
            w: CHAR_SIZE as u16,
            h: CHAR_SIZE as u16,
        }
    }
}

pub trait DrawTextExt {
    fn draw_text(&mut self, f: Font, w: &str, pos: Vec2i);
}

use crate::screen::Screen;
impl<'fb> DrawTextExt for Screen<'fb> {
    fn draw_text(&mut self, f: Font, w: &str, mut pos: Vec2i) {
        for c in w.chars() {
            let frame = f.char_to_pos(c);
            self.bitblt(&f.image, frame, pos);
            pos.0 += CHAR_SIZE as i32;
        }
    }
}

// cargo run --example simple --release
// pub fn main() {
//     // Loading and rasterization
//     let font = include_bytes!("../../content/Andale Mono.ttf") as &[u8];
//     let settings = fontdue::FontSettings {
//         scale: SIZE,
//         ..fontdue::FontSettings::default()
//     };
//     let font = fontdue::Font::from_bytes(font, settings).unwrap();
//     let (metrics, bitmap) = font.rasterize_subpixel(CHARACTER, SIZE);

//     // Output
//     let mut o = File::create("rgb.ppm").unwrap();
//     let _ = o.write(format!("P6\n{} {}\n255\n", metrics.width, metrics.height).as_bytes());
//     let _ = o.write(&bitmap);
// }
