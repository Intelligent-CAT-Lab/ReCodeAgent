import hashlib
import math
seed = 1
def user_hash_random():
    global seed
    h = hashlib.sha256()
    h.update(str(seed).encode('utf-8'))
    seed += 1
    return int(h.hexdigest(), 16) / (2 ** 256 - 1)

def user_randint(min, max):
    return math.floor(user_hash_random() * (max - min + 1)) + min

def user_choice_func1(x):
    if user_hash_random() < 0.5:
        return x[0]
    # else:
    #     return x[1]

def user_choice_func2(x):
    return x[math.floor(user_hash_random() * len(x))]

def user_sample_func1(x, n):
    # This sample function works assuming x is in the form of range(a, b)
    a = x[0]
    b = x[len(x)-1] + 1

    lst = []
    for i in range(n):
        lst.append(math.floor(user_hash_random() * (b - a + 1)) + a)
    return lst

def user_sample_func2(x, n):
    x = list(x)
    lst = []
    for _ in range(n):
        index = math.floor(user_hash_random() * len(x))
        lst.append(x[index])
        del x[index]
    return lst

def user_uniform(a, b):
    return user_hash_random() * (b - a) + a

def user_reset_seed():
    global seed
    seed = 1

