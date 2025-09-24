
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

def heappush(heap, item):
    ### --- BLOCK BEGIN 1
    heap.append(item)
    _siftdown(heap, 0, len(heap)-1)
    ### --- BLOCK END 1



def heappop(heap):
    ### --- BLOCK BEGIN 2
    lastelt = heap.pop()
    if heap:
        returnitem = heap[0]
        heap[0] = lastelt
        _siftup(heap, 0)
        return returnitem
    return lastelt
    ### --- BLOCK END 2



def heapreplace(heap, item):
    ### --- BLOCK BEGIN 3
    returnitem = heap[0]
    heap[0] = item
    _siftup(heap, 0)
    return returnitem
    ### --- BLOCK END 3



def heappushpop(heap, item):
    ### --- BLOCK BEGIN 4
    if heap and heap[0] < item:
        item, heap[0] = heap[0], item
        _siftup(heap, 0)
    return item
    ### --- BLOCK END 4



def heapify(x):
    ### --- BLOCK BEGIN 5
    n = len(x)
    for i in reversed(range(n//2)):
        _siftup(x, i)
    ### --- BLOCK END 5



def _heappop_max(heap):
    ### --- BLOCK BEGIN 6
    lastelt = heap.pop()
    if heap:
        returnitem = heap[0]
        heap[0] = lastelt
        _siftup_max(heap, 0)
        return returnitem
    return lastelt
    ### --- BLOCK END 6



def _heapreplace_max(heap, item):
    ### --- BLOCK BEGIN 7
    returnitem = heap[0]
    heap[0] = item
    _siftup_max(heap, 0)
    return returnitem
    ### --- BLOCK END 7



def _heapify_max(x):
    ### --- BLOCK BEGIN 8
    n = len(x)
    for i in reversed(range(n//2)):
        _siftup_max(x, i)
    ### --- BLOCK END 8



def _siftdown(heap, startpos, pos):
    ### --- BLOCK BEGIN 9
    newitem = heap[pos]
    while pos > startpos:
        parentpos = (pos - 1) >> 1
        parent = heap[parentpos]
        if newitem < parent:
            heap[pos] = parent
            pos = parentpos
            continue
        break
    heap[pos] = newitem
    ### --- BLOCK END 9



def _siftup(heap, pos):
    ### --- BLOCK BEGIN 10
    endpos = len(heap)
    startpos = pos
    newitem = heap[pos]
    childpos = 2*pos + 1
    while childpos < endpos:
        rightpos = childpos + 1
        if rightpos < endpos and not heap[childpos] < heap[rightpos]:
            childpos = rightpos
        heap[pos] = heap[childpos]
        pos = childpos
        childpos = 2*pos + 1
    heap[pos] = newitem
    _siftdown(heap, startpos, pos)
    ### --- BLOCK END 10



def _siftdown_max(heap, startpos, pos):
    ### --- BLOCK BEGIN 11
    newitem = heap[pos]
    while pos > startpos:
        parentpos = (pos - 1) >> 1
        parent = heap[parentpos]
        if parent < newitem:
            heap[pos] = parent
            pos = parentpos
            continue
        break
    heap[pos] = newitem
    ### --- BLOCK END 11



def _siftup_max(heap, pos):
    ### --- BLOCK BEGIN 12
    endpos = len(heap)
    startpos = pos
    newitem = heap[pos]
    childpos = 2*pos + 1
    while childpos < endpos:
        rightpos = childpos + 1
        if rightpos < endpos and not heap[rightpos] < heap[childpos]:
            childpos = rightpos
        heap[pos] = heap[childpos]
        pos = childpos
        childpos = 2*pos + 1
    heap[pos] = newitem
    _siftdown_max(heap, startpos, pos)
    ### --- BLOCK END 12



def merge(reverse, *iterables):
    def h_append(x):
        ### --- BLOCK BEGIN 13
        h.append(x)
        ### --- BLOCK END 13
    
    
    
    ### --- BLOCK BEGIN 14
    h = []
    if reverse:
        _heapify = _heapify_max
        _heappop = _heappop_max
        _heapreplace = _heapreplace_max
        direction = -1
    else:
        _heapify = heapify
        _heappop = heappop
        _heapreplace = heapreplace
        direction = 1
    for order, it in enumerate(map(iter, iterables)):
        try:
            next = it.__next__
            h_append([next(), order * direction, next])
        except StopIteration:
            pass
    _heapify(h)
    while len(h) > 1:
        try:
            while True:
                value, order, next = s = h[0]
                yield value
                s[0] = next()
                _heapreplace(h, s)
        except StopIteration:
            _heappop(h)
    if h:
        value, order, next = h[0]
        yield value
        yield from next.__self__
    return
    ### --- BLOCK END 14



def nsmallest(n, iterable):
    ### --- BLOCK BEGIN 15
    if n == 1:
        it = iter(iterable)
        sentinel = object()
        result = min(it, default=sentinel)
        return [] if result is sentinel else [result]
    try:
        size = len(iterable)
    except (TypeError, AttributeError):
        pass
    else:
        if n >= size:
            return sorted(iterable)[:n]
    it = iter(iterable)
    result = [(elem, i) for i, elem in zip(range(n), it)]
    if not result:
        return result
    _heapify_max(result)
    top = result[0][0]
    order = n
    _heapreplace = _heapreplace_max
    for elem in it:
        if elem < top:
            _heapreplace(result, (elem, order))
            top, _order = result[0]
            order += 1
    result.sort()
    return [elem for (elem, order) in result]
    ### --- BLOCK END 15



def nlargest(n, iterable):
    ### --- BLOCK BEGIN 16
    if n == 1:
        it = iter(iterable)
        sentinel = object()
        result = max(it, default=sentinel)
        return [] if result is sentinel else [result]
    try:
        size = len(iterable)
    except (TypeError, AttributeError):
        pass
    else:
        if n >= size:
            return sorted(iterable, reverse=True)[:n]
    it = iter(iterable)
    result = [(elem, i) for i, elem in zip(range(0, -n, -1), it)]
    if not result:
        return result
    heapify(result)
    top = result[0][0]
    order = -n
    _heapreplace = heapreplace
    for elem in it:
        if top < elem:
            _heapreplace(result, (elem, order))
            top = result[0][0]
            _order = result[0][1]
            order -= 1
    result.sort(reverse=True)
    return [elem for (elem, order) in result]
    ### --- BLOCK END 16



def assert_value_equal(a, b):
    ### --- BLOCK BEGIN 17
    assert abs(a - b) <= 0.0001
    ### --- BLOCK END 17



def assert_equal(a, b):
    ### --- BLOCK BEGIN 18
    for i, j in zip(a, b):
        assert_value_equal(i, j)
    ### --- BLOCK END 18



def test_heappush_help_function(items):
    ### --- BLOCK BEGIN 19
    heap = []
    for item in items:
        heappush(heap, item)
    a = heappop(heap)
    b = heappop(heap)
    return [a, b]
    ### --- BLOCK END 19



def test_heappush():
    ### --- BLOCK BEGIN 20
    tmp = test_heappush_help_function([6,1,-2,5])
    assert_equal(tmp, [-2,1])
    tmp = test_heappush_help_function([34,-3,-12,0])
    assert_equal(tmp, [-12,-3])
    tmp = test_heappush_help_function([5,4,3,2,1])
    assert_equal(tmp, [1,2])
    tmp = test_heappush_help_function([4.7,8,-1.2,7.2])
    assert_equal(tmp, [-1.2,4.7])
    ### --- BLOCK END 20



def test_heapify_help_function(x):
    ### --- BLOCK BEGIN 21
    heapify(x)
    a = heappop(x)
    b = heappop(x)
    return [a, b]
    ### --- BLOCK END 21



def test_heapify():
    ### --- BLOCK BEGIN 22
    tmp = test_heapify_help_function([6,1,-2,5])
    assert_equal(tmp, [-2,1])
    tmp = test_heapify_help_function([34,-3,-12,0])
    assert_equal(tmp, [-12,-3])
    tmp = test_heapify_help_function([5,4,3,2,1])
    assert_equal(tmp, [1,2])
    tmp = test_heapify_help_function([4.7,8,-1.2,7.2])
    assert_equal(tmp, [-1.2,4.7])
    ### --- BLOCK END 22



def test_heappushpop_help_function(x, i):
    ### --- BLOCK BEGIN 23
    heapify(x)
    a = heappushpop(x, i)
    return a
    ### --- BLOCK END 23



def test_heappushpop():
    ### --- BLOCK BEGIN 24
    tmp = test_heappushpop_help_function([6,1,-2,5], -5)
    assert_value_equal(tmp, -5)
    tmp = test_heappushpop_help_function([34,-3,-12,0], -13)
    assert_value_equal(tmp, -13)
    tmp = test_heappushpop_help_function([5,4,3,2,1], 0)
    assert_value_equal(tmp, 0)
    tmp = test_heappushpop_help_function([4.7,8,-1.2,7.2], 9)
    assert_value_equal(tmp, -1.2)
    ### --- BLOCK END 24



def test_heapreplace_help_function(x, i):
    ### --- BLOCK BEGIN 25
    heapify(x)
    a = heapreplace(x, i)
    b = heappop(x)
    return [a,b]
    ### --- BLOCK END 25



def test_heapreplace():
    ### --- BLOCK BEGIN 26
    tmp = test_heapreplace_help_function([6,1,-2,5], -5)
    assert_equal(tmp, [-2,-5])
    tmp = test_heapreplace_help_function([34,-3,-12,0], -13)
    assert_equal(tmp, [-12,-13])
    tmp = test_heapreplace_help_function([5,4,3,2,1], 0)
    assert_equal(tmp, [1,0])
    tmp = test_heapreplace_help_function([4.7,8,-1.2,7.2], 9)
    assert_equal(tmp, [-1.2,4.7])
    ### --- BLOCK END 26



def test_merge():
    ### --- BLOCK BEGIN 27
    tmp = list(merge(False, [1,3,5,7], [0,2,4,8], [5,10,15,20], [], [25]))
    assert_equal(tmp, [0, 1, 2, 3, 4, 5, 5, 7, 8, 10, 15, 20, 25])
    tmp = list(merge(True, [7,5,3,1], [8,4,2,0]))
    assert_equal(tmp, [8, 7, 5, 4, 3, 2, 1, 0])
    ### --- BLOCK END 27



def test_nsmallest():
    ### --- BLOCK BEGIN 28
    tmp = nsmallest(1, [6,1,-2,5])
    assert_equal(tmp, [-2])
    tmp = nsmallest(2, [34,-3,-12,0])
    assert_equal(tmp, [-12,-3])
    tmp = nsmallest(2, [5,4,3,2,1])
    assert_equal(tmp, [1,2])
    tmp = nsmallest(2, [4.7,8,-1.2,7.2])
    assert_equal(tmp, [-1.2,4.7])
    ### --- BLOCK END 28



def test_nlargest():
    ### --- BLOCK BEGIN 29
    tmp = nlargest(1, [6,1,-2,5])
    assert_equal(tmp, [6])
    tmp = nlargest(2, [34,-3,-12,0])
    assert_equal(tmp, [34,0])
    tmp = nlargest(2, [5,4,3,2,1])
    assert_equal(tmp, [5,4])
    tmp = nlargest(2, [4.7,8,-1.2,7.2])
    assert_equal(tmp, [8,7.2])
    ### --- BLOCK END 29



def test():
    ### --- BLOCK BEGIN 30
    test_heappush()
    test_heapify()
    test_heappushpop()
    test_heapreplace()
    test_nsmallest()
    test_nlargest()
    test_merge()
    additional_tests()
    ### --- BLOCK END 30



def additional_tests():
    ### --- BLOCK BEGIN 31
    heap = [1]
    tmp = heappop(heap)
    assert_value_equal(tmp, 1)
    heap = [1]
    tmp = _heappop_max(heap)
    assert_value_equal(tmp, 1)
    tmp = nsmallest(0, [1])
    assert_equal(tmp, [])
    tmp = nsmallest(2, [])
    assert_equal(tmp, [])
    tmp = nsmallest(0, zip([], [1]))
    assert_equal(tmp, [])
    tmp = nlargest(0, [1])
    assert_equal(tmp, [])
    tmp = nlargest(2, [])
    assert_equal(tmp, [])
    tmp = nlargest(0, zip([], [1]))
    assert_equal(tmp, [])
    heap = [1,2,3]
    tmp = _siftup_max(heap, 0)
    assert_equal(heap, [3,2,1])
    ### --- BLOCK END 31



### Global Begin

### --- BLOCK BEGIN 0
test()

### --- BLOCK END 0
