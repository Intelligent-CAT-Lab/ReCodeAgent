
### SKEL HEAD BEGIN
def user_get_type(obj):
    if hasattr(obj, '_class_name'):
        return "<function " + obj._class_name.split(";")[0] + " >"
    else:
        return type(obj)

def user_check_type(obj, _type):
    if str(_type).startswith("<class") and str(_type).split("'")[1] in ["dict", "object"]:
        return isinstance(obj, _type)
    elif hasattr(obj, '_class_name'):
        if "function" in str(_type):
            for i in obj._class_name.split(";"):
                if i == str(_type).split(" ")[1]:
                    return True
            return False
    else:
        if str(_type).startswith("<function"):
            typename = str(_type).split(" ")[1]
            if typename == 'func_dict':
                return isinstance(obj, dict)
        return isinstance(obj, _type)


def SkelClass(class_name, super_class=None):
    if super_class is None:
        class myclass:
            _class_name = class_name
    else:
        class myclass(super_class):
            _class_name = class_name
    return myclass()

### SKEL HEAD END

import math

def rgb_to_yiq(r, g, b):
    ### --- BLOCK BEGIN 1
    y = 0.30*r + 0.59*g + 0.11*b
    i = 0.74*(r-y) - 0.27*(b-y)
    q = 0.48*(r-y) + 0.41*(b-y)
    return (y, i, q)
    ### --- BLOCK END 1



def yiq_to_rgb(y, i, q):
    ### --- BLOCK BEGIN 2
    r = y + 0.9468822170900693*i + 0.6235565819861433*q
    g = y - 0.27478764629897834*i - 0.6356910791873801*q
    b = y - 1.1085450346420322*i + 1.7090069284064666*q
    if r < 0.0:
        r = 0.0
    if g < 0.0:
        g = 0.0
    if b < 0.0:
        b = 0.0
    if r > 1.0:
        r = 1.0
    if g > 1.0:
        g = 1.0
    if b > 1.0:
        b = 1.0
    return [r, g, b]
    ### --- BLOCK END 2



def rgb_to_hls(r, g, b):
    ### --- BLOCK BEGIN 3
    maxc = max(r, g, b)
    minc = min(r, g, b)
    sumc = (maxc+minc)
    rangec = (maxc-minc)
    l = sumc/2.0
    if minc == maxc:
        return 0.0, l, 0.0
    if l <= 0.5:
        s = rangec / sumc
    else:
        s = rangec / (2.0-sumc)
    rc = (maxc-r) / rangec
    gc = (maxc-g) / rangec
    bc = (maxc-b) / rangec
    if r == maxc:
        h = bc-gc
    elif g == maxc:
        h = 2.0+rc-bc
    else:
        h = 4.0+gc-rc
    h = (h/6.0) % 1.0
    return h, l, s
    ### --- BLOCK END 3



def hls_to_rgb(h, l, s):
    ### --- BLOCK BEGIN 4
    if s == 0.0:
        return l, l, l
    if l <= 0.5:
        m2 = l * (1.0+s)
    else:
        m2 = l+s-(l*s)
    m1 = 2.0*l - m2
    tmp_1 = _v(m1, m2, h+ONE_THIRD)
    tmp_2 = _v(m1, m2, h)
    tmp_3 = _v(m1, m2, h-ONE_THIRD)
    return [tmp_1, tmp_2, tmp_3]
    ### --- BLOCK END 4



def _v(m1, m2, hue):
    ### --- BLOCK BEGIN 5
    hue = hue % 1.0
    if hue < ONE_SIXTH:
        return m1 + (m2-m1)*hue*6.0
    if hue < 0.5:
        return m2
    if hue < TWO_THIRD:
        return m1 + (m2-m1)*(TWO_THIRD-hue)*6.0
    return m1
    ### --- BLOCK END 5



def rgb_to_hsv(r, g, b):
    ### --- BLOCK BEGIN 6
    maxc = max(r, g, b)
    minc = min(r, g, b)
    rangec = (maxc-minc)
    v = maxc
    if minc == maxc:
        return 0.0, 0.0, v
    s = rangec / maxc
    rc = (maxc-r) / rangec
    gc = (maxc-g) / rangec
    bc = (maxc-b) / rangec
    if r == maxc:
        h = bc-gc
    elif g == maxc:
        h = 2.0+rc-bc
    else:
        h = 4.0+gc-rc
    h = (h/6.0) % 1.0
    return h, s, v
    ### --- BLOCK END 6



