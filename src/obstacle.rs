use crate::texture::Texture;
use crate::types::Rect;
use std::rc::Rc;

pub struct Obstacle {
    pub image: Rc<Texture>,
    pub rect: Rect,  // on tilemap for collisions
    pub frame: Rect, // on source image texture
    pub destroyed: bool,
}
