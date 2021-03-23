use crate::texture::Texture;
use crate::tiles::*;
use crate::types::{Rect, Rgba, Vec2i};
use std::rc::Rc;

pub struct Obstacle {
    pub image: Option<Rc<Texture>>,
    pub frame: Option<Rect>,
    pub tile_id: Option<TileID>,
    pub rect: Option<Rect>, // on tilemap for collisions
    pub destroyed: bool,
}

impl Obstacle {
    pub fn new(
        image: Option<Rc<Texture>>,
        frame: Option<Rect>,
        tile_id: Option<TileID>,
        rect: Option<Rect>,
    ) -> Self {
        Self {
            image,
            frame,
            tile_id,
            rect,
            destroyed: false,
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
                let image = o.image.as_ref().unwrap();
                let frame = o.frame.unwrap();
                self.bitblt(&image, frame, Vec2i(frame.x, frame.y));
            }
            None => self.rect(o.rect.unwrap(), Rgba(200, 200, 200, 255)),
        }
    }
}

// ----------------------------------------------------------------- new?
pub struct Obstacle2 {
    pub image: Rc<Texture>,
    pub frame: Rect,
    pub tile_id: TileID,
    pub destroyed: bool,
}

impl Obstacle2 {
    pub fn new(image: Rc<Texture>, frame: Rect, tile_id: TileID, rect: Rect) -> Self {
        Obstacle2 {
            image,
            frame,
            tile_id,
            destroyed: false,
        }
    }
}

pub trait DrawObstacle2Ext {
    fn draw_obstacle2(&mut self, o: &Obstacle2);
}

impl<'fb> DrawObstacle2Ext for Screen<'fb> {
    fn draw_obstacle2(&mut self, o: &Obstacle2) {
        // This works because we're only using a public method of Screen here,
        // and the private fields of sprite are visible inside this module
        self.bitblt(&o.image, o.frame, Vec2i(o.frame.x, o.frame.y));
    }
}

pub struct Terrain {
    pub rect: Option<Rect>, // on tilemap for collisions
    pub destroyed: bool,
}

pub trait DrawTerrainExt {
    fn draw_terrain(&mut self, o: &Terrain, c: Rgba);
}

impl<'fb> DrawTerrainExt for Screen<'fb> {
    fn draw_terrain(&mut self, o: &Terrain, c: Rgba) {
        // This works because we're only using a public method of Screen here,
        // and the private fields of sprite are visible inside this module
        self.rect(o.rect.unwrap(), c)
    }
}
