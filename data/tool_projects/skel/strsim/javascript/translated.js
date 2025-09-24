var {input_shanghai, input_shanghai_city} = require('./tracer_skip.js')


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

function StringDistance(...args){
    function distance(s0, s1){
        /// --- BLOCK BEGIN 1
return null;    
        /// --- BLOCK END 1
    
    }
    
    var class_var = SkelClass('StringDistance');
    class_var.distance = distance;
    return class_var;
}


function NormalizedStringDistance(...args){
    function distance(s0, s1){
        /// --- BLOCK BEGIN 2
return null;    
        /// --- BLOCK END 2
    
    }
    
    var class_var = StringDistance(...args);
    class_var._class_name = 'NormalizedStringDistance;' + class_var._class_name;
    class_var.distance = distance;
    return class_var;
}


function MetricStringDistance(...args){
    function distance(s0, s1){
        /// --- BLOCK BEGIN 3
return null;    
        /// --- BLOCK END 3
    
    }
    
    var class_var = StringDistance(...args);
    class_var._class_name = 'MetricStringDistance;' + class_var._class_name;
    class_var.distance = distance;
    return class_var;
}


function Levenshtein(...args){
    function distance(s0, s1){
        /// --- BLOCK BEGIN 4
if (s0 === null) {
    throw new TypeError("Argument s0 is NoneType.");
}
if (s1 === null) {
    throw new TypeError("Argument s1 is NoneType.");
}
if (s0 === s1) {
    return 0.0;
}
if (s0.length === 0) {
    return s1.length;
}
if (s1.length === 0) {
    return s0.length;
}
var v0 = new Array(s1.length + 1);
var v1 = new Array(s1.length + 1);
for (var i = 0; i < v0.length; i++) {
    v0[i] = i;
}
for (var i = 0; i < s0.length; i++) {
    v1[0] = i + 1;
    for (var j = 0; j < s1.length; j++) {
        var cost = 1;
        if (s0[i] === s1[j]) {
            cost = 0;
        }
        v1[j + 1] = Math.min(v1[j] + 1, v0[j + 1] + 1, v0[j] + cost);
    }
    var temp = v0;
    v0 = v1;
    v1 = temp;
}
return v0[s1.length];    
        /// --- BLOCK END 4
    
    }
    
    var class_var = MetricStringDistance(...args);
    class_var._class_name = 'Levenshtein;' + class_var._class_name;
    class_var.distance = distance;
    return class_var;
}


function LongestCommonSubsequence(...args){
    function distance(s0, s1){
        /// --- BLOCK BEGIN 5
        if (s0 === null) {
            throw new TypeError("Argument s0 is NoneType.");
        }
        if (s1 === null) {
            throw new TypeError("Argument s1 is NoneType.");
        }
        if (s0 === s1) {
            return 0.0;
        }
        return s0.length + s1.length - 2 * class_var.length(s0, s1);    
        /// --- BLOCK END 5
    
    }
    
    function length(s0, s1){
        /// --- BLOCK BEGIN 6
if (s0 === null) {
    throw new TypeError("Argument s0 is NoneType.");
}
if (s1 === null) {
    throw new TypeError("Argument s1 is NoneType.");
}
var s0_len = s0.length, s1_len = s1.length;
var x = s0.slice(), y = s1.slice();
var matrix = Array.from({ length: s0_len + 1 }, () => Array(s1_len + 1).fill(0));
for (var i = 1; i <= s0_len; i++) {
    for (var j = 1; j <= s1_len; j++) {
        if (x[i - 1] === y[j - 1]) {
            matrix[i][j] = matrix[i - 1][j - 1] + 1;
        } else {
            matrix[i][j] = Math.max(matrix[i][j - 1], matrix[i - 1][j]);
        }
    }
}
return matrix[s0_len][s1_len];
    
        /// --- BLOCK END 6
    
    }
    
    var class_var = StringDistance(...args);
    class_var._class_name = 'LongestCommonSubsequence;' + class_var._class_name;
    class_var.distance = distance;
    class_var.length = length;
    return class_var;
}


function MetricLCS(){
    function __init__(){
        /// --- BLOCK BEGIN 7
class_var.lcs = new LongestCommonSubsequence();    
        /// --- BLOCK END 7
    
    }
    
    function distance(s0, s1){
        /// --- BLOCK BEGIN 8
        if (s0 === null) {
            throw new TypeError("Argument s0 is NoneType.");
        }
        if (s1 === null) {
            throw new TypeError("Argument s1 is NoneType.");
        }
        if (s0 === s1) {
            return 0.0;
        }
        var max_len = Math.max(s0.length, s1.length);
        if (max_len === 0) {
            return 0.0;
        }
        return 1.0 - (1.0 * class_var.lcs.length(s0, s1)) / max_len;    
        /// --- BLOCK END 8
    
    }
    
    var class_var = MetricStringDistance();
    class_var._class_name = 'MetricLCS;' + class_var._class_name;
    class_var.__init__ = __init__;
    class_var.distance = distance;
    __init__();
    return class_var;
}


