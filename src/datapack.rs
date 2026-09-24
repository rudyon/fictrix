use crate::fields::Sample;
use image::{GrayImage, ImageBuffer, RgbaImage};
use serde_json::{json, to_string_pretty};
use std::{fs, path::Path};

pub fn export_datapack(path: &str, world_size: usize, elevation: &Sample, biomes: Vec<[u8; 4]>) {
    let pack_mcmeta = json!({
        "pack": {
            "description": "Fictrix",
            "min_format": 121,
            "max_format": 121
        }
    });

    let dimension = json!({
        "type": "minecraft:overworld",
        "generator": {
            "type": "novoatlas:image_map",
            "map_info": "fictrix:overworld",
            "settings": "minecraft:overworld",
            "underground_density_function": "novoatlas:caves",
            "biome_source": {
                "type": "novoatlas:color_map",
                "map_info": "fictrix:overworld",
                "default_biome": "minecraft:the_void"
            }
        }
    });

    let map_info = json!({
        "height_map": "fictrix:overworld",
        "starting_y": 0,
        "surface_biomes": {
            "map": "fictrix:overworld",
            "biomes": [
                {
                    "biome": "minecraft:ocean",
                    "color": "#0000ff"
                },
                {
                    "biome": "minecraft:beach",
                    "color": "#ffff00"
                },
                {
                    "biome": "minecraft:snowy_taiga",
                    "color": "#aaffaa"
                },
                {
                    "biome": "minecraft:forest",
                    "color": "#00ff00"
                },
                {
                    "biome": "minecraft:frozen_peaks",
                    "color": "#ffffff"
                },
                {
                    "biome": "minecraft:stony_peaks",
                    "color": "#888888"
                }
            ]
        }
    });

    let heightmap: GrayImage =
        ImageBuffer::from_fn(world_size as u32, world_size as u32, |x, y| {
            let index = (y as usize) * world_size + (x as usize);
            let value = elevation.array[index];
            let pixel_value = (value * 125.0) as u8;
            image::Luma([pixel_value])
        });

    let biomemap: RgbaImage = ImageBuffer::from_fn(world_size as u32, world_size as u32, |x, y| {
        let index = (y as usize) * world_size + (x as usize);
        let pixel_value = biomes[index];
        image::Rgba(pixel_value)
    });

    let export_path = Path::new(path);

    fs::create_dir_all(export_path.join("data/minecraft/dimension")).unwrap();
    fs::create_dir_all(export_path.join("data/fictrix/novoatlas/map_info")).unwrap();
    fs::create_dir_all(export_path.join("data/fictrix/novoatlas/heightmap")).unwrap();
    fs::create_dir_all(export_path.join("data/fictrix/novoatlas/biome_map")).unwrap();

    fs::write(
        export_path.join("pack.mcmeta"),
        to_string_pretty(&pack_mcmeta).unwrap(),
    )
    .unwrap();
    fs::write(
        export_path.join("data/minecraft/dimension/overworld.json"),
        to_string_pretty(&dimension).unwrap(),
    )
    .unwrap();
    fs::write(
        export_path.join("data/fictrix/novoatlas/map_info/overworld.json"),
        to_string_pretty(&map_info).unwrap(),
    )
    .unwrap();

    heightmap
        .save(export_path.join("data/fictrix/novoatlas/heightmap/overworld.png"))
        .unwrap();
    biomemap
        .save(export_path.join("data/fictrix/novoatlas/biome_map/overworld.png"))
        .unwrap();
}
