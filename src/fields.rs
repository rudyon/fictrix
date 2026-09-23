use noise::{NoiseFn, OpenSimplex as NoiseOpenSimplex};

// Core trait
pub trait Field {
    fn evaluate(&self, x: f32, y: f32) -> f32;
}

// Field implementations
pub struct OpenSimplex {
    noise: NoiseOpenSimplex,
    pub scale: f32,
}

pub struct Abs<F> {
    field: F,
}

pub struct Spline<F> {
    field: F,
    pub points: Vec<(f32, f32, f32)>,
}

// Constructors / implementations
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

impl<F: Field> Field for Abs<F> {
    fn evaluate(&self, x: f32, y: f32) -> f32 {
        self.field.evaluate(x, y).abs()
    }
}

impl<F: Field> Field for Spline<F> {
    fn evaluate(&self, x: f32, y: f32) -> f32 {
        let value = self.field.evaluate(x, y);
        let mut result = 0.0;

        for i in 0..self.points.len() - 1 {
            let (x0, y0, m0) = self.points[i];
            let (x1, y1, m1) = self.points[i + 1];

            if value >= x0 && value <= x1 {
                let t = (value - x0) / (x1 - x0);
                let h00 = (2.0 * t.powi(3)) - (3.0 * t.powi(2)) + 1.0;
                let h10 = t.powi(3) - (2.0 * t.powi(2)) + t;
                let h01 = (-2.0 * t.powi(3)) + (3.0 * t.powi(2));
                let h11 = t.powi(3) - t.powi(2);

                result = h00 * y0 + h10 * m0 * (x1 - x0) + h01 * y1 + h11 * m1 * (x1 - x0);
                break;
            }
        }

        result
    }
}

// Field operations
pub fn abs<F: Field>(field: F) -> Abs<F> {
    Abs { field }
}

pub fn spline<F: Field>(field: F, points: Vec<(f32, f32, f32)>) -> Spline<F> {
    Spline { field, points }
}

// Sampling
pub struct Sample {
    pub width: usize,
    pub height: usize,
    pub array: Vec<f32>,
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

// Sample operations
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
