use noise::{NoiseFn, OpenSimplex as NoiseOpenSimplex};

pub trait Field {
    fn evaluate(&self, x: f32, y: f32) -> f32;
}

pub struct Sample {
    pub width: usize,
    pub height: usize,
    pub array: Vec<f32>,
}

pub struct OpenSimplex {
    noise: NoiseOpenSimplex,
    pub scale: f32,
}

impl OpenSimplex {
    pub fn new(seed: u32, scale: f32) -> Self {
        Self {
            noise: NoiseOpenSimplex::new(seed),
            scale,
        }
    }
}

impl Field for OpenSimplex {
    fn evaluate(&self, x: f32, y: f32) -> f32 {
        self.noise
            .get([x as f64 / self.scale as f64, y as f64 / self.scale as f64]) as f32
    }
}

pub fn sample<F: Field>(field: &F, width: usize, height: usize) -> Sample {
    let mut array = vec![0.0; width * height];

    for i in 0..height {
        for j in 0..width {
            let x = j as f32 + width as f32 / 2.0;
            let y = i as f32 + height as f32 / 2.0;
            let value = field.evaluate(x, y);
            array[i * width + j] = value;
        }
    }

    Sample {
        width,
        height,
        array,
    }
}

pub fn normalize(sample: &Sample) -> Sample {
    let min = sample.array.iter().copied().fold(f32::INFINITY, f32::min);
    let max = sample
        .array
        .iter()
        .copied()
        .fold(f32::NEG_INFINITY, f32::max);

    let array = sample
        .array
        .iter()
        .map(|value| (value - min) / (max - min))
        .collect();

    Sample {
        width: sample.width,
        height: sample.height,
        array,
    }
}