function NGram(param_0){
    function __init__(n){
        /// --- BLOCK BEGIN 9
class_var.n = n;
return null;    
        /// --- BLOCK END 9
    
    }
    
    function distance(s0, s1){
        /// --- BLOCK BEGIN 10
if (s0 === null) {
    throw new TypeError("Argument s0 is NoneType.");
}
if (s1 === null) {
    throw new TypeError("Argument s1 is NoneType.");
}
if (s0 === s1) {
    return 0.0;
}
var special = '\n';
var sl = s0.length;
var tl = s1.length;
if (sl === 0 || tl === 0) {
    return 1.0;
}
var cost = 0;
if (sl < class_var.n || tl < class_var.n) {
    for (var i = 0; i < Math.min(sl, tl); i++) {
        if (s0[i] === s1[i]) {
            cost += 1;
        }
    }
    return 1.0 - cost / Math.max(sl, tl);
}
var sa = Array(sl + class_var.n - 1).fill('');
for (var i = 0; i < sa.length; i++) {
    if (i < class_var.n - 1) {
        sa[i] = special;
    } else {
        sa[i] = s0[i - class_var.n + 1];
    }
}
var p = Array(sl + 1).fill(0.0);
var d = Array(sl + 1).fill(0.0);
var t_j = Array(class_var.n).fill('');
for (var i = 0; i < sl + 1; i++) {
    p[i] = 1.0 * i;
}
for (var j = 1; j < tl + 1; j++) {
    if (j < class_var.n) {
        for (var ti = 0; ti < class_var.n - j; ti++) {
            t_j[ti] = special;
        }
        for (var ti = class_var.n - j; ti < class_var.n; ti++) {
            t_j[ti] = s1[ti - (class_var.n - j)];
        }
    } else {
        t_j = s1.slice(j - class_var.n, j);
    }
    d[0] = 1.0 * j;
    for (var i = 1; i < sl + 1; i++) {
        cost = 0;
        var tn = class_var.n;
        for (var ni = 0; ni < class_var.n; ni++) {
            if (sa[i - 1 + ni] !== t_j[ni]) {
                cost += 1;
            } else if (sa[i - 1 + ni] === special) {
                tn -= 1;
            }
        }
        var ec = cost / tn;
        d[i] = Math.min(d[i - 1] + 1, p[i] + 1, p[i - 1] + ec);
    }
    var temp = p;
    p = d;
    d = temp;
}
return p[sl] / Math.max(tl, sl);    
        /// --- BLOCK END 10
    
    }
    
    var class_var = NormalizedStringDistance();
    class_var._class_name = 'NGram;' + class_var._class_name;
    class_var.__init__ = __init__;
    class_var.distance = distance;
    __init__(param_0);
    return class_var;
}


function Damerau(...args){
    function distance(s0, s1){
        /// --- BLOCK BEGIN 11
if (s0 === null) {
    throw new TypeError("Argument s0 is NoneType.");
}
if (s1 === null) {
    throw new TypeError("Argument s1 is NoneType.");
}
if (s0 === s1) {
    return 0.0;
}
var inf = s0.length + s1.length;
var da = {};
for (var i = 0; i < s0.length; i++) {
    da[s0[i]] = '0';
}
for (var i = 0; i < s1.length; i++) {
    da[s1[i]] = '0';
}
var h = [];
for (var i = 0; i < s0.length + 2; i++) {
    h.push(new Array(s1.length + 2).fill(0));
}
for (var i = 0; i < s0.length + 1; i++) {
    h[i + 1][0] = inf;
    h[i + 1][1] = i;
}
for (var j = 0; j < s1.length + 1; j++) {
    h[0][j + 1] = inf;
    h[1][j + 1] = j;
}
for (var i = 1; i < s0.length + 1; i++) {
    var db = 0;
    for (var j = 1; j < s1.length + 1; j++) {
        var i1 = parseInt(da[s1[j - 1]]);
        var j1 = db;
        var cost = 1;
        if (s0[i - 1] === s1[j - 1]) {
            cost = 0;
            db = j;
        }
        h[i + 1][j + 1] = Math.min(h[i][j] + cost,
                                   h[i + 1][j] + 1,
                                   h[i][j + 1] + 1,
                                   h[i1][j1] + (i - i1 - 1) + 1 + (j - j1 - 1));
    }
    da[s0[i - 1]] = i.toString();
}
return h[s0.length + 1][s1.length + 1];    
        /// --- BLOCK END 11
    
    }
    
    var class_var = MetricStringDistance(...args);
    class_var._class_name = 'Damerau;' + class_var._class_name;
    class_var.distance = distance;
    return class_var;
}


