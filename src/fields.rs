use crate::indicators::progress_bar;
use noise::{Fbm, MultiFractal, NoiseFn};

pub enum Axis {
    X,
    Y,
}

pub enum Tiling {
    ClampedToEdge,
    Repeat,
    MirrorRepeat,
}

// Core trait
pub trait Field {
    fn evaluate(&self, x: f32, y: f32) -> f32;
}

// Field implementations
pub struct OpenSimplex {
    noise: Fbm<noise::OpenSimplex>,
    amplitude: f32,
}

pub struct Constant {
    value: f32,
}

pub struct DistanceToPoint {
    point: (f32, f32),
}

pub struct Gradient {
    axis: Axis,
    tiling: Tiling,
    from_coord: f32,
    to_coord: f32,
    from_value: f32,
    to_value: f32,
}

pub struct NormalizedField<F> {
    field: F,
    min: f32,
    max: f32,
}

pub struct Abs<F> {
    field: F,
}

pub struct Spline<F> {
    field: F,
    points: Vec<(f32, f32, f32)>,
}

pub struct Sub<A, B> {
    left: A,
    right: B,
}

pub struct Negate<F> {
    field: F,
}

pub struct Mul<A, B> {
    left: A,
    right: B,
}

pub struct Add<A, B> {
    left: A,
    right: B,
}

pub struct Clamp<F> {
    field: F,
    min: f32,
    max: f32,
}

pub struct Div<A, B> {
    left: A,
    right: B,
}

// Constructors / implementations
impl Field for OpenSimplex {
    fn evaluate(&self, x: f32, y: f32) -> f32 {
        self.noise.get([x as f64, y as f64]) as f32 * self.amplitude
    }
}

impl Field for Constant {
    fn evaluate(&self, _x: f32, _y: f32) -> f32 {
        self.value
    }
}

impl Field for DistanceToPoint {
    fn evaluate(&self, x: f32, y: f32) -> f32 {
        let dx = x - self.point.0;
        let dy = y - self.point.1;
        (dx * dx + dy * dy).sqrt()
    }
}

impl Field for Gradient {
    fn evaluate(&self, x: f32, y: f32) -> f32 {
        let coord = match self.axis {
            Axis::X => x,
            Axis::Y => y,
        };

        let t = match self.tiling {
            Tiling::ClampedToEdge => {
                if coord <= self.from_coord {
                    0.0
                } else if coord >= self.to_coord {
                    1.0
                } else {
                    (coord - self.from_coord) / (self.to_coord - self.from_coord)
                }
            }
            Tiling::Repeat => {
                let range = self.to_coord - self.from_coord;
                let wrapped_coord =
                    ((coord - self.from_coord) % range + range) % range + self.from_coord;
                (wrapped_coord - self.from_coord) / range
            }
            Tiling::MirrorRepeat => {
                let range = self.to_coord - self.from_coord;
                let wrapped_coord =
                    ((coord - self.from_coord) % (2.0 * range) + (2.0 * range)) % (2.0 * range);
                let mirrored_coord = if wrapped_coord < range {
                    wrapped_coord
                } else {
                    2.0 * range - wrapped_coord
                };
                (mirrored_coord + self.from_coord) / range
            }
        };

        self.from_value + t * (self.to_value - self.from_value)
    }
}

