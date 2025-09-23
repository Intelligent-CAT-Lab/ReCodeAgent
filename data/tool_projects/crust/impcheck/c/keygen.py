
import random
import sys

"""
Generates a random 128-bit key and writes it into src/trusted/secret.c.
An arbitrary seed can be provided as the 1st argument.
"""

if len(sys.argv) > 1:
    print(f"Using seed {sys.argv[1]}")
    random.seed(sys.argv[1])

f = open("src/trusted/secret.c", 'w')
f.write("\n#include \"secret.h\"\n\nconst unsigned char SECRET_KEY[] = {\n")
for x in range(2):
    f.write("    ")
    for i in range(8):
        r = random.randint(0, 255)
        f.write(f"{r}, ")
    f.write("\n");
f.write("};\n")
