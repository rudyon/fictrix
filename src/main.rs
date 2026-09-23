mod datapack;
mod fields;
use datapack::export_datapack;
use fields::{OpenSimplex, Sample, abs, normalize, sample, spline};

fn generate_elevation(seed: u32, world_size: usize) -> Sample {
    let continents_noise = abs(OpenSimplex::new(seed, 32.0));
    let continents = spline(
        continents_noise,
        vec![
            (0.0, 0.0, 0.0),
            (0.6, 0.2, 1.0),
            (0.8, 0.4, 1.0),
            (1.0, 1.0, 1.0),
        ],
    );

    let elevation = sample(&continents, world_size, world_size);
    let elevation_normalized = normalize(&elevation);

    elevation_normalized
}

fn generate_biomes(elevation: &Sample) -> Vec<[u8; 4]> {
    elevation
        .array
        .iter()
        .map(|&value| {
            if value < 0.25 {
                [0, 0, 255, 255] // minecraft:ocean
            } else {
                [0, 255, 0, 255] // minecraft:plains
            }
        })
        .collect::<Vec<[u8; 4]>>()
}

fn main() {
    let seed = 0;
    let world_size = 256;

    let elevation = generate_elevation(seed, world_size);
    let biomes = generate_biomes(&elevation);

    export_datapack("output/Fictrix", world_size, &elevation, biomes);
}
