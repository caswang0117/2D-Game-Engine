use crate::animation::Animation;
use crate::animation::AnimationState;
use crate::texture::Texture;
use crate::types::*;
use std::rc::Rc;

#[derive(Clone)]
pub struct SpriteID(pub usize);

#[derive(Clone)]
pub struct Sprite {
    image: Rc<Texture>,
    pub animation: Rc<Animation>,
    pub animation_state: AnimationState,
    pub rect: Rectf,
    pub vx: f32,
    pub vy: f32,
}

impl Sprite {
    pub fn new(
        image: &Rc<Texture>,
        animation: &Rc<Animation>,
        state: AnimationState,
        rect: Rectf,
        vx: f32,
        vy: f32,
    ) -> Self {
        Self {
            image: Rc::clone(image),
            animation: Rc::clone(animation),
            animation_state: state,
            rect,
            vx,
            vy,
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

    pub fn on_screen(
        &self,
        camera_position: Vec2f,
        screen_height: usize,
        screen_width: usize,
    ) -> bool {
        let x_on = self.rect.x >= camera_position.0
            && self.rect.x + self.rect.w as f32 <= camera_position.0 + screen_width as f32;

        // Check going off top and then bottom
        let y_on = self.rect.y + self.rect.h as f32 >= camera_position.1
            && self.rect.y as f32 <= camera_position.1 + screen_height as f32;
        x_on && y_on
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
        let frame_f = s.animation.current_frame(s.animation_state.current_tick);
        let frame = Rect {
            x: frame_f.x as i32,
            y: frame_f.y as i32,
            w: frame_f.w,
            h: frame_f.h,
        };
        let position = Vec2f(s.rect.x, s.rect.y);
        self.bitblt(&s.image, frame, position);
    }
}