function ShingleBased(param_0){
    function __init__(k){
        /// --- BLOCK BEGIN 12
class_var.k = k;
return null;    
        /// --- BLOCK END 12
    
    }
    
    function get_k(){
        /// --- BLOCK BEGIN 13
return class_var.k;    
        /// --- BLOCK END 13
    
    }
    
    function get_profile(string){
        /// --- BLOCK BEGIN 14
var shingles = {};
    var no_space_str = string.replace(/\s+/g, " ");
    for (var i = 0; i <= no_space_str.length - class_var.k; i++) {
        var shingle = no_space_str.substring(i, i + class_var.k);
        var old = shingles[shingle];
        if (old) {
            shingles[shingle] = old + 1;
        } else {
            shingles[shingle] = 1;
        }
    }
    return shingles;
    
        /// --- BLOCK END 14
    
    }
    
    var class_var = SkelClass('ShingleBased');
    class_var.__init__ = __init__;
    class_var.get_k = get_k;
    class_var.get_profile = get_profile;
    __init__(param_0);
    return class_var;
}


function StringSimilarity(...args){
    function similarity(s0, s1){
        /// --- BLOCK BEGIN 15
return null;    
        /// --- BLOCK END 15
    
    }
    
    var class_var = SkelClass('StringSimilarity');
    class_var.similarity = similarity;
    return class_var;
}


function NormalizedStringSimilarity(...args){
    function similarity(s0, s1){
        /// --- BLOCK BEGIN 16
return null;    
        /// --- BLOCK END 16
    
    }
    
    var class_var = StringSimilarity(...args);
    class_var._class_name = 'NormalizedStringSimilarity;' + class_var._class_name;
    class_var.similarity = similarity;
    return class_var;
}


function Cosine(param_0){
    function __init__(k){
        /// --- BLOCK BEGIN 17
return null;    
        /// --- BLOCK END 17
    
    }
    
    function distance(s0, s1){
        /// --- BLOCK BEGIN 18
return 1.0 - class_var.similarity(s0, s1);    
        /// --- BLOCK END 18
    
    }
    
    function similarity(s0, s1){
        /// --- BLOCK BEGIN 19
if (s0 === null) {
    throw new TypeError("Argument s0 is NoneType.");
}
if (s1 === null) {
    throw new TypeError("Argument s1 is NoneType.");
}
if (s0 === s1) {
    return 1.0;
}
if (s0.length < class_var.get_k() || s1.length < class_var.get_k()) {
    return 0.0;
}
var profile0 = class_var.get_profile(s0);
var profile1 = class_var.get_profile(s1);
return class_var._dot_product(profile0, profile1) / (class_var._norm(profile0) * class_var._norm(profile1));
    
        /// --- BLOCK END 19
    
    }
    
    function similarity_profiles(profile0, profile1){
        /// --- BLOCK BEGIN 20
return class_var._dot_product(profile0, profile1) / (class_var._norm(profile0) * class_var._norm(profile1));    
        /// --- BLOCK END 20
    
    }
    
    function _dot_product(profile0, profile1){
        /// --- BLOCK BEGIN 21
var small = profile1;
    var large = profile0;
    if (Object.keys(profile0).length < Object.keys(profile1).length) {
        small = profile0;
        large = profile1;
    }
    var agg = 0.0;
    for (var k in small) {
        var v = small[k];
        var i = large[k];
        if (!i) {
            continue;
        }
        agg += 1.0 * v * i;
    }
    return agg;
    
        /// --- BLOCK END 21
    
    }
    
    function _norm(profile){
        /// --- BLOCK BEGIN 22
var agg = 0.0;
    for (var k in profile) {
        var v = profile[k];
        agg += 1.0 * v * v;
    }
    return Math.sqrt(agg);
    
        /// --- BLOCK END 22
    
    }
    
    var class_var = ShingleBased(param_0);
    class_var._class_name = 'Cosine;' + class_var._class_name;
    class_var.__init__ = __init__;
    class_var.distance = distance;
    class_var.similarity = similarity;
    class_var.similarity_profiles = similarity_profiles;
    class_var._dot_product = _dot_product;
    class_var._norm = _norm;
    __init__(param_0);
    return class_var;
}


