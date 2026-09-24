mod datapack;
mod fields;
mod indicators;
use datapack::export_datapack;
use fields::{
    Axis, Field, Sample, Tiling, abs, add, clamp, constant, debug_export, distance_to_point, div,
    gradient, mul, negate, normalize_field, normalize_sample, opensimplex, sample, spline, sub,
};

fn generate_temperature(seed: u32, world_size: usize) -> Sample {
    let noise = opensimplex(seed, 1.0, 4, 0.005, 2.0, 0.5);
    let noise_normalized = normalize_field(noise, 256, 256);

    // only supporting north pole for now
    let latitude = gradient(
        Axis::Y,
        Tiling::ClampedToEdge,
        (world_size as f32 * -1.0) / 2.0,
        world_size as f32 / 2.0,
        0.0,
        1.0,
    );

    let temperature = mul(noise_normalized, latitude);

    normalize_sample(&sample(&temperature, world_size, world_size))
}

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

fn generate_biomes(elevation: &Sample, temperature: &Sample) -> Vec<[u8; 4]> {
    let mut biomes = Vec::new();
    for i in 0..elevation.height {
        for j in 0..elevation.width {
            let index = i * elevation.width + j;
            let elev = elevation.array[index];
            let temp = temperature.array[index];

            let biome = if elev < 0.5 {
                [0, 0, 255, 255] // minecraft:ocean
            } else if elev < 0.6 {
                [255, 255, 0, 255] // minecraft:beach
            } else if elev < 0.7 {
                if temp < 0.3 {
                    [170, 255, 170, 255] // minecraft:snowy_taiga
                } else {
                    [0, 255, 0, 255] // minecraft:forest
                }
            } else {
                if temp < 0.3 {
                    [255, 255, 255, 255] // minecraft:frozen_peaks
                } else {
                    [136, 136, 136, 255] // minecraft:stony_peaks
                }
            };

            biomes.push(biome);
        }
    }

    biomes
}

fn main() {
    let seed = 0;
    let world_size = 2400;

    println!("Generating world with size {}x{}", world_size, world_size);

    let temperature = generate_temperature(seed, world_size);
    debug_export(&temperature, "temperature");

    let elevation = generate_elevation(seed, world_size);
    let biomes = generate_biomes(&elevation, &temperature);

    export_datapack("Fictrix", world_size, &elevation, biomes);
    println!("Datapack exported");
}
