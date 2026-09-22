trait Field {
    fn evaluate(&self, x: f32, y: f32) -> f32;
}

struct Sample {
    pub width: usize,
    pub height: usize,
    pub array: Vec<f32>,
}

struct Normalized<F> {
    field: F,
    min: f32,
    max: f32,
}

impl<F: Field> Field for Normalized<F> {
    fn evaluate(&self, x: f32, y: f32) -> f32 {
        let value = self.field.evaluate(x, y);
        (value - self.min) / (self.max - self.min)
    }
}

fn sample<F: Field>(field: &F, width: usize, height: usize) -> Sample {
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

fn normalize(sample: &Sample) -> Sample {
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

fn normalize_field<F: Field>(field: F, width: usize, height: usize) -> Normalized<F> {
    let sample = sample(&field, width, height);
    let min = sample.array.iter().copied().fold(f32::INFINITY, f32::min);
    let max = sample
        .array
        .iter()
        .copied()
        .fold(f32::NEG_INFINITY, f32::max);

    Normalized { field, min, max }
}
