import json
from pathlib import Path

import numpy as np
from PIL import Image

from field import normalize, simplex


def generate_world(seed=0):

    elevation = normalize(simplex(seed=seed, octave=4.0).array(256, 256))

    biomes = np.zeros((256, 256, 4), dtype=np.uint8)
    for i in range(256):
        for j in range(256):
            if elevation[i, j] < 0.5:
                biomes[i, j] = [0, 0, 255, 255]  # minecraft:ocean
            else:
                biomes[i, j] = [0, 255, 0, 255]  # minecraft:plains

    return elevation, biomes


def export_datapack(path, elevation, biomes):
    pack_mcmeta = {
        "pack": {"description": "Fictrix", "min_format": 121, "max_format": 121}
    }

    dimension = {
        "type": "minecraft:overworld",
        "generator": {
            "type": "novoatlas:image_map",
            "map_info": "fictrix:overworld",
            "settings": "minecraft:overworld",
            "underground_density_function": "novoatlas:caves",
            "biome_source": {
                "type": "novoatlas:biome_cell_color_map",
                "map_info": "fictrix:overworld",
                "default_biome": "minecraft:the_void",
            },
        },
    }

    map_info = {
        "height_map": "fictrix:overworld",
        "starting_y": 0,
        "surface_biomes": {
            "map": "fictrix:overworld",
            "biomes": [
                {"biome": "minecraft:plains", "color": "#00ff00"},
                {"biome": "minecraft:ocean", "color": "#0000ff"},
            ],
        },
    }

    heightmap = Image.fromarray((elevation * 255).astype(np.uint8), mode="L")
    biomemap = Image.fromarray(biomes, mode="RGBA")

    path.mkdir(parents=True, exist_ok=True)
    path.joinpath("data/fictrix/novoatlas").mkdir(parents=True, exist_ok=True)
    path.joinpath("data/minecraft/dimension").mkdir(parents=True, exist_ok=True)
    path.joinpath("data/fictrix/novoatlas/map_info").mkdir(parents=True, exist_ok=True)
    path.joinpath("data/fictrix/novoatlas/heightmap").mkdir(parents=True, exist_ok=True)
    path.joinpath("data/fictrix/novoatlas/biome_map").mkdir(parents=True, exist_ok=True)

    with open(path / "pack.mcmeta", "w") as f:
        json.dump(pack_mcmeta, f, indent=4)

    with open(path / "data/minecraft/dimension/overworld.json", "w") as f:
        json.dump(dimension, f, indent=4)

    with open(path / "data/fictrix/novoatlas/map_info/overworld.json", "w") as f:
        json.dump(map_info, f, indent=4)

    heightmap.save(path / "data/fictrix/novoatlas/heightmap/overworld.png")
    biomemap.save(path / "data/fictrix/novoatlas/biome_map/overworld.png")


def main():
    elevation, biomes = generate_world()
    export_datapack(Path("output/Fictrix/"), elevation, biomes)


if __name__ == "__main__":
    main()
