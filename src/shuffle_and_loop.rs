use image::{Rgb, RgbImage};
use rand::{seq::SliceRandom, thread_rng};

pub struct ShuffleAndLoop<'a> {
    pixels: Vec<(u32, u32, &'a Rgb<u8>)>,
}

impl<'a> ShuffleAndLoop<'a> {
    pub fn new(image: &'a RgbImage) -> Self {
        let mut pixels: Vec<_> = image
            .enumerate_pixels()
            .filter(|(_, _, color)| color[1] > 0)
            .collect();
        pixels.shuffle(&mut thread_rng());
        Self { pixels }
    }

    pub fn iter(&self) -> impl Iterator<Item = &(u32, u32, &'a Rgb<u8>)> {
        self.pixels.iter()
    }
}
