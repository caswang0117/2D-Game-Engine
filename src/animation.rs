use crate::types::Rectf;

#[derive(PartialEq, Clone, Debug)]
pub struct Animation {
    pub frames: Vec<Rectf>,
    pub timing: Vec<usize>,
    pub looping: bool,
    pub done: bool,
    pub duration: usize,
}

impl Animation {
    pub fn new(frames: Vec<Rectf>, timing: Vec<usize>, looping: bool) -> Self {
        let duration = timing.iter().sum();
        Self {
            frames,
            timing,
            looping,
            done: false,
            duration,
        }
    }
    pub fn current_frame(&self, current_tick: usize) -> Rectf {
        let mut ticks_so_far = 0;
        for (frame, ticks) in self.frames.iter().zip(self.timing.iter()) {
            ticks_so_far += ticks;
            if ticks_so_far >= current_tick {
                return *frame;
            }
        }
        return *self.frames.last().unwrap(); // If current_ticks is longer than duration of animation
    }
}

#[derive(Clone)]
pub struct AnimationState {
    pub current_tick: usize,
    pub done: bool,
}

impl AnimationState {
    pub fn new() -> Self {
        Self {
            current_tick: 0,
            done: false,
        }
    }
}
