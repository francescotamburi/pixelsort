import rawpy
import numpy as np

raw = rawpy.imread("IMGP7751.PEF")
rgb = raw.postprocess(rawpy.Params(use_camera_wb=True))
f = open("raw_img.np", "w")
print(rgb.dtype)
rgb.tofile(f)
print(rgb.shape)
#f.write(f"\n\n{rgb.size}")