function Jaccard(param_0){
    function __init__(k){
        /// --- BLOCK BEGIN 23
return null;    
        /// --- BLOCK END 23
    
    }
    
    function distance(s0, s1){
        /// --- BLOCK BEGIN 24
return 1.0 - class_var.similarity(s0, s1);    
        /// --- BLOCK END 24
    
    }
    
    function similarity(s0, s1){
        /// --- BLOCK BEGIN 25
if (s0 === null) {
    throw new TypeError("Argument s0 is NoneType.");
}
if (s1 === null) {
    throw new TypeError("Argument s1 is NoneType.");
}
if (s0 === s1) {
    return 1.0;
}
if (s0.length < class_var.get_k() || s1.length < class_var.get_k()) {
    return 0.0;
}
var profile0 = class_var.get_profile(s0);
var profile1 = class_var.get_profile(s1);
var union = new Set();
for (var ite in profile0) {
    union.add(ite);
}
for (var ite in profile1) {
    union.add(ite);
}
var inter = (Object.keys(profile0).length + Object.keys(profile1).length - union.size);
return 1.0 * inter / union.size;    
        /// --- BLOCK END 25
    
    }
    
    var class_var = ShingleBased(param_0);
    class_var._class_name = 'Jaccard;' + class_var._class_name;
    class_var.__init__ = __init__;
    class_var.distance = distance;
    class_var.similarity = similarity;
    __init__(param_0);
    return class_var;
}


function JaroWinkler(param_0){
    function __init__(threshold){
        /// --- BLOCK BEGIN 26
class_var.threshold = threshold;
class_var.three = 3;
class_var.jw_coef = 0.1;    
        /// --- BLOCK END 26
    
    }
    
    function get_threshold(){
        /// --- BLOCK BEGIN 27
return class_var.threshold;    
        /// --- BLOCK END 27
    
    }
    
    function similarity(s0, s1){
        /// --- BLOCK BEGIN 28
if (s0 === null) {
    throw new TypeError("Argument s0 is NoneType.");
}
if (s1 === null) {
    throw new TypeError("Argument s1 is NoneType.");
}
if (s0 === s1) {
    return 1.0;
}
var mtp = class_var.matches(s0, s1);
var m = mtp[0];
if (m === 0) {
    return 0.0;
}
var j = (m / s0.length + m / s1.length + (m - mtp[1]) / m) / class_var.three;
var jw = j;
if (j > class_var.get_threshold()) {
    jw = j + Math.min(class_var.jw_coef, 1.0 / mtp[class_var.three]) * mtp[2] * (1 - j);
}
return jw;    
        /// --- BLOCK END 28
    
    }
    
    function distance(s0, s1){
        /// --- BLOCK BEGIN 29
return 1.0 - class_var.similarity(s0, s1);    
        /// --- BLOCK END 29
    
    }
    
    function matches(s0, s1){
        /// --- BLOCK BEGIN 30
var max_str, min_str;
        if (s0.length > s1.length) {
            max_str = s0;
            min_str = s1;
        } else {
            max_str = s1;
            min_str = s0;
        }
        var ran = Math.max(Math.floor(max_str.length / 2 - 1), 0);
        var match_indexes = Array(min_str.length).fill(-1);
        var match_flags = Array(max_str.length).fill(false);
        var matches = 0;
        for (var mi = 0; mi < min_str.length; mi++) {
            var c1 = min_str[mi];
            for (var xi = Math.max(mi - ran, 0); xi < Math.min(mi + ran + 1, max_str.length); xi++) {
                if (!match_flags[xi] && c1 === max_str[xi]) {
                    match_indexes[mi] = xi;
                    match_flags[xi] = true;
                    matches++;
                    break;
                }
            }
        }
        var ms0 = Array(matches).fill(0);
        var ms1 = Array(matches).fill(0);
        var si = 0;
        for (var i = 0; i < min_str.length; i++) {
            if (match_indexes[i] !== -1) {
                ms0[si] = min_str[i];
                si++;
            }
        }
        si = 0;
        for (var j = 0; j < max_str.length; j++) {
            if (match_flags[j]) {
                ms1[si] = max_str[j];
                si++;
            }
        }
        var transpositions = 0;
        for (var mi = 0; mi < ms0.length; mi++) {
            if (ms0[mi] !== ms1[mi]) {
                transpositions++;
            }
        }
        var prefix = 0;
        for (var mi = 0; mi < min_str.length; mi++) {
            if (s0[mi] === s1[mi]) {
                prefix++;
            } else {
                break;
            }
        }
        return [matches, Math.floor(transpositions / 2), prefix, max_str.length];    
        /// --- BLOCK END 30
    
    }
    
    var class_var = NormalizedStringSimilarity();
    class_var._class_name = 'JaroWinkler;' + class_var._class_name;
    class_var.__init__ = __init__;
    class_var.get_threshold = get_threshold;
    class_var.similarity = similarity;
    class_var.distance = distance;
    class_var.matches = matches;
    __init__(param_0);
    return class_var;
}


