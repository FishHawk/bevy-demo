import numpy as np
import imageio.v3 as iio


def load_img(path):
    img = iio.imread(path)
    if img.shape[2] == 3:
        img = np.pad(
            img, ((0, 0), (0, 0), (0, 1)), "constant", constant_values=(0, 255)
        )
    return img


def generate_lut(img):
    shape = img.shape

    img = img.reshape(shape[0] * shape[1], 8)
    lut, indice = np.unique(img, axis=0, return_inverse=True)

    # Padding a transprent pixel. For outbound pixel.
    lut = np.vstack((np.zeros((1, 8)), lut))
    indice += 1

    lut = lut.reshape(lut.shape[0], 2, 4).astype(np.uint8)
    indice = indice.reshape(shape[0], shape[1]).astype(np.uint16)
    return lut, indice


def generate_lut_from_layers(layers_groups):
    groups = [np.vstack(layers) for layers in layers_groups]
    img = np.dstack((groups))
    lut, indice = generate_lut(img)
    return lut, np.split(indice, len(layers_groups[0]))


layers_day = [load_img(f"assets/demo/day/{i}.png") for i in range(1, 7)]
layers_night = [load_img(f"assets/demo/night/{i}.png") for i in range(1, 7)]

lut, indices = generate_lut_from_layers([layers_day, layers_night])

iio.imwrite("assets/demo/lut.png", lut)
for i, img in enumerate(indices):
    iio.imwrite(f"assets/demo/{i+1}.png", img)