def hsv_to_rgb(h, s, v):
    ### --- BLOCK BEGIN 7
    if s == 0.0:
        return v, v, v
    i = int(h*6.0)
    f = (h*6.0) - i
    p = v*(1.0 - s)
    q = v*(1.0 - s*f)
    t = v*(1.0 - s*(1.0-f))
    i = i%6
    if i == 0:
        return v, t, p
    if i == 1:
        return q, v, p
    if i == 2:
        return p, v, t
    if i == 3:
        return p, q, v
    if i == 4:
        return t, p, v
    if i == 5:
        return v, p, q
    # Cannot get here
    ### --- BLOCK END 7



def user_assert_almost_equal(a, b):
    ### --- BLOCK BEGIN 8
    assert abs(a-b) <= 0.0001
    return True
    ### --- BLOCK END 8



def assert_iter_almost_equal(iter1, iter2):
    ### --- BLOCK BEGIN 9
    for a, b in zip(iter1, iter2):
        user_assert_almost_equal(a, b)
    return True
    ### --- BLOCK END 9



def test_assertions():
    ### --- BLOCK BEGIN 10
    print("--- rgb_to_yiq ---")
    tmp = rgb_to_yiq(0.5, 0.5, 0.5)
    const =  [0.49999999999999994, 2.6090241078691177e-17, 4.940492459581946e-17]
    assert_iter_almost_equal(tmp, const)
    tmp = rgb_to_yiq(0, 0.5, 1)
    const =  [0.40499999999999997, -0.46035, 0.04954999999999998]
    assert_iter_almost_equal(tmp, const)
    tmp = rgb_to_yiq(1, 0, 0)
    const =  [0.3, 0.599, 0.21299999999999997]
    assert_iter_almost_equal(tmp, const)
    tmp = rgb_to_yiq(0, 0, 0)
    const =  [0.0, 0.0, 0.0]
    assert_iter_almost_equal(tmp, const)
    tmp = rgb_to_yiq(1, 0.1, 0.3)
    const =  [0.392, 0.47476, 0.25411999999999996]
    assert_iter_almost_equal(tmp, const)
    print("--- yiq_to_rgb ---")
    tmp = yiq_to_rgb(1.0, 0.5957, 0.0)
    const =  [1.0, 0.8363089990996986, 0.33963972286374133]
    assert_iter_almost_equal(tmp, const)
    tmp = yiq_to_rgb(0.0, -0.5957, -0.5226)
    const =  [0.0, 0.49590315888362624, 0.0]
    assert_iter_almost_equal(tmp, const)
    tmp = yiq_to_rgb(0.8, 0.1, 0.2)
    const =  [1.0, 0.6453830195326262, 1.0]
    assert_iter_almost_equal(tmp, const)
    tmp = yiq_to_rgb(0.0, 0.0, 0.0)
    const =  [0.0, 0.0, 0.0]
    assert_iter_almost_equal(tmp, const)
    tmp = yiq_to_rgb(1.0, 0.0, 0.0)
    const =  [1.0, 1.0, 1.0]
    assert_iter_almost_equal(tmp, const)
    tmp = yiq_to_rgb(0.5, 0.0, 0.0)
    const =  [0.5, 0.5, 0.5]
    assert_iter_almost_equal(tmp, const)
    print("--- rgb_to_hls ---")
    tmp = rgb_to_hls(0.5, 0.5, 0.5)
    const = [0.0, 0.5, 0.0]
    assert_iter_almost_equal(tmp, const)
    tmp = rgb_to_hls(0, 0.5, 1)
    const = [0.5833333333333334, 0.5, 1.0]
    assert_iter_almost_equal(tmp, const)
    tmp = rgb_to_hls(1, 0, 0)
    const = [0.0, 0.5, 1.0]
    assert_iter_almost_equal(tmp, const)
    tmp = rgb_to_hls(0, 0, 0)
    const = [0.0, 0.0, 0.0]
    assert_iter_almost_equal(tmp, const)
    tmp = rgb_to_hls(1, 0.1, 0.3)
    const = [0.9629629629629629, 0.55, 1.0000000000000002]
    assert_iter_almost_equal(tmp, const)
    print("--- hls_to_rgb ---")
    tmp = hls_to_rgb(0.5, 0.5, 0.5)
    const = [0.25, 0.7499999999999999, 0.75]
    assert_iter_almost_equal(tmp, const)
    tmp = hls_to_rgb(0, 0.5, 1)
    const = [1.0, 0.0, 0.0]
    assert_iter_almost_equal(tmp, const)
    tmp = hls_to_rgb(1, 0, 0)
    const = [0, 0, 0]
    assert_iter_almost_equal(tmp, const)
    tmp = hls_to_rgb(0, 0, 0)
    const = [0, 0, 0]
    assert_iter_almost_equal(tmp, const)
    tmp = hls_to_rgb(1, 0.1, 0.3)
    const = [0.13, 0.07, 0.07]
    assert_iter_almost_equal(tmp, const)
    print("--- rgb_to_hsv ---")
    tmp = rgb_to_hsv(0.5, 0.5, 0.5)
    const = [0.0, 0.0, 0.5]
    assert_iter_almost_equal(tmp, const)
    tmp = rgb_to_hsv(0, 0.5, 1)
    const = [0.5833333333333334, 1.0, 1]
    assert_iter_almost_equal(tmp, const)
    tmp = rgb_to_hsv(1, 0, 0)
    const = [0.0, 1.0, 1]
    assert_iter_almost_equal(tmp, const)
    tmp = rgb_to_hsv(0, 0, 0)
    const = [0.0, 0.0, 0]
    assert_iter_almost_equal(tmp, const)
    tmp = rgb_to_hsv(1, 0.1, 0.3)
    const = [0.9629629629629629, 0.9, 1]
    assert_iter_almost_equal(tmp, const)
    print("--- hsv_to_rgb ---")
    tmp = hsv_to_rgb(0.5, 0.5, 0.5)
    const = [0.25, 0.5, 0.5]
    assert_iter_almost_equal(tmp, const)
    tmp = hsv_to_rgb(0, 0.5, 1)
    const = [1, 0.5, 0.5]
    assert_iter_almost_equal(tmp, const)
    tmp = hsv_to_rgb(1, 0, 0)
    const = [0, 0, 0]
    assert_iter_almost_equal(tmp, const)
    tmp = hsv_to_rgb(0, 0, 0)
    const = [0, 0, 0]
    assert_iter_almost_equal(tmp, const)
    tmp = hsv_to_rgb(1, 0.1, 0.3)
    const = [0.3, 0.27, 0.27]
    assert_iter_almost_equal(tmp, const)
    ### --- BLOCK END 10