function NormalizedLevenshtein(){
    function __init__(){
        /// --- BLOCK BEGIN 31
class_var.levenshtein = new Levenshtein();    
        /// --- BLOCK END 31
    
    }
    
    function distance(s0, s1){
        /// --- BLOCK BEGIN 32
        if (s0 === null) {
            throw new TypeError("Argument s0 is NoneType.");
        }
        if (s1 === null) {
            throw new TypeError("Argument s1 is NoneType.");
        }
        if (s0 === s1) {
            return 0.0;
        }
        var m_len = Math.max(s0.length, s1.length);
        if (m_len === 0) {
            return 0.0;
        }
        return class_var.levenshtein.distance(s0, s1) / m_len;    
        /// --- BLOCK END 32
    
    }
    
    function similarity(s0, s1){
        /// --- BLOCK BEGIN 33
return 1.0 - class_var.distance(s0, s1);    
        /// --- BLOCK END 33
    
    }
    
    var class_var = NormalizedStringDistance();
    class_var._class_name = 'NormalizedLevenshtein;' + class_var._class_name;
    class_var.__init__ = __init__;
    class_var.distance = distance;
    class_var.similarity = similarity;
    __init__();
    return class_var;
}


function OptimalStringAlignment(...args){
    function distance(s0, s1){
        /// --- BLOCK BEGIN 34
if (s0 === null) {
    throw new TypeError("Argument s0 is NoneType.");
}
if (s1 === null) {
    throw new TypeError("Argument s1 is NoneType.");
}
if (s0 === s1) {
    return 0.0;
}
var n = s0.length, m = s1.length;
if (n === 0) {
    return 1.0 * n;
}
if (m === 0) {
    return 1.0 * m;
}
var d = Array.from({ length: n + 2 }, () => Array(m + 2).fill(0));
for (var i = 0; i <= n; i++) {
    d[i][0] = i;
}
for (var j = 0; j <= m; j++) {
    d[0][j] = j;
}
for (var i = 1; i <= n; i++) {
    for (var j = 1; j <= m; j++) {
        var cost = 1;
        if (s0[i - 1] === s1[j - 1]) {
            cost = 0;
        }
        d[i][j] = Math.min(d[i - 1][j - 1] + cost, d[i][j - 1] + 1, d[i - 1][j] + 1);
        if (i > 1 && j > 1 && s0[i - 1] === s1[j - 2] && s0[i - 2] === s1[j - 1]) {
            d[i][j] = Math.min(d[i][j], d[i - 2][j - 2] + cost);
        }
    }
}
return d[n][m];    
        /// --- BLOCK END 34
    
    }
    
    var class_var = StringDistance(...args);
    class_var._class_name = 'OptimalStringAlignment;' + class_var._class_name;
    class_var.distance = distance;
    return class_var;
}


function OverlapCoefficient(param_0){
    function __init__(k){
        /// --- BLOCK BEGIN 35
return null;    
        /// --- BLOCK END 35
    
    }
    
    function distance(s0, s1){
        /// --- BLOCK BEGIN 36
return 1.0 - class_var.similarity(s0, s1);    
        /// --- BLOCK END 36
    
    }
    
    function similarity(s0, s1){
        /// --- BLOCK BEGIN 37
        if (s0 === null) {
            throw new TypeError("Argument s0 is NoneType.");
        }
        if (s1 === null) {
            throw new TypeError("Argument s1 is NoneType.");
        }
        if (s0 === s1) {
            return 1.0;
        }
        var union = new Set();
        var profile0 = class_var.get_profile(s0);
        var profile1 = class_var.get_profile(s1);
        for (var k in profile0) {
            union.add(k);
        }
        for (var k in profile1) {
            union.add(k);
        }
        var inter = parseInt(Object.keys(profile0).length + Object.keys(profile1).length - union.size);
        return inter / Math.min(Object.keys(profile0).length, Object.keys(profile1).length);    
        /// --- BLOCK END 37
    
    }
    
    var class_var = ShingleBased(param_0);
    class_var._class_name = 'OverlapCoefficient;' + class_var._class_name;
    class_var.__init__ = __init__;
    class_var.distance = distance;
    class_var.similarity = similarity;
    __init__(param_0);
    return class_var;
}


function QGram(param_0){
    function __init__(k){
        /// --- BLOCK BEGIN 38
return null;    
        /// --- BLOCK END 38
    
    }
    
    function distance(s0, s1){
        /// --- BLOCK BEGIN 39
        if (s0 === null) {
            throw new TypeError("Argument s0 is NoneType.");
        }
        if (s1 === null) {
            throw new TypeError("Argument s1 is NoneType.");
        }
        if (s0 === s1) {
            return 0.0;
        }
        var profile0 = class_var.get_profile(s0);
        var profile1 = class_var.get_profile(s1);
        return class_var.distance_profile(profile0, profile1);    
        /// --- BLOCK END 39
    
    }
    
    function distance_profile(profile0, profile1){
        /// --- BLOCK BEGIN 40
var union = new Set();
        for (var k in profile0) {
            union.add(k);
        }
        for (var k in profile1) {
            union.add(k);
        }
        var agg = 0;
        union.forEach(function(k) {
            var v0 = 0, v1 = 0;
            if (profile0[k] !== undefined) {
                v0 = parseInt(profile0[k]);
            }
            if (profile1[k] !== undefined) {
                v1 = parseInt(profile1[k]);
            }
            agg += Math.abs(v0 - v1);
        });
        return agg;
    
        /// --- BLOCK END 40
    
    }
    
    var class_var = ShingleBased(param_0);
    class_var._class_name = 'QGram;' + class_var._class_name;
    class_var.__init__ = __init__;
    class_var.distance = distance;
    class_var.distance_profile = distance_profile;
    __init__(param_0);
    return class_var;
}


