use crate::texture::Texture;
use crate::tiles::*;
use crate::types::{Rect, Rgba, Vec2i};
use std::rc::Rc;

pub struct Obstacle {
    pub tile_id: Option<TileID>,
    pub rect: Option<Rect>, // on tilemap for collisions
    pub destroyed: bool,
}

impl Obstacle {
    pub fn new(tile_id: Option<TileID>, rect: Rect){
        match tile_id{
            Some(id) => {
                Obstacle{
                    tile_id: tile_id,
                    rect: 
                }
            }
            None => Obstacle {
                
            }
        }

    }
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
                let frame = o.frame.unwrap();
                self.bitblt(&i, frame, Vec2i(frame.x, frame.y));
            }
            None => self.rect(o.rect, Rgba(200, 200, 200, 255)),
        }
    }
}
