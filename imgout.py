import matplotlib.pyplot as plt
import numpy as np

f = open("processed_img.np", "r")
pixelarray = np.fromfile(f, dtype="uint8")
print(len(pixelarray))
rgb = np.reshape(pixelarray, (2868,4309,3))

plt.imshow(rgb)
plt.show()