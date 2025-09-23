# CSyncmers

Closed syncmers in pure C. Supports arbitrary k values and s < 64 (using `uint128_t`). s-mers are selected according to lexicographic order. Achieves around 60 MB/sec throughput.




### Some of existing code

Strobealign code: uses 64-bit smers 
https://github.com/ksahlin/strobealign/blob/71866c31b578e5166c83aaf1fde79d238246490d/src/randstrobes.cpp#L57

Minimap2 code: uses 64-bit smers
https://github.com/lh3/minimap2/blob/c2f07ff2ac8bdc5c6768e63191e614ea9012bd5d/sketch.c#L145-L192

Curiouscoding blog:
https://curiouscoding.nl/posts/fast-minimizers/

Sliding window minimum algorithm explanation:
https://github.com/keegancsmith/Sliding-Window-Minimum?tab=readme-ov-file#sliding-window-minimum-algorithm
