use crate::texture::Texture;
use crate::tiles::*;
use crate::types::{Rect, Rgba, Vec2i};
use std::rc::Rc;

pub struct Obstacle {
    pub image: Option<Rc<Texture>>,
    pub tile_id: Option<TileID>,
    pub rect: Option<Rect>, // on tilemap for collisions
    pub destroyed: bool,
}

impl Obstacle {
    pub fn new(image: Option<Rc<Texture>>, tile_id: Option<TileID>, rect: Option<Rect>) -> Self {
        Self{
            image,
            tile_id,
            rect,
            destroyed: false
        }
    }
        // match tile{
        //     Some(id) => {
        //         Self{
        //             image,
        //             tile_id: Some(id),
        //             rect: Some(rect.unwrap()),
        //             destroyed: false,
        //         }
        //     }
        //     None => Err("Invalid tile ID")
        // }

}

pub trait DrawObstacleExt {
    fn draw_obstacle(&mut self, o: &Obstacle);
}

use crate::screen::Screen;
impl<'fb> DrawObstacleExt for Screen<'fb> {
    fn draw_obstacle(&mut self, o: &Obstacle) {
        // This works because we're only using a public method of Screen here,
        // and the private fields of sprite are visible inside this module
        match &o.tile_id {
            Some(i) => {
                let frame = o.rect.unwrap();
                let image = o.image.as_ref().unwrap();
                self.bitblt(&image, frame, Vec2i(frame.x, frame.y));
            }
            None => self.rect(o.rect.unwrap(), Rgba(200, 200, 200, 255)),
        }
    }
}
