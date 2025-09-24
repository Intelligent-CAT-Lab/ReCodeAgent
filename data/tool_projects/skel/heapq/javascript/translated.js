
/// SKEL HEAD BEGIN
function user_check_type(obj, _type) {
    if (typeof obj === 'object' && !Array.isArray(obj) && obj !== null && obj.hasOwnProperty("_class_name")) {
        if (String(_type).includes('function')) {
            for (let i of obj["_class_name"].split(";")) {
                if (i === String(_type).split(" ")[1].split("(")[0]) {
                    return true;
                }
            }
            return false;
        } else if (typeof _type === 'string') {
            for (let i of obj["_class_name"].split(";")) {
                if (i === _type) {
                    return true;
                }
            }
            return false;
        }
        else{
            return false;
        }
    } else {
        if (typeof _type === 'symbol') {
            if (_type.description === 'str' || _type.description === 'string') {
                return typeof obj === 'string';
            }
            if (_type.description === 'list' || _type.description === 'array') {
                return Array.isArray(obj);
            }
            if (_type.description === 'dict') {
                return obj.constructor === Object;
            }
            if (_type.description === 'int' || _type.description === 'number') {
                return Number.isSafeInteger(obj)  && obj !== 1e6;;
            }
            if (_type.description === 'float') {
                return typeof obj === 'number';
            }
            if (_type.description === 'bool' || _type.description === 'boolean') {
                return typeof obj === 'boolean';
            }
            if (_type.description === 'datetime') {
                return obj instanceof Date;
            }
            if (_type.description === 'time') {
                return obj instanceof Date && obj.getFullYear() === 1970 && obj.getMonth() === 0 && obj.getDate() === 1;
            }
            if (_type.description === 'date') {
                return obj instanceof Date && obj.getHours() === 0 && obj.getMinutes() === 0 && obj.getSeconds() === 0;
            }
            return false;
        }

        if (typeof _type === 'string') {
            if (_type === 'str' || _type === 'string') {
                return typeof obj === 'string';
            }
            if (_type === 'list' || _type === 'array') {
                return Array.isArray(obj);
            }
            if (_type === 'dict') {
                return obj.constructor === Object;
            }
            if (_type === 'int' || _type === 'number') {
                return Number.isSafeInteger(obj) && obj !== 1e6;
            }
            if (_type === 'float') {
                return typeof obj === 'number';
            }
            if (_type === 'bool' || _type === 'boolean') {
                return typeof obj === 'boolean';
            }
            if (_type === 'datetime') {
                return obj instanceof Date;
            }
            if (_type === 'time') {
                return obj instanceof Date && obj.getFullYear() === 1970 && obj.getMonth() === 0 && obj.getDate() === 1;
            }
            if (_type === 'date') {
                return obj instanceof Date && obj.getHours() === 0 && obj.getMinutes() === 0 && obj.getSeconds() === 0;
            }
            return false;
        }
        else return obj instanceof _type;
    }
}


function SkelClass(name) {
    let _class_var = {};
    _class_var._class_name = name;
    return _class_var;
}

/// SKEL HEAD END

function heappush(heap, item){
    /// --- BLOCK BEGIN 1
heap.push(item);
_siftdown(heap, 0, heap.length - 1);
    /// --- BLOCK END 1

}

function heappop(heap){
    /// --- BLOCK BEGIN 2
var lastelt = heap.pop();
    if (heap.length > 0) {
        var returnitem = heap[0];
        heap[0] = lastelt;
        _siftup(heap, 0);
        return returnitem;
    }
    return lastelt;
    /// --- BLOCK END 2

}

function heapreplace(heap, item){
    /// --- BLOCK BEGIN 3
var returnitem = heap[0];
    heap[0] = item;
    _siftup(heap, 0);
    return returnitem;
    /// --- BLOCK END 3

}

function heappushpop(heap, item){
    /// --- BLOCK BEGIN 4
    if (heap.length > 0 && heap[0] < item) {
        var temp = item;
        item = heap[0];
        heap[0] = temp;
        _siftup(heap, 0);
    }
    return item;
    /// --- BLOCK END 4

}

function heapify(x){
    /// --- BLOCK BEGIN 5
var n = x.length;
    for (var i = Math.floor(n / 2) - 1; i >= 0; i--) {
        _siftup(x, i);
    }
    /// --- BLOCK END 5

}

