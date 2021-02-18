use crate::types::Rect;

#[derive(PartialEq, Eq, Clone, Hash, Debug)]
pub struct Animation {
    // Do this for the exercise today!
    // You'll want to know the frames involved and the timing for each frame
    // But then there's also dynamic data, which might live in this struct or might live somewhere else
    // An Animation/AnimationState split could be fine, if AnimationState holds the start time and the present frame (or just the start time) and possibly a reference to the Animation
    // but there are lots of designs that will work!
    frames: Vec<Rect>,
    timing: Vec<usize>,
    current_tick: usize,
    looping: bool,
    done: bool,
    duration: usize,
}

impl Animation {
    // Should hold some data...
    // Be used to decide what frame to use...
    // And sprites can be updated based on that information.
    // Or we could give sprites an =animation= field instead of a =frame=!
    // Could have a query function like current_frame(&self, start_time:usize, now:usize, speedup_factor:usize)
    // Or could be ticked in-place

    pub fn new(frames: Vec<Rect>, timing: Vec<usize>, looping: bool) -> Self {
        let duration = timing.iter().sum();
        Self {
            frames: frames,
            timing: timing,
            current_tick: 0,
            looping: looping,
            done: false,
            duration: duration,
        }
    }
    
    pub fn current_frame(&self) -> Rect {
        let mut ticks_so_far = 0;
        for (frame, ticks) in self.frames.iter().zip(self.timing.iter()) {
            ticks_so_far += ticks;
            if ticks_so_far >= self.current_tick {
                return *frame;
            }
        }
        return *self.frames.last().unwrap(); // If current_ticks is longer than duration of animation
    }

    pub fn tick_forward(&mut self) {
        self.current_tick += 1;
        if self.looping {
            self.current_tick = self.current_tick % self.duration;
        } else if self.current_tick > self.duration {
            self.done = true;
        }
    }

}