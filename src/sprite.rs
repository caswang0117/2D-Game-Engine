use crate::animation::Animation;
use crate::texture::Texture;
use crate::types::{Rect, Vec2i};
use std::rc::Rc;

pub struct Sprite {
    image: Rc<Texture>,
    // pub frame: Rect, // Maybe better to use a type that can't have a negative origin
    // Or use =animation:Animation= instead of a frame field
    pub animation: Animation,
    pub position: Vec2i,
}

impl Sprite {
    pub fn new(image: &Rc<Texture>, animation: Animation, position: Vec2i) -> Self {
        Self {
            image: Rc::clone(image),
            animation: animation,
            position: position,
        }
    }
}

pub trait DrawSpriteExt {
    fn draw_sprite(&mut self, s: &Sprite);
}

use crate::screen::Screen;
impl<'fb> DrawSpriteExt for Screen<'fb> {
    fn draw_sprite(&mut self, s: &Sprite) {
        // This works because we're only using a public method of Screen here,
        // and the private fields of sprite are visible inside this module
        let frame = s.animation.current_frame();
        self.bitblt(&s.image, frame, s.position);
    }
}
