use crate::texture::Texture;
use crate::types::{Rect, Vec2i};
use std::rc::Rc;

pub struct Background {
    image: Rc<Texture>,
    frame: Rect,
}