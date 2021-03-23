use crate::Texture;
use std::fs::File;
use std::io::Write;
use image::{RgbaImage, Rgb};
use fontdue::Font;
use crate::Rect;
use crate::types::Vec2i;
use std::path::Path;

// Scratch pad for glyphs: ⅞ g
// const CHARACTER: char = 'g';
// const SIZE: f32 = 20.0;

pub struct MyFont {
    font: Font,
    size: f32,
}

impl MyFont {
    pub fn new(path: &Path, size: f32) -> Self {
        let font = include_bytes!(path).expect("Couldn't load font") as &[u8];

        let settings = fontdue::FontSettings {
            scale: size,
            ..fontdue::FontSettings::default()
        };

        Self{
            font: fontdue::Font::from_bytes(font, settings).unwrap(),
            size: size,
        }
    }

    pub fn rasterize(&self, string: &str) -> Texture {
        let (metrics, bitmap) = self.font.rasterize_subpixel(string, self.size);

        let mut img = RgbaImage::from_vec(metrics.width, metrics.height, bitmap);

        Texture::new(img.unwrap())
    

        // let mut o = File::create("rgb.ppm").unwrap();
        // let _ = o.write(format!("P6\n{} {}\n255\n", metrics.width, metrics.height).as_bytes());
        // let _ = o.write(&bitmap);
    }
}

pub trait DrawTextExt {
    fn draw_text(&mut self, t: &Texture, pos: Vec2i);
}

use crate::screen::Screen;
impl<'fb> DrawTextExt for Screen<'fb> {
    fn draw_text(&mut self, t: &Texture, pos:Vec2i) {
        // This works because we're only using a public method of Screen here,
        // and the private fields of sprite are visible inside this module
        let frame = Rect{
            x:0,
            y:0,
            h: t.height as u16,
            w: t.width as u16
        }
        // let position = Vec2i(s.rect.x, s.rect.y);
        self.bitblt(&t, frame, pos);
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
