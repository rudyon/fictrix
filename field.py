from collections.abc import Callable

import numpy as np


class Field:
    def __init__(self, function: Callable[[float, float], float]):
        self.function = function

    def __call__(self, x: float, y: float) -> float:
        return self.function(x, y)

    def sample(self, width: int, height: int) -> "Sample":
        return Sample(self, width, height)

    def normalize(self, width: int = 256, height: int = 256) -> "Field":
        sample = self.sample(width, height)
        min_val, max_val = sample.range()

        def normalized_function(x: float, y: float) -> float:
            return (self(x, y) - min_val) / (max_val - min_val)

        return Field(normalized_function)


class Sample:
    def __init__(self, field: Field, width: int, height: int):
        self.field = field
        self.width = width
        self.height = height
        self.array = np.zeros((height, width), dtype=float)

        for i in range(height):
            for j in range(width):
                x = j / (width - 1)
                y = i / (height - 1)
                self.array[i, j] = field(x, y)

    def range(self) -> tuple[float, float]:
        return np.min(self.array), np.max(self.array)

    def normalize(self) -> "Sample":
        min_val, max_val = self.range()

        normalized_sample = Sample.__new__(Sample)
        normalized_sample.field = self.field
        normalized_sample.width = self.width
        normalized_sample.height = self.height
        normalized_sample.array = (self.array - min_val) / (max_val - min_val)

        return normalized_sample


def simplex(seed: int = 0, octave: float = 1.0) -> Field:
    import opensimplex

    opensimplex.seed(seed)

    def noise_function(x: float, y: float) -> float:
        return opensimplex.noise2(x * octave, y * octave)

    return Field(noise_function)
