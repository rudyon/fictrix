from collections.abc import Callable

import numpy as np


class Field:
    def __init__(self, function: Callable[[float, float], float]):
        self.function = function

    def __call__(self, x: float, y: float) -> float:
        return self.function(x, y)

    def array(self, width: int, height: int) -> np.ndarray:
        arr = np.zeros((height, width), dtype=np.float32)
        for i in range(height):
            for j in range(width):
                arr[i, j] = self.function(j / width, i / height)
        return arr


def normalize(arr: np.ndarray) -> np.ndarray:
    min_val = np.min(arr)
    max_val = np.max(arr)
    return (arr - min_val) / (max_val - min_val)


def simplex(seed: int = 0, octave: float = 1.0) -> Field:
    import opensimplex

    opensimplex.seed(seed)

    def noise_function(x: float, y: float) -> float:
        return opensimplex.noise2(x * octave, y * octave)

    return Field(noise_function)
