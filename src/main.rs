mod datapack;
mod fields;
use datapack::export_datapack;
use fields::{OpenSimplex, Sample, normalize, sample};

fn generate_elevation(seed: u32, world_size: usize) -> Sample {
    let noise = OpenSimplex::new(seed, 32.0);

    let elevation = sample(&noise, world_size, world_size);
    let elevation_normalized = normalize(&elevation);

    elevation_normalized
}

fn generate_biomes(elevation: &Sample) -> Vec<[u8; 4]> {
    elevation
        .array
        .iter()
        .map(|&value| {
            if value < 0.3 {
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