function SIFT4Options(param_0){
    function __init__(options){
        function _code0(x){
            /// --- BLOCK BEGIN 41
return Array.from(x);        
            /// --- BLOCK END 41
        
        }
        
        function _code1(t1, t2){
            /// --- BLOCK BEGIN 42
return t1 === t2;        
            /// --- BLOCK END 42
        
        }
        
        function _code2(t1, t2){
            /// --- BLOCK BEGIN 43
return 1;        
            /// --- BLOCK END 43
        
        }
        
        function _code3(x){
            /// --- BLOCK BEGIN 44
return x;        
            /// --- BLOCK END 44
        
        }
        
        function _code4(c1, c2){
            /// --- BLOCK BEGIN 45
return 1;        
            /// --- BLOCK END 45
        
        }
        
        function _code5(lcss, trans){
            /// --- BLOCK BEGIN 46
return lcss - trans;        
            /// --- BLOCK END 46
        
        }
        
        /// --- BLOCK BEGIN 47
class_var.options = {
    'maxdistance': 0,
    'tokenizer': _code0,
    'tokenmatcher': _code1,
    'matchingevaluator': _code2,
    'locallengthevaluator': _code3,
    'transpositioncostevaluator': _code4,
    'transpositionsevaluator': _code5
};
var otheroptions = {
    'tokenizer': {'ngram': class_var.ngramtokenizer, 'wordsplit': class_var.wordsplittokenizer, 'characterfrequency': class_var.characterfrequencytokenizer},
    'tokematcher': {'sift4tokenmatcher': class_var.sift4tokenmatcher},
    'matchingevaluator': {'sift4matchingevaluator': class_var.sift4matchingevaluator},
    'locallengthevaluator': {'rewardlengthevaluator': class_var.rewardlengthevaluator, 'rewardlengthevaluator2': class_var.rewardlengthevaluator2},
    'transpositioncostevaluator': {'longertranspositionsaremorecostly': class_var.longertranspositionsaremorecostly},
    'transpositionsevaluator': {}
};
if (typeof options === 'object' && options !== null) {
    for (var k in options) {
        if (options.hasOwnProperty(k) && class_var.options.hasOwnProperty(k)) {
            if (k === 'maxdistance') {
                if (typeof options[k] === 'number') {
                    class_var.options[k] = options[k];
                } else {
                    throw new Error("Option maxdistance should be int");
                }
            } else {
                if (typeof options[k] === 'function') {
                    class_var.options[k] = options[k];
                } else {
                    if (otheroptions[k].hasOwnProperty(options[k])) {
                        class_var.options[k] = otheroptions[k][options[k]];
                    } else {
                        var msg = "Option " + k + " should be callable or one of [" + Object.keys(otheroptions[k]).join(', ') + "]";
                        throw new Error(msg);
                    }
                }
            }
        } else {
            throw new Error("Option " + k + " not recognized.");
        }
    }
} else if (options !== null) {
    throw new Error("options should be a dictionary");
}
class_var.maxdistance = class_var.options['maxdistance'];
class_var.tokenizer = class_var.options['tokenizer'];
class_var.tokenmatcher = class_var.options['tokenmatcher'];
class_var.matchingevaluator = class_var.options['matchingevaluator'];
class_var.locallengthevaluator = class_var.options['locallengthevaluator'];
class_var.transpositioncostevaluator = class_var.options['transpositioncostevaluator'];
class_var.transpositionsevaluator = class_var.options['transpositionsevaluator'];    
        /// --- BLOCK END 47
    
    }
    
    function ngramtokenizer(s, n){
        /// --- BLOCK BEGIN 48
var result = [];
        if (!s) {
            return result;
        }
        for (var i = 0; i < s.length - n - 1; i++) {
            result.push(s.substring(i, i + n));
        }
        return result;    
        /// --- BLOCK END 48
    
    }
    
    function wordsplittokenizer(s){
        /// --- BLOCK BEGIN 49
if (!s) {
    return [];
}
return s.split(' ');
    
        /// --- BLOCK END 49
    
    }
    
    function characterfrequencytokenizer(s){
        /// --- BLOCK BEGIN 50
var letters = 'abcdefghijklmnopqrstuvwxyz'.split('');
    return letters.map(function(x) { return s.toLowerCase().split(x).length - 1; });    
        /// --- BLOCK END 50
    
    }
    
    function sift4tokenmatcher(t1, t2){
        /// --- BLOCK BEGIN 51
var similarity = 1 - SIFT4().distance(t1, t2, 5) / Math.max(t1.length, t2.length);
        return similarity > 0.7;    
        /// --- BLOCK END 51
    
    }
    
    function sift4matchingevaluator(t1, t2){
        /// --- BLOCK BEGIN 52
var similarity = 1 - SIFT4().distance(t1, t2, 5) / Math.max(t1.length, t2.length);
    return similarity;    
        /// --- BLOCK END 52
    
    }
    
    function rewardlengthevaluator(l){
        /// --- BLOCK BEGIN 53
if (l < 1) {
    return l;
}
return l - 1 / (l + 1);    
        /// --- BLOCK END 53
    
    }
    
    function rewardlengthevaluator2(l){
        /// --- BLOCK BEGIN 54
return Math.pow(l, 1.5);    
        /// --- BLOCK END 54
    
    }
    
    function longertranspositionsaremorecostly(c1, c2){
        /// --- BLOCK BEGIN 55
return Math.abs(c2 - c1) / 9 + 1;    
        /// --- BLOCK END 55
    
    }
    
    var class_var = MetricStringDistance();
    class_var._class_name = 'SIFT4Options;' + class_var._class_name;
    class_var.__init__ = __init__;
    class_var.ngramtokenizer = ngramtokenizer;
    class_var.wordsplittokenizer = wordsplittokenizer;
    class_var.characterfrequencytokenizer = characterfrequencytokenizer;
    class_var.sift4tokenmatcher = sift4tokenmatcher;
    class_var.sift4matchingevaluator = sift4matchingevaluator;
    class_var.rewardlengthevaluator = rewardlengthevaluator;
    class_var.rewardlengthevaluator2 = rewardlengthevaluator2;
    class_var.longertranspositionsaremorecostly = longertranspositionsaremorecostly;
    __init__(param_0);
    return class_var;
}


