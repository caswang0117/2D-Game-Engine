use crate::types::Vec2f;
use crate::Rect;
use crate::Texture;
use std::rc::Rc;

const CHAR_SIZE: i32 = 16;
const COLUMNS: i32 = 16;

#[derive(Clone)]
pub struct Font {
    pub image: Rc<Texture>,
}

impl Font {
    pub fn char_to_pos(&self, c: char) -> Rect {
        let mut x = ((c as u32) - 31) as i32 % COLUMNS * CHAR_SIZE - CHAR_SIZE;
        let y = ((c as u32) - 32) as i32 / COLUMNS * CHAR_SIZE;

        if x < 0 {
            x += CHAR_SIZE * COLUMNS;
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
    fn draw_text(&mut self, t: &mut Text);
}

use crate::screen::Screen;
impl<'fb> DrawTextExt for Screen<'fb> {
    fn draw_text(&mut self, t: &mut Text) {
        let mut pos = t.pos.0;
        for c in t.message.chars() {
            let frame = t.font.char_to_pos(c);
            self.bitblt(&t.font.image, frame, Vec2f(pos, t.pos.1));
            pos += CHAR_SIZE as f32;
            // t.pos.0 += CHAR_SIZE as f32;
        }
    }
}

pub struct Text {
    pub font: Rc<Font>,
    pub message: String,
    pub pos: Vec2f,
    pub length: usize,
}

impl Text {
    pub fn new(font: Rc<Font>, message: &str, pos: Vec2f) -> Self {
        let length = message.chars().count() * CHAR_SIZE as usize;
        let message = message.to_string();
        Self {
            font,
            message,
            pos,
            length,
        }
    }
}