impl<F: Field> Field for NormalizedField<F> {
    fn evaluate(&self, x: f32, y: f32) -> f32 {
        let value = self.field.evaluate(x, y);
        (value - self.min) / (self.max - self.min)
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

impl<A: Field, B: Field> Field for Sub<A, B> {
    fn evaluate(&self, x: f32, y: f32) -> f32 {
        self.left.evaluate(x, y) - self.right.evaluate(x, y)
    }
}

impl<F: Field> Field for Negate<F> {
    fn evaluate(&self, x: f32, y: f32) -> f32 {
        -self.field.evaluate(x, y)
    }
}

impl<A: Field, B: Field> Field for Mul<A, B> {
    fn evaluate(&self, x: f32, y: f32) -> f32 {
        self.left.evaluate(x, y) * self.right.evaluate(x, y)
    }
}

impl<A: Field, B: Field> Field for Add<A, B> {
    fn evaluate(&self, x: f32, y: f32) -> f32 {
        self.left.evaluate(x, y) + self.right.evaluate(x, y)
    }
}

impl<F: Field> Field for Clamp<F> {
    fn evaluate(&self, x: f32, y: f32) -> f32 {
        let value = self.field.evaluate(x, y);
        value.clamp(self.min, self.max)
    }
}

impl<A: Field, B: Field> Field for Div<A, B> {
    fn evaluate(&self, x: f32, y: f32) -> f32 {
        self.left.evaluate(x, y) / self.right.evaluate(x, y)
    }
}

pub fn opensimplex(
    seed: u32,
    amplitude: f32,
    octaves: usize,
    frequency: f32,
    lacunarity: f32,
    persistence: f32,
) -> OpenSimplex {
    OpenSimplex {
        noise: Fbm::<noise::OpenSimplex>::new(seed)
            .set_octaves(octaves)
            .set_frequency(frequency as f64)
            .set_lacunarity(lacunarity as f64)
            .set_persistence(persistence as f64),
        amplitude,
    }
}

pub fn constant(value: f32) -> Constant {
    Constant { value }
}

pub fn distance_to_point(x: f32, y: f32) -> DistanceToPoint {
    DistanceToPoint { point: (x, y) }
}

pub fn gradient(
    axis: Axis,
    tiling: Tiling,
    from_coord: f32,
    to_coord: f32,
    from_value: f32,
    to_value: f32,
) -> Gradient {
    Gradient {
        axis,
        tiling,
        from_coord,
        to_coord,
        from_value,
        to_value,
    }
}

// Field operations
pub fn normalize_field<F: Field>(field: F, width: usize, height: usize) -> NormalizedField<F> {
    let bar = progress_bar((width * height) as u64, "Normalizing");

    let mut min = f32::INFINITY;
    let mut max = f32::NEG_INFINITY;

    for i in 0..height {
        for j in 0..width {
            let x = j as f32 - width as f32 / 2.0;
            let y = i as f32 - height as f32 / 2.0;
            let value = field.evaluate(x, y);
            if value < min {
                min = value;
            }
            if value > max {
                max = value;
            }
            bar.inc(1);
        }
    }
    bar.finish();

    NormalizedField { field, min, max }
}

pub fn abs<F: Field>(field: F) -> Abs<F> {
    Abs { field }
}

pub fn spline<F: Field>(field: F, points: Vec<(f32, f32, f32)>) -> Spline<F> {
    Spline { field, points }
}

pub fn sub<A: Field, B: Field>(left: A, right: B) -> Sub<A, B> {
    Sub { left, right }
}

pub fn negate<F: Field>(field: F) -> Negate<F> {
    Negate { field }
}

pub fn mul<A: Field, B: Field>(left: A, right: B) -> Mul<A, B> {
    Mul { left, right }
}

pub fn add<A: Field, B: Field>(left: A, right: B) -> Add<A, B> {
    Add { left, right }
}

pub fn clamp<F: Field>(field: F, min: f32, max: f32) -> Clamp<F> {
    Clamp { field, min, max }
}

pub fn div<A: Field, B: Field>(left: A, right: B) -> Div<A, B> {
    Div { left, right }
}

// Sampling
pub struct Sample {
    pub width: usize,
    pub height: usize,
    pub array: Vec<f32>,
}

pub fn sample<F: Field>(field: &F, width: usize, height: usize) -> Sample {
    let bar = progress_bar((width * height) as u64, "Sampling");
    let mut array = vec![0.0; width * height];

    for i in 0..height {
        for j in 0..width {
            let x = j as f32 - width as f32 / 2.0; // Center the x-coordinate
            let y = i as f32 - width as f32 / 2.0; // Center the y-coordinate
            let value = field.evaluate(x, y);
            array[i * width + j] = value;
            bar.inc(1);
        }
    }
    bar.finish();

    Sample {
        width,
        height,
        array,
    }
}

// Sample operations
pub fn normalize_sample(sample: &Sample) -> Sample {
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
