mod fields;
use fields::{OpenSimplex, normalize, sample};
use image::{GrayImage, ImageBuffer};

fn generate_world(seed: u32, world_size: usize) -> Vec<f32> {
    let noise = OpenSimplex::new(seed, 16.0);

    let elevation = sample(&noise, world_size, world_size);
    let elevation_normalized = normalize(&elevation);

    elevation_normalized.array
}

fn main() {
    let seed = 0;
    let world_size = 1600;

    let elevation = generate_world(seed, world_size);

    let heightmap: GrayImage =
        ImageBuffer::from_fn(world_size as u32, world_size as u32, |x, y| {
            let index = (y as usize) * world_size + (x as usize);
            let value = elevation[index];
            let pixel_value = (value * 255.0) as u8;
            image::Luma([pixel_value])
        });

    heightmap
        .save("heightmap.png")
        .expect("Failed to save heightmap");
}
