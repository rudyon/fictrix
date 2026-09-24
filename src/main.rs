mod datapack;
mod fields;
mod indicators;
use datapack::export_datapack;
use fields::{
    Field, Sample, abs, add, clamp, constant, distance_to_point, div, mul, negate, normalize_field,
    normalize_sample, opensimplex, sample, spline, sub,
};

fn generate_elevation(seed: u32, world_size: usize) -> Sample {
    let noise = opensimplex(seed, 1.0, 4, 0.005, 2.0, 0.5);
    let origin_distance = distance_to_point(0.0, 0.0);
    let landmask = negate(sub(
        constant(1.0),
        div(origin_distance, constant(world_size as f32 / 4.0)),
    ));

    let terrain = sub(noise, landmask);

    normalize_sample(&sample(&terrain, world_size, world_size))
}

fn generate_biomes(elevation: &Sample) -> Vec<[u8; 4]> {
    elevation
        .array
        .iter()
        .map(|&value| {
            if value < 0.5 {
                [0, 0, 255, 255] // minecraft:ocean
            } else {
                [0, 255, 0, 255] // minecraft:plains
            }
        })
        .collect::<Vec<[u8; 4]>>()
}

fn main() {
    let seed = 0;
    let world_size = 1200;

    println!("Generating world with size {}x{}", world_size, world_size);
    let elevation = generate_elevation(seed, world_size);
    let biomes = generate_biomes(&elevation);

    export_datapack("output/Fictrix", world_size, &elevation, biomes);
    println!("Datapack exported");
}