function _heappop_max(heap){
    /// --- BLOCK BEGIN 6
var lastelt = heap.pop();
    if (heap.length > 0) {
        var returnitem = heap[0];
        heap[0] = lastelt;
        _siftup_max(heap, 0);
        return returnitem;
    }
    return lastelt;
    /// --- BLOCK END 6

}

function _heapreplace_max(heap, item){
    /// --- BLOCK BEGIN 7
var returnitem = heap[0];
    heap[0] = item;
    _siftup_max(heap, 0);
    return returnitem;
    /// --- BLOCK END 7

}

function _heapify_max(x){
    /// --- BLOCK BEGIN 8
var n = x.length;
    for (var i = Math.floor(n / 2) - 1; i >= 0; i--) {
        _siftup_max(x, i);
    }
    /// --- BLOCK END 8

}

function _siftdown(heap, startpos, pos){
    /// --- BLOCK BEGIN 9
    var compare = require('./tracer_skip.js');
    var newitem = heap[pos];
    while (pos > startpos) {
        var parentpos = (pos - 1) >> 1;
        var parent = heap[parentpos];
        if (compare(parent, newitem) > 0) {
            heap[pos] = parent;
            pos = parentpos;
            continue;
        }
        break;
    }
    heap[pos] = newitem;
    /// --- BLOCK END 9

}

function _siftup(heap, pos){
    /// --- BLOCK BEGIN 10
    var compare = require('./tracer_skip.js');
    var endpos = heap.length;
    var startpos = pos;
    var newitem = heap[pos];
    var childpos = 2 * pos + 1;
    while (childpos < endpos) {
        var rightpos = childpos + 1;
        if (rightpos < endpos && !(compare(heap[childpos], heap[rightpos]) < 0)) {
            childpos = rightpos;
        }
        
        heap[pos] = heap[childpos];
        pos = childpos;
        childpos = 2 * pos + 1;
    }
    heap[pos] = newitem;
    _siftdown(heap, startpos, pos);
    /// --- BLOCK END 10

}

function _siftdown_max(heap, startpos, pos){
    /// --- BLOCK BEGIN 11
var newitem = heap[pos];
    while (pos > startpos) {
        var parentpos = (pos - 1) >> 1;
        var parent = heap[parentpos];
        if (parent[0] < newitem[0]) {
            heap[pos] = parent;
            pos = parentpos;
            continue;
        }
        break;
    }
    heap[pos] = newitem;
    /// --- BLOCK END 11

}

function _siftup_max(heap, pos){
    /// --- BLOCK BEGIN 12
var endpos = heap.length;
    var startpos = pos;
    var newitem = heap[pos];
    var childpos = 2 * pos + 1;
    while (childpos < endpos) {
        var rightpos = childpos + 1;
        if (rightpos < endpos && !(heap[rightpos] < heap[childpos])) {
            childpos = rightpos;
        }
        heap[pos] = heap[childpos];
        pos = childpos;
        childpos = 2 * pos + 1;
    }
    heap[pos] = newitem;
    _siftdown_max(heap, startpos, pos);

    /// --- BLOCK END 12

}

function* merge(reverse, ...iterables){
    function h_append(x){
        /// --- BLOCK BEGIN 13
h.push(x);    
        /// --- BLOCK END 13
    
    }
    
    /// --- BLOCK BEGIN 14
var h = [];
    if (reverse) {
        _heapify = _heapify_max;
        _heappop = _heappop_max;
        _heapreplace = _heapreplace_max;
        direction = -1;
    } else {
        _heapify = heapify;
        _heappop = heappop;
        _heapreplace = heapreplace;
        direction = 1;
    }
    for (var order = 0; order < iterables.length; order++) {
        var it = iterables[order][Symbol.iterator]();
        try {
            var next = it.next.bind(it);
            var next_elem = next();
            if (next_elem.done) continue;
            h_append([next_elem.value, order * direction, next]);
        } catch (e) {
            // pass
            throw e;
        }
    }
    order -= 1;
    _heapify(h);
    while (h.length > 1) {
        try {
            while (true) {
                var s = h[0];
                var [value, order, next] = s;
                yield value;
                var next_elem = next();
                var done = next_elem.done;
                if (done) {
                    _heappop(h);
                    break;
                }
                s[0] = next_elem.value;
                _heapreplace(h, s);
            }
        } catch (e) {
            throw e;
        }
    }
    if (h.length > 0) {
        var [value, order, next] = h[0];
        yield value;
        yield* (function* next_wrap() {
            while(true) {
                var next_elem = next(); 
                var val = next_elem.value;
                var done = next_elem.done;
                if (done) {
                    break;
                }
                yield val;
            };
        })();
    }
    return;
    /// --- BLOCK END 14

}

