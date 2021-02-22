use crate::texture::Texture;
use crate::types::Rect;
use crate::Vec2i;
use std::rc::Rc;

pub struct Background {
    image: Rc<Texture>,
    frame: Rect,
}

impl Background {
    pub fn new(image: &Rc<Texture>, width: usize, height: usize) -> Self {
        Self {
            image: Rc::clone(image),
            frame: Rect {
                x: 0,
                y: 0,
                h: height as u16,
                w: width as u16,
            },
        }
    }

    pub fn tick_right(&mut self, w: usize) {
        if self.frame.x as usize + w < self.image.width {
            self.frame.x += 1;
        }
    }
}

pub trait DrawBackgroundExt {
    fn draw_background(&mut self, b: &Background);
}

use crate::screen::Screen;
impl<'fb> DrawBackgroundExt for Screen<'fb> {
    fn draw_background(&mut self, b: &Background) {
        // This works because we're only using a public method of Screen here,
        // and the private fields of sprite are visible inside this module
        let frame = b.frame;
        self.bitblt(&b.image, frame, Vec2i(0, 0));
    }
}
