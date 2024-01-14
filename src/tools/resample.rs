pub struct Resample<I, T>
where
I: Iterator<Item = T>,
T: Copy {
    ratio: f32,
    source: I,
}

impl<I, T> Resample<I, T>
where
I: Iterator<Item = T>,
T: Copy {
    pub fn new(freq1: usize, freq2: usize, source: I) -> Self {
        Resample {
            ratio: freq1 as f32 / freq2 as f32,
            source,
        }
    }

    #[inline(always)]
    fn lerp(u: f32, v: f32, t: f32) -> f32 {
        u + t * (v - u)
    }

}

impl<I, T> Iterator for Resample<I, T>
where
I: Iterator<Item = T>,
T: Copy {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.ratio > 1.0 {
            for _ in 0..self.ratio as usize {
                self.source.next();
            }
        } else {
            println!("[Resample] todo lerp");
        }
        self.source.next()
    }
}
