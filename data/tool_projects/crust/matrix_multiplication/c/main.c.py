import numpy as np
import time

start = time.time()

a = np.ones((128, 128))
for i in range(128):
    for j in range(128):
        a[i][j] = i * 128 + j
for times in range(1000):
    b = a @ a
end = time.time()
print((end - start) * 1000)
