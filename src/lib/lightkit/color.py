import colorsys
import os
import struct
import subprocess
import tempfile
from PIL import Image
import numpy as np
import scipy
import scipy.misc
import scipy.cluster
import binascii
import colour
def get_dominant_color(image_path: str):
    NUM_CLUSTERS = 5
    if not os.path.exists(image_path):
        return (0, 0, 0)
    image = Image.open(image_path)
    image = image.resize((150, 150))
    ar = np.asarray(image)
    shape = ar.shape
    ar = ar.reshape(np.product(shape[:2]), shape[2]).astype(float)

    codes, dist = scipy.cluster.vq.kmeans(ar, NUM_CLUSTERS)

    vecs, dist = scipy.cluster.vq.vq(ar, codes)
    counts, bins = np.histogram(vecs, len(codes))

    index_max = np.argmax(counts)
    peak = codes[index_max]

    return (int(peak[0]), int(peak[1]), int(peak[2]))


def rgb_to_hsl(r: int, g: int, b: int):
    h, s, l = colorsys.rgb_to_hsv(r / 255, g / 255, b / 255)
    return (int(h * 360), int(s * 100), int(l * 100))




def gradient(start_color: tuple[int, int, int], end_color: tuple[int, int, int], steps: int):
    c1 = colour.Color()
    c1.hsl = (start_color[0], start_color[1] / 100, start_color[2] / 100)
    c2 = colour.Color()
    c2.hsl = (end_color[0], end_color[1] / 100, end_color[2] / 100)
    colors = []
    for c in list(c1.range_to(c2, steps)):
        h, s, l = c.hsl
        colors.append((int(h), int(s * 100), int(l * 100)))
    return colors
