import matplotlib.pyplot as plt
import rawpy
import numpy as np

raw = rawpy.imread("IMGP7963.PEF")
rgb = raw.postprocess(rawpy.Params(use_camera_wb=True))

def fx(minval, maxval, M, row_or_col):

    if row_or_col != "row" and row_or_col != "col":
        print("check row or col")
        return
    
    print("creating contrast mask")
    L = np.float64(M)
    L /= 255
    rcount=0
    rows, cols, x = L.shape
    for i in range(rows):
        if rcount % 100 == 0: print(f"{rcount}/{rows}")
        for j in range(cols):
            val = (L[i,j].max() - L[i,j].min())/2
            if val > maxval or val < minval:
                L[i,j] = np.array((0,0,0))
            else:
                L[i,j] = np.array((1,1,1))
        rcount += 1
    
    plt.imshow(L)
    plt.show()

    print("copying original matrix")
    N = M.copy()
    
    print("applying contrast mask to copied matrix")
    N = np.multiply(N, L)

    
    if row_or_col == "row":
        for i in range(rows):
            N[i].sort()
    elif row_or_col == "col":
        N = N

    

    return M

def apply_con_mask(minval, maxval, M):
    image = fx(minval, maxval, M,"row")
    return np.int16(image)

out = apply_con_mask(0.12,0.99,rgb)
plt.imshow(out)
plt.show()