use std::{
    collections::VecDeque,
    time::{Duration, Instant},
};

pub struct AveragingFpsCounter {
    frames: VecDeque<Instant>,
    duration: Duration,
}

impl AveragingFpsCounter {
    pub fn new(duration: Duration) -> Self {
        Self {
            frames: VecDeque::new(),
            duration,
        }
    }

    pub fn tick(&mut self) -> Option<f64> {
        let now = Instant::now();
        self.frames.push_back(now);
        let earlier = now - self.duration;

        let num_frames = self.frames.len();
        if num_frames < 2 {
            return None;
        }
        let duration = *self.frames.back()? - *self.frames.front()?;
        let frames_per_second = (num_frames - 1) as f64 / duration.as_secs_f64();
        self.frames.retain(|&frame| frame >= earlier);

        Some(frames_per_second)
    }
}
