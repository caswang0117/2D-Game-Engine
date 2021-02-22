#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub w: u16,
    pub h: u16,
}

impl Rect {
    pub fn create_frames(row: u16, nframes: u16, w: u16, h: u16) -> Vec<Rect> {
        let mut frames = vec![];
        for i in 0..nframes {
            let rect = Rect {
                x: (i * w) as i32,
                y: (row * h) as i32,
                w: w,
                h: h,
            };
            frames.push(rect);
        }
        frames
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub struct Vec2i(pub i32, pub i32);

#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub struct Rgba(pub u8, pub u8, pub u8, pub u8);
