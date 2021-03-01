use crate::texture::Texture;
use crate::types::{Rect, Rgba, Vec2i};
use std::rc::Rc;

pub struct Obstacle {
    pub image: Option<Rc<Texture>>,
    pub frame: Option<Rect>, // on source image texture
    pub rect: Rect,          // on tilemap for collisions
    pub destroyed: bool,
}

pub trait DrawObstacleExt {
    fn draw_obstacle(&mut self, o: &Obstacle);
}

use crate::screen::Screen;
impl<'fb> DrawObstacleExt for Screen<'fb> {
    fn draw_obstacle(&mut self, o: &Obstacle) {
        // This works because we're only using a public method of Screen here,
        // and the private fields of sprite are visible inside this module
        match &o.image {
            Some(i) => {
                let frame = o.frame.unwrap();
                self.bitblt(&i, frame, Vec2i(frame.x, frame.y));
            }
            None => self.rect(o.rect, Rgba(0, 0, 0, 0)),
        }
    }
}