function nsmallest(n, iterable){
    /// --- BLOCK BEGIN 15
if (n === 1) {
    var it = iterable[Symbol.iterator]();
    var sentinel = {};
    var result = Math.min(...it);
    return result === Infinity ? [] : [result];
}
try {
    var size = iterable.length;
} catch (e) {
    // pass
}
if (size !== undefined) {
    if (n >= size) {
        return iterable.slice().sort().slice(0, n);
    }
}
it = iterable[Symbol.iterator]();
result = [];
for (var i = 0; i < n; i++) {
    var elem = it.next().value;
    if (elem !== undefined) {
        result.push([elem, i]);
    }
}
if (!result.length) {
    return result;
}
_heapify_max(result);
var top = result[0][0];
var order = n;
var _heapreplace = _heapreplace_max;
for (elem of it) {
    if (elem < top) {
        _heapreplace(result, [elem, order]);
        top = result[0][0];
        order++;
    }
}
result.sort((a, b) => a[0] - b[0]);
return result.map(function (x) { return x[0]; });
    /// --- BLOCK END 15

}

function nlargest(n, iterable){
    /// --- BLOCK BEGIN 16
if (n === 1) {
    var it = iterable[Symbol.iterator]();
    var sentinel = {};
    var result = Math.max(...iterable);
    return result === -Infinity ? [] : [result];
}
try {
    var size = iterable.length;
} catch (e) {
    // pass
}
if (size !== undefined) {
    if (n >= size) {
        return iterable.slice().sort((a, b) => b - a).slice(0, n);
    }
}
it = iterable[Symbol.iterator]();
result = [];
for (var i = 0; i < n; i++) {
    var next = it.next();
    if (next.done) break;
    result.push([next.value, -i]);
}
if (!result.length) {
    return result;
}
heapify(result);
var top = result[0][0];
var order = -n;
var _heapreplace = heapreplace;
for (var elem of it) {
    if (top < elem) {
        _heapreplace(result, [elem, order]);
        top = result[0][0];
        order = result[0][1];
        order -= 1;
    }
}
result.sort((a, b) => b[0] - a[0]);
return result.map(x => x[0]);
    /// --- BLOCK END 16

}

function assert_value_equal(a, b){
    /// --- BLOCK BEGIN 17
if (!(Math.abs(a - b) <= 0.0001)) {
    throw new Error("Assertion failed: abs(a - b) <= 0.0001");
}
    /// --- BLOCK END 17

}

function assert_equal(a, b){
    /// --- BLOCK BEGIN 18
for (var index = 0; index < a.length; index++) {
    assert_value_equal(a[index], b[index]);
}
    /// --- BLOCK END 18

}

function test_heappush_help_function(items){
    /// --- BLOCK BEGIN 19
var heap = [];
    for (var index = 0; index < items.length; index++) {
        var item = items[index];
        heappush(heap, item);
    }
    var a = heappop(heap);
    var b = heappop(heap);
    return [a, b];
    /// --- BLOCK END 19

}

function test_heapify_help_function(x){
    /// --- BLOCK BEGIN 21
    heapify(x);
    var a = heappop(x);
    var b = heappop(x);
    return [a, b];
    /// --- BLOCK END 21

}

function test_heappushpop_help_function(x, i){
    /// --- BLOCK BEGIN 23
heapify(x);
    var a = heappushpop(x, i);
    return a;
    /// --- BLOCK END 23

}

function test_heapreplace_help_function(x, i){
    /// --- BLOCK BEGIN 25
    heapify(x);
    var a = heapreplace(x, i);
    var b = heappop(x);
    return [a, b];
    /// --- BLOCK END 25

}
