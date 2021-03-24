use crate::types::Rectf;

#[derive(PartialEq, Clone, Debug)]
pub struct Animation {
    // Do this for the exercise today!
    // You'll want to know the frames involved and the timing for each frame
    // But then there's also dynamic data, which might live in this struct or might live somewhere else
    // An Animation/AnimationState split could be fine, if AnimationState holds the start time and the present frame (or just the start time) and possibly a reference to the Animation
    // but there are lots of designs that will work!
    pub frames: Vec<Rectf>,
    pub timing: Vec<usize>,
    pub looping: bool,
    pub done: bool,
    pub duration: usize,
}

impl Animation {
    // Should hold some data...
    // Be used to decide what frame to use...
    // And sprites can be updated based on that information.
    // Or we could give sprites an =animation= field instead of a =frame=!
    // Could have a query function like current_frame(&self, start_time:usize, now:usize, speedup_factor:usize)
    // Or could be ticked in-place

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