function SIFT4(...args){
    function distance(s1, s2, maxoffset, options){
        /// --- BLOCK BEGIN 56
options = new SIFT4Options(options);
var t1 = options.tokenizer(s1), t2 = options.tokenizer(s2);
var l1 = t1.length, l2 = t2.length;
if (l1 === 0) {
    return l2;
}
if (l2 === 0) {
    return l1;
}
var c1 = 0, c2 = 0, lcss = 0, local_cs = 0, trans = 0, offset_arr = [];
while (c1 < l1 && c2 < l2) {
    if (options.tokenmatcher(t1[c1], t2[c2])) {
        local_cs += options.matchingevaluator(t1[c1], t2[c2]);
        var isTrans = false;
        var i = 0;
        while (i < offset_arr.length) {
            var ofs = offset_arr[i];
            if (c1 <= ofs.c1 || c2 <= ofs.c2) {
                isTrans = Math.abs(c2 - c1) >= Math.abs(ofs.c2 - ofs.c1);
                if (isTrans) {
                    trans += options.transpositioncostevaluator(c1, c2);
                } else {
                    if (!ofs.trans) {
                        ofs.trans = true;
                        trans += options.transpositioncostevaluator(ofs.c1, ofs.c2);
                    }
                }
                break;
            } else {
                if (c1 > ofs.c2 && c2 > ofs.c1) {
                    offset_arr.splice(i, 1);
                } else {
                    i++;
                }
            }
        }
        offset_arr.push({c1: c1, c2: c2, trans: isTrans});
    } else {
        lcss += options.locallengthevaluator(local_cs);
        local_cs = 0;
        if (c1 !== c2) {
            c1 = c2 = Math.min(c1, c2);
        }
        for (i = 0; i < maxoffset; i++) {
            if ((c1 + i < l1) || (c2 + i < l2)) {
                if ((c1 + i < l1) && options.tokenmatcher(t1[c1 + i], t2[c2])) {
                    c1 += i - 1;
                    c2 -= 1;
                    break;
                }
                if ((c2 + i < l2) && options.tokenmatcher(t1[c1], t2[c2 + i])) {
                    c1 -= 1;
                    c2 += i - 1;
                    break;
                }
            }
        }
    }
    c1++;
    c2++;
    if (options.maxdistance) {
        var temporarydistance = options.locallengthevaluator(Math.max(c1, c2)) - options.transpositionsevaluator(lcss, trans);
        if (temporarydistance >= options.maxdistance) {
            return Math.round(temporarydistance);
        }
    }
    if (c1 >= l1 || c2 >= l2) {
        lcss += options.locallengthevaluator(local_cs);
        local_cs = 0;
        c1 = c2 = Math.min(c1, c2);
    }
}
lcss += options.locallengthevaluator(local_cs);
return Math.round(options.locallengthevaluator(Math.max(l1, l2)) - options.transpositionsevaluator(lcss, trans));    
        /// --- BLOCK END 56
    
    }
    
    var class_var = SkelClass('SIFT4');
    class_var.distance = distance;
    return class_var;
}


