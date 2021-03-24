#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub w: u16,
    pub h: u16,
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub struct Rectf {
    pub x: f32,
    pub y: f32,
    pub w: u16,
    pub h: u16,
}
impl Rectf {
    pub fn create_frames(row: u16, nframes: u16, w: u16, h: u16) -> Vec<Rectf> {
        let mut frames = vec![];
        for i in 0..nframes {
            let rect = Rectf {
                x: (i * w) as f32,
                y: (row * h) as f32,
                w,
                h,
            };
            frames.push(rect);
        }
        frames
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub struct Vec2i(pub i32, pub i32);

#[derive(PartialEq, Clone, Copy, Debug)]
pub struct Vec2f(pub f32, pub f32);

#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub struct Rgba(pub u8, pub u8, pub u8, pub u8);