def test():
    ### --- BLOCK BEGIN 11
    test_assertions()
    additional_tests()
    ### --- BLOCK END 11



def additional_tests():
    ### --- BLOCK BEGIN 12
    tmp = yiq_to_rgb(0.0, 1.0, 0.3)
    const = [1.0, 0.0, 0.0]
    assert_iter_almost_equal(tmp, const)
    tmp = yiq_to_rgb(2.0, 0.0, 0.0)
    const = [1.0, 1.0, 1.0]
    assert_iter_almost_equal(tmp, const)
    tmp = rgb_to_hls(0.5, 1.5, 0.2)
    const = [0.2948717948717949, 0.85, 4.333333333333333]
    assert_iter_almost_equal(tmp, const)
    tmp = hls_to_rgb(0.5, 0.6, 0.2)
    const = [0.5199999999999999, 0.68, 0.68]
    assert_iter_almost_equal(tmp, const)
    tmp = rgb_to_hsv(0.5, 1.5, 0.2)
    const = [0.2948717948717949, 0.8666666666666666, 1.5]
    assert_iter_almost_equal(tmp, const)
    tmp = hsv_to_rgb(0.2, 0.6, 0.2)
    const = [0.176, 0.2, 0.08]
    assert_iter_almost_equal(tmp, const)
    tmp = hsv_to_rgb(0.4, 0.6, 0.2)
    const = [0.08, 0.2, 0.128]
    assert_iter_almost_equal(tmp, const)
    tmp = hsv_to_rgb(0.7, 0.6, 0.2)
    const = [0.10399999999999993, 0.08000000000000002, 0.2]
    assert_iter_almost_equal(tmp, const)
    tmp = hsv_to_rgb(0.9, 0.6, 0.2)
    const = [0.2, 0.08000000000000002, 0.15199999999999997]
    assert_iter_almost_equal(tmp, const)
    ### --- BLOCK END 12



### Global Begin

### --- BLOCK BEGIN 0
ONE_THIRD = 1.0/3.0

ONE_SIXTH = 1.0/6.0

TWO_THIRD = 2.0/3.0

test()

### --- BLOCK END 0