function SorensenDice(param_0){
    function __init__(k){
        /// --- BLOCK BEGIN 57
return null;    
        /// --- BLOCK END 57
    
    }
    
    function distance(s0, s1){
        /// --- BLOCK BEGIN 58
return 1.0 - class_var.similarity(s0, s1);    
        /// --- BLOCK END 58
    
    }
    
    function similarity(s0, s1){
        /// --- BLOCK BEGIN 59
if (s0 === null) {
    throw new TypeError("Argument s0 is NoneType.");
}
if (s1 === null) {
    throw new TypeError("Argument s1 is NoneType.");
}
if (s0 === s1) {
    return 1.0;
}
var union = new Set();
var profile0 = class_var.get_profile(s0);
var profile1 = class_var.get_profile(s1);
for (var k in profile0) {
    union.add(k);
}
for (var k in profile1) {
    union.add(k);
}
var inter = parseInt(Object.keys(profile0).length + Object.keys(profile1).length - union.size);
return 2.0 * inter / (Object.keys(profile0).length + Object.keys(profile1).length);
    
        /// --- BLOCK END 59
    
    }
    
    var class_var = ShingleBased(param_0);
    class_var._class_name = 'SorensenDice;' + class_var._class_name;
    class_var.__init__ = __init__;
    class_var.distance = distance;
    class_var.similarity = similarity;
    __init__(param_0);
    return class_var;
}


function default_insertion_cost(char){
    /// --- BLOCK BEGIN 60
return 1.0;
    /// --- BLOCK END 60

}

function default_deletion_cost(char){
    /// --- BLOCK BEGIN 61
return 1.0;
    /// --- BLOCK END 61

}

function default_substitution_cost(char_a, char_b){
    /// --- BLOCK BEGIN 62
return 1.0;
    /// --- BLOCK END 62

}

function WeightedLevenshtein(param_0, param_1, param_2){
    function __init__(substitution_cost_fn, insertion_cost_fn, deletion_cost_fn){
        /// --- BLOCK BEGIN 63
class_var.substitution_cost_fn = substitution_cost_fn;
class_var.insertion_cost_fn = insertion_cost_fn;
class_var.deletion_cost_fn = deletion_cost_fn;
return null;
    
        /// --- BLOCK END 63
    
    }
    
    function distance(s0, s1){
        /// --- BLOCK BEGIN 64
if (s0 === null) {
    throw new TypeError("Argument s0 is NoneType.");
}
if (s1 === null) {
    throw new TypeError("Argument s1 is NoneType.");
}
if (s0 === s1) {
    return 0.0;
}
if (s0.length === 0) {
    return s1.split('').reduce(function(cost, char) {
        return cost + class_var.insertion_cost_fn(char);
    }, 0);
}
if (s1.length === 0) {
    return s0.split('').reduce(function(cost, char) {
        return cost + class_var.deletion_cost_fn(char);
    }, 0);
}
var v0 = new Array(s1.length + 1).fill(0.0);
var v1 = new Array(s1.length + 1).fill(0.0);
v0[0] = 0;
for (var i = 1; i < v0.length; i++) {
    v0[i] = v0[i - 1] + class_var.insertion_cost_fn(s1[i - 1]);
}
for (var i = 0; i < s0.length; i++) {
    var s0i = s0[i];
    var deletion_cost = class_var.deletion_cost_fn(s0i);
    v1[0] = v0[0] + deletion_cost;
    for (var j = 0; j < s1.length; j++) {
        var s1j = s1[j];
        var cost = 0;
        if (s0i !== s1j) {
            cost = class_var.substitution_cost_fn(s0i, s1j);
        }
        var insertion_cost = class_var.insertion_cost_fn(s1j);
        v1[j + 1] = Math.min(v1[j] + insertion_cost, v0[j + 1] + deletion_cost, v0[j] + cost);
    }
    var temp = v0;
    v0 = v1;
    v1 = temp;
}
return v0[s1.length];    
        /// --- BLOCK END 64
    
    }
    
    var class_var = StringDistance();
    class_var._class_name = 'WeightedLevenshtein;' + class_var._class_name;
    class_var.__init__ = __init__;
    class_var.distance = distance;
    __init__(param_0, param_1, param_2);
    return class_var;
}
