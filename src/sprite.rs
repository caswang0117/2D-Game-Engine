use crate::animation::Animation;
use crate::animation::AnimationState;
use crate::texture::Texture;
use crate::types::Vec2i;
use crate::Rect;
use std::rc::Rc;

#[derive(Clone)]
pub struct SpriteID(pub usize);

#[derive(Clone)]
pub struct Sprite {
    image: Rc<Texture>,
    pub animation: Rc<Animation>,
    pub animation_state: AnimationState,
    pub rect: Rect,
    pub vx: f32,
    pub vy: f32,
}

impl Sprite {
    pub fn new(
        image: &Rc<Texture>,
        animation: &Rc<Animation>,
        state: AnimationState,
        rect: Rect,
        vx: f32,
        vy: f32,
    ) -> Self {
        Self {
            image: Rc::clone(image),
            animation: Rc::clone(animation),
            animation_state: state,
            rect: rect,
            vx: vx,
            vy: vy,
        }
    }

    pub fn tick_forward(&mut self) {
        self.animation_state.current_tick += 1;
        let mut current_tick = self.animation_state.current_tick;
        if self.animation.looping {
            current_tick %= self.animation.duration;
        } else if current_tick > self.animation.duration {
            self.animation_state.done = true;
        }
        self.animation_state.current_tick = current_tick;
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
        let frame = s.animation.current_frame(s.animation_state.current_tick);
        let position = Vec2i(s.rect.x, s.rect.y);
        self.bitblt(&s.image, frame, position);
    }
}
