var {get_input, test_str, self_split} = require('./tracer_skip.js');
var tool_functions = {"get_input":get_input,"test_str":test_str};


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

function func_dict(...args){
    var class_var = {};
    return class_var;
}


function _detect_pathlib_path(p){
    /// --- BLOCK BEGIN 1
    if (typeof process !== 'undefined' && process.versions.node.split('.')[0] >= 4) {
        pathlib = require('path');
        if (user_check_type(p, pathlib)) {
            return true;
        }
    }
    return false;

    /// --- BLOCK END 1

}

function _ispath(p){
    /// --- BLOCK BEGIN 2
if (typeof p === 'string' || p instanceof Buffer) {
        return true;
    }
    return _detect_pathlib_path(p);

    /// --- BLOCK END 2

}

function _getpath(p){
    /// --- BLOCK BEGIN 3
if (typeof p === 'string' || p instanceof String) {
    return p;
} else if (p instanceof URL) {
    return p.pathname;
} else {
    return p.toString();
}
    /// --- BLOCK END 3

}

function TomlDecodeError(param_0, param_1, param_2){
    function __init__(msg, doc, pos){
        /// --- BLOCK BEGIN 4
var lineno = doc.substring(0, pos).split('\n').length;
var colno = pos - doc.lastIndexOf('\n', pos);
var emsg = msg + ' (line ' + lineno + ' column ' + colno + ' char ' + pos + ')';
ValueError.call(class_var, emsg);
class_var.msg = msg;
class_var.doc = doc;
class_var.pos = pos;
class_var.lineno = lineno;
class_var.colno = colno;    
        /// --- BLOCK END 4
    
    }
    
    var class_var = ValueError();
    class_var._class_name = 'TomlDecodeError;' + class_var._class_name;
    class_var.__init__ = __init__;
    __init__(param_0, param_1, param_2);
    return class_var;
}


function CommentValue(param_0, param_1, param_2, param_3){
    function __init__(val, comment, beginline, _dict){
        /// --- BLOCK BEGIN 5
class_var.val = val;
var separator = beginline ? "\n" : " ";
class_var.comment = separator + comment;
class_var._dict = _dict;    
        /// --- BLOCK END 5
    
    }
    
    function __getitem__(key){
        /// --- BLOCK BEGIN 6
return class_var.val[key];    
        /// --- BLOCK END 6
    
    }
    
    function __setitem__(key, value){
        /// --- BLOCK BEGIN 7
class_var.val[key] = value;    
        /// --- BLOCK END 7
    
    }
    
    function dump(dump_value_func){
        /// --- BLOCK BEGIN 8
var retstr = dump_value_func(class_var.val);
    if (user_check_type(class_var.val, class_var._dict)) {
        return class_var.comment + "\n" + retstr.toString();
    } else {
        return retstr.toString() + class_var.comment;
    }
    
        /// --- BLOCK END 8
    
    }
    
    var class_var = SkelClass('CommentValue');
    class_var.__init__ = __init__;
    class_var.__getitem__ = __getitem__;
    class_var.__setitem__ = __setitem__;
    class_var.dump = dump;
    __init__(param_0, param_1, param_2, param_3);
    return class_var;
}


function _strictly_valid_num(n){
    /// --- BLOCK BEGIN 9
n = n.trim();
    if (!n) {
        return false;
    }
    if (n[0] === '_') {
        return false;
    }
    if (n[n.length - 1] === '_') {
        return false;
    }
    if (n.includes("_.") || n.includes("._")) {
        return false;
    }
    if (n.length === 1) {
        return true;
    }
    if (n[0] === '0' && !['.', 'o', 'b', 'x'].includes(n[1])) {
        return false;
    }
    if (n[0] === '+' || n[0] === '-') {
        n = n.substring(1);
        if (n.length > 1 && n[0] === '0' && n[1] !== '.') {
            return false;
        }
    }
    if (n.includes('__')) {
        return false;
    }
    return true;
    /// --- BLOCK END 9

}

function load(f, _dict, decoder){
    /// --- BLOCK BEGIN 10
    // Not Reachable
    /// --- BLOCK END 10

}

function loads(s, _dict, decoder){
    function handle_keyname(){
        /// --- BLOCK BEGIN 11
key += item;
if (item === '\n') {
    throw new TomlDecodeError("Key name found without value. Reached end of line.", original, i);
}
if (openstring) {
    if (item === openstrchar) {
        var oddbackslash = false;
        var k = 1;
        while (i >= k && sl[i - k] === '\\') {
            oddbackslash = !oddbackslash;
            k += 1;
        }
        if (!oddbackslash) {
            keyname = 2;
            openstring = false;
            openstrchar = "";
        }
    }
    return "continue";
} else if (keyname === 1) {
    if (/\s/.test(item)) {
        keyname = 2;
        return "continue";
    } else if (item === '.') {
        dottedkey = true;
        return "continue";
    } else if (/[\w-]/.test(item)) { // \w matches alphanumeric and underscore
        return "continue";
    } else if (dottedkey && sl[i - 1] === '.' && (item === '"' || item === "'")) {
        openstring = true;
        openstrchar = item;
        return "continue";
    }
} else if (keyname === 2) {
    if (/\s/.test(item)) {
        if (dottedkey) {
            var nextitem = sl[i + 1];
            if (!/\s/.test(nextitem) && nextitem !== '.') {
                keyname = 1;
            }
        }
        return "continue";
    }
    if (item === '.') {
        dottedkey = true;
        var nextitem = sl[i + 1];
        if (!/\s/.test(nextitem) && nextitem !== '.') {
            keyname = 1;
        }
        return "continue";
    }
}
if (item === '=') {
    keyname = 0;
    prev_key = key.slice(0, -1).trim();
    key = '';
    dottedkey = false;
} else {
    throw new TomlDecodeError("Found invalid character in key name: '" + item + "'. Try quoting the key name.", original, i);
}    
        /// --- BLOCK END 11
    
    }
    
    function handle_single_quote_1(){
        /// --- BLOCK BEGIN 12
var k = 1;
        try {
            while (sl[i - k] === "'") {
                k += 1;
                if (k === 3) {
                    break;
                }
            }
        } catch (error) {
            if (error instanceof RangeError) {
                // pass
            } else {
                throw error; // rethrow the error if it's not a RangeError
            }
        }
        if (k === 3) {
            multilinestr = !multilinestr;
            openstring = multilinestr;
        } else {
            openstring = !openstring;
        }
        if (openstring) {
            openstrchar = "'";
        } else {
            openstrchar = "";
        }    
        /// --- BLOCK END 12
    
    }
    
    function handle_single_quote_2(){
        /// --- BLOCK BEGIN 13
var oddbackslash = false;
var k = 1;
var tripquote = false;
try {
    while (sl[i - k] === '"') {
        k += 1;
        if (k === 3) {
            tripquote = true;
            break;
        }
    }
    if (k === 1 || (k === 3 && tripquote)) {
        while (sl[i - k] === '\\') {
            oddbackslash = !oddbackslash;
            k += 1;
        }
    }
} catch (error) {
    if (error instanceof RangeError) {
        // pass
    } else {
        throw error;
    }
}
if (!oddbackslash) {
    if (tripquote) {
        multilinestr = !multilinestr;
        openstring = multilinestr;
    } else {
        openstring = !openstring;
    }
}
if (openstring) {
    openstrchar = '"';
} else {
    openstrchar = "";
}    
        /// --- BLOCK END 13
    
    }
    
    function handle_comment(){
        /// --- BLOCK BEGIN 14
var j = i;
var comment = "";
try {
    while (sl[j] !== '\n') {
        comment += sl[j];
        sl[j] = ' ';
        j++;
    }
} catch (error) {
    if (error instanceof RangeError) {
        return "break";
    }
}
if (!openarr) {
    decoder.preserve_comment(line_no, prev_key, comment, beginline);
}    
        /// --- BLOCK END 14
    
    }
    
    function handle_backslash(){
        /// --- BLOCK BEGIN 15
        if (item === '\n') {
            if (openstring || multilinestr) {
                if (!multilinestr) {
                    throw new TomlDecodeError("Unbalanced quotes", original, i);
                }
                if ((sl[i - 1] === "'" || sl[i - 1] === '"') && (sl[i - 2] === sl[i - 1])) {
                    sl[i] = sl[i - 1];
                    if (sl[i - 3] === sl[i - 1]) {
                        sl[i - 3] = ' ';
                    }
                }
            } else if (openarr) {
                sl[i] = ' ';
            } else {
                beginline = true;
            }
            line_no++;
        } else if (beginline && sl[i] !== ' ' && sl[i] !== '\t') {
            beginline = false;
            if (!keygroup && !arrayoftables) {
                if (sl[i] === '=') {
                    throw new TomlDecodeError("Found empty keyname. ", original, i);
                }
                keyname = 1;
                key += item;
            }
        }    
        /// --- BLOCK END 15
    
    }
    
    function handle_bracket(){
        /// --- BLOCK BEGIN 16
        if (item === '[' && (!openstring && !keygroup &&
        !arrayoftables)) {
            if (beginline) {
                if (sl.length > i + 1 && sl[i + 1] === '[') {
                    arrayoftables = true;
                } else {
                    keygroup = true;
                }
            } else {
                openarr += 1;
            }
        }
        if (item === ']' && !openstring) {
            if (keygroup) {
                keygroup = false;
            } else if (arrayoftables) {
                if (sl[i - 1] === ']') {
                    arrayoftables = false;
                }
            } else {
                openarr -= 1;
            }
        }    
        /// --- BLOCK END 16
    
    }
    
    function handle_remaining(){
        function handle_multikey(){
            /// --- BLOCK BEGIN 17
            if (multibackslash) {
                multilinestr += line;
            } else {
                multilinestr += line;
            }
            multibackslash = false;
            var closed = false;
            if (multilinestr[0] === '[') {
                closed = line[line.length - 1] === ']';
            } else if (line.length > 2) {
                closed = (line[line.length - 1] === multilinestr[0] &&
                          line[line.length - 2] === multilinestr[0] &&
                          line[line.length - 3] === multilinestr[0]);
            }
            if (closed) {
                try {
                    var [value, vtype] = decoder.load_value(multilinestr, true);
                } catch (err) {
                    throw new TomlDecodeError(err.toString(), original, pos);
                }
                currentlevel[multikey] = value;
                multikey = null;
                multilinestr = "";
            } else {
                var k = multilinestr.length - 1;
                while (k > -1 && multilinestr[k] === '\\') {
                    multibackslash = !multibackslash;
                    k -= 1;
                }
                if (multibackslash) {
                    multilinestr = multilinestr.slice(0, -1);
                } else {
                    multilinestr += "\n";
                }
            }
            return "continue";        
            /// --- BLOCK END 17
        
        }
        
        function handle_start_bracket(){
            function handle_groupname(){
                /// --- BLOCK BEGIN 18
var i = 0;
while (i < groups.length) {
    groups[i] = groups[i].trim();
    if (groups[i].length > 0 && (groups[i][0] === '"' || groups[i][0] === "'")) {
        var groupstr = groups[i];
        var j = i + 1;
        while ((groupstr[0] !== groupstr[groupstr.length - 1]) || groupstr.length === 1) {
            j++;
            if (j > groups.length + 2) {
                throw new TomlDecodeError("Invalid group name '" + groupstr + "' Something went wrong.", original, pos);
            }
            groupstr = groups.slice(i, j).join('.').trim();
        }
        groups[i] = groupstr.substring(1, groupstr.length - 1);
        groups.splice(i + 1, j - (i + 1));
    } else {
        if (!_groupname_re.test(groups[i])) {
            throw new TomlDecodeError("Invalid group name '" + groups[i] + "'. Try quoting it.", original, pos);
        }
    }
    i++;
}            
                /// --- BLOCK END 18
            
            }
            
            /// --- BLOCK BEGIN 19
arrayoftables = false;
    if (line.length === 1) {
        throw new Error("Opening key group bracket on line by itself.");
    }

    if (line[1] === '[') {
        arrayoftables = true;
        line = line.substring(2);
        splitstr = ']]';
    } else {
        line = line.substring(1);
        splitstr = ']';
    }

    var i = 1;
    var quotesplits = decoder['_get_split_on_quotes'](line);
    var quoted = false;
    for (var quotesplit of quotesplits) {
        if (!quoted && quotesplit.includes(splitstr)) {
            break;
        }
        i += (quotesplit.match(new RegExp(splitstr, "g")) || []).length;
        quoted = !quoted;
    }

    line = self_split(line, splitstr, i);

    if (line.length < i + 1 || line[line.length - 1].trim() !== "") {
        throw new Error("Key group not on a line by itself.");
    }
    var groups = line.slice(0, -1).join(splitstr).split('.');
    handle_groupname()
    currentlevel = retval;
    for (i = 0; i < groups.length; i++) {
        var group = groups[i];
        if (group === "") {
            throw new Error("Can't have a keygroup with an empty name");
        }
        try {
            if (currentlevel.constructor.name === 'Array' && isNaN(parseInt(group))) {
                throw new TypeError("abc");
            }
            if (!currentlevel.hasOwnProperty(group)){
                throw new RangeError("abc")
            }
            if (i === groups.length - 1) {
                if (implicitgroups.includes(group)) {
                    implicitgroups.splice(implicitgroups.indexOf(group), 1);
                    if (arrayoftables) {
                        throw new Error("An implicitly defined table can't be an array");
                    }
                } else if (arrayoftables) {
                    currentlevel[group].push(decoder['get_empty_table']());
                } else {
                    throw new Error("What? " + group + " already exists?" + JSON.stringify(currentlevel));
                }
            }
        } catch (error) {
            if (error instanceof TypeError) {
                currentlevel = currentlevel[currentlevel.length - 1];
                if (!(group in currentlevel)) {
                    currentlevel[group] = decoder['get_empty_table']();
                    if (i === groups.length - 1 && arrayoftables) {
                        currentlevel[group] = [decoder['get_empty_table']()];
                    }
                }
            } else if (error instanceof RangeError) {
                if (i !== groups.length - 1) {
                    implicitgroups.push(group);
                }
                currentlevel[group] = decoder['get_empty_table']();
                if (i === groups.length - 1 && arrayoftables) {
                    currentlevel[group] = [decoder['get_empty_table']()];
                }
            }
        }
        currentlevel = currentlevel[group];
        if (arrayoftables) {
            try {
                currentlevel = currentlevel[currentlevel.length - 1];
            } catch (KeyError) {
                // pass
            }
        }
    }
        
            /// --- BLOCK END 19
        
        }
        
        /// --- BLOCK BEGIN 20
        var s = sl.join('');
        s = s.split('\n');
        var multikey = null;
        var multilinestr = "";
        var multibackslash = false;
        var pos = 0;
        for (var idx = 0; idx < s.length; idx++) {
            var line = s[idx];
            if (idx > 0) {
                pos += s[idx - 1].length + 1;
            }
            decoder.embed_comments(idx, currentlevel);
            if (!multilinestr || multibackslash || !multilinestr.includes('\n')) {
                line = line.trim();
            }
            if (line === "" && (!multikey || multibackslash)) {
                continue;
            }
            if (multikey) {
                var act = handle_multikey();
                if (act === "continue") {
                    continue;
                }
            }
            if (line[0] === '[') {
                handle_start_bracket();
            } else if (line[0] === "{") {
                if (line[line.length - 1] !== "}") {
                    throw new TomlDecodeError("Line breaks are not allowed in inline objects", original, pos);
                }
                try {
                    decoder.load_inline_object(line, currentlevel, multikey, multibackslash);
                } catch (err) {
                    throw new TomlDecodeError(err.toString(), original, pos);
                }
            } else if (line.includes("=")) {
                try {
                    var ret = decoder.load_line(line, currentlevel, multikey, multibackslash);
                } catch (err) {
                    throw new TomlDecodeError(err.toString(), original, pos);
                }
                if (ret !== null) {
                    multikey = ret[0];
                    multilinestr = ret[1];
                    multibackslash = ret[2];
                }
            }
        }
        return retval;    
        /// --- BLOCK END 20
    
    }
    
    /// --- BLOCK BEGIN 21
var implicitgroups = [];
    if (decoder === null) {
        decoder = new TomlDecoder(_dict);
    }
    var retval = decoder.get_empty_table();
    var currentlevel = retval;
    if (typeof s !== 'string') {
        throw new TypeError("Expecting something like a string");
    }
    var original = s;
    var sl = s.split('');
    var openarr = 0;
    var openstring = false;
    var openstrchar = "";
    var multilinestr = false;
    var arrayoftables = false;
    var beginline = true;
    var keygroup = false;
    var dottedkey = false;
    var keyname = 0;
    var key = '';
    var prev_key = '';
    var line_no = 1;
    for (var i = 0; i < sl.length; i++) {
        var item = sl[i];
        if (item === '\r' && sl.length > (i + 1) && sl[i + 1] === '\n') {
            sl[i] = ' ';
            continue;
        }
        if (keyname) {
            var act = handle_keyname();
            if (act === "continue") {
                continue;
            }
        }
        if (item === "'" && openstrchar !== '"') {
            handle_single_quote_1();
        }
        if (item === '"' && openstrchar !== "'") {
            handle_single_quote_2();
        }
        if (item === '#' && (!openstring && !keygroup && !arrayoftables)) {
            act = handle_comment();
            if (act === "break") {
                break;
            }
        }
        handle_bracket();
        handle_backslash();
    }
    if (keyname) {
        throw new TomlDecodeError("Key name found without value. Reached end of file.", original, s.length);
    }
    if (openstring) {  // reached EOF and have an unterminated string
        throw new TomlDecodeError("Unterminated string found. Reached end of file.", original, s.length);
    }
    return handle_remaining();

    /// --- BLOCK END 21

}

function _load_date(val){
    /// --- BLOCK BEGIN 22
var microsecond = 0;
var tz = null;
try {
    if (val.length > 19) {
        if (val[19] === '.') {
            var subsecondval, tzval;
            if (val[val.length - 1].toUpperCase() === 'Z') {
                subsecondval = val.substring(20, val.length - 1);
                tzval = "Z";
            } else {
                var subsecondvalandtz = val.substring(20);
                var splitpoint;
                if (subsecondvalandtz.includes('+')) {
                    splitpoint = subsecondvalandtz.indexOf('+');
                    subsecondval = subsecondvalandtz.substring(0, splitpoint);
                    tzval = subsecondvalandtz.substring(splitpoint);
                } else if (subsecondvalandtz.includes('-')) {
                    splitpoint = subsecondvalandtz.indexOf('-');
                    subsecondval = subsecondvalandtz.substring(0, splitpoint);
                    tzval = subsecondvalandtz.substring(splitpoint);
                } else {
                    tzval = null;
                    subsecondval = subsecondvalandtz;
                }
            }
            if (tzval !== null) {
                tz = new TomlTz(tzval);
            }
            microsecond = parseInt(parseInt(subsecondval) * Math.pow(10, (6 - subsecondval.length)));
        } else {
            tz = new TomlTz(val.substring(19).toUpperCase());
        }
    }
} catch (e) {
    tz = null;
}
if (!val.substring(1).includes("-")) {
    return null;
}
var d = null;
try {
    if (val.length === 10) {
        d = new Date(Date.UTC(
            parseInt(val.substring(0, 4)), parseInt(val.substring(5, 7)) - 1,
            parseInt(val.substring(8, 10))));
    } else {
        d = new Date(Date.UTC(
            parseInt(val.substring(0, 4)), parseInt(val.substring(5, 7)) - 1,
            parseInt(val.substring(8, 10)), parseInt(val.substring(11, 13)),
            parseInt(val.substring(14, 16)), parseInt(val.substring(17, 19)), microsecond));
        // if (tz !== null) {
        //     d = new Date(d.getTime() + tz.getOffset(d));
        // }
    }
    d.tz = tz;
    if (isNaN(d)) {
        throw new Error("Invalid date");
    }
} catch (e) {
    return null;
}
return d;

    /// --- BLOCK END 22

}

function _load_unicode_escapes(v, hexbytes, prefix){
    /// --- BLOCK BEGIN 23
var skip = false;
    var i = v.length - 1;
    while (i > -1 && v[i] === '\\') {
        skip = !skip;
        i -= 1;
    }
    for (var hx of hexbytes) {
        if (skip) {
            skip = false;
            i = hx.length - 1;
            while (i > -1 && hx[i] === '\\') {
                skip = !skip;
                i -= 1;
            }
            v += prefix;
            v += hx;
            continue;
        }
        var hxb = "";
        i = 0;
        var hxblen = 4;
        if (prefix === "\\U") {
            hxblen = 8;
        }
        hxb = hx.substring(i, i + hxblen).toLowerCase();
        if (/[^0123456789abcdef]/.test(hxb)) {
            throw new Error("Invalid escape sequence: " + hxb);
        }
        if (hxb[0] === "d" && /[^01234567]/.test(hxb[1])) {
            throw new Error("Invalid escape sequence: " + hxb +
            ". Only scalar unicode points are allowed.");
        }
        v += unichr(parseInt(hxb, 16));
        v += hx.substring(hxb.length);
    }
    return v;

    /// --- BLOCK END 23

}

function _unescape(v){
    /// --- BLOCK BEGIN 24
var i = 0;
    var backslash = false;
    while (i < v.length) {
        if (backslash) {
            backslash = false;
            if (_escapes.includes(v[i])) {
                v = v.substring(0, i - 1) + _escape_to_escapedchars[v[i]] + v.substring(i + 1);
            } else if (v[i] === '\\') {
                v = v.substring(0, i - 1) + v.substring(i);
            } else if (v[i] === 'u' || v[i] === 'U') {
                i += 1;
            } else {
                throw new Error("Reserved escape sequence used");
            }
            continue;
        } else if (v[i] === '\\') {
            backslash = true;
        }
        i += 1;
    }
    return v;

    /// --- BLOCK END 24

}

function InlineTableDict(...args){
    var class_var = SkelClass('InlineTableDict');
    return class_var;
}


function DynamicInlineTableDict(...args){
    var class_var = {};
    return class_var;
}


function TomlDecoder(param_0){
    function __init__(_dict){
        /// --- BLOCK BEGIN 25
class_var._dict = _dict;
return null;    
        /// --- BLOCK END 25
    
    }
    
    function get_empty_table(){
        /// --- BLOCK BEGIN 26
return class_var._dict();    
        /// --- BLOCK END 26
    
    }
    
    function get_empty_inline_table(){
        /// --- BLOCK BEGIN 27
return DynamicInlineTableDict();
    
        /// --- BLOCK END 27
    
    }
    
    function load_inline_object(line, currentlevel, multikey, multibackslash){
        /// --- BLOCK BEGIN 28
var candidate_groups = line.slice(1, -1).split(",");
var groups = [];
if (candidate_groups.length === 1 && !candidate_groups[0].trim()) {
    candidate_groups.pop();
}
while (candidate_groups.length > 0) {
    var candidate_group = candidate_groups.shift();
    var splitResult = candidate_group.split('=', 2);
    if (splitResult.length < 2) {
        throw new Error("Invalid inline table encountered");
    }
    var value = splitResult[1].trim();
    if ((value[0] === value[value.length - 1] && ("'\"".indexOf(value[0]) !== -1)) ||
        ('-0123456789'.indexOf(value[0]) !== -1) ||
        (value === 'true' || value === 'false') ||
        (value[0] === "[" && value[value.length - 1] === "]") ||
        (value[0] === '{' && value[value.length - 1] === '}')) {
        groups.push(candidate_group);
    } else if (candidate_groups.length > 0) {
        candidate_groups[0] = candidate_group + "," + candidate_groups[0];
    } else {
        throw new Error("Invalid inline table value encountered");
    }
}
for (var index = 0; index < groups.length; index++) {
    var group = groups[index];
    var status = class_var.load_line(group, currentlevel, multikey, multibackslash);
    if (status !== null) {
        break;
    }
}    
        /// --- BLOCK END 28
    
    }
    
    function _get_split_on_quotes(line){
        /// --- BLOCK BEGIN 29
var doublequotesplits = line.split('"');
    var quoted = false;
    var quotesplits = [];
    if (doublequotesplits.length > 1 && doublequotesplits[0].includes("'")) {
        var singlequotesplits = doublequotesplits[0].split("'");
        doublequotesplits = doublequotesplits.slice(1);
        while (singlequotesplits.length % 2 === 0 && doublequotesplits.length) {
            singlequotesplits[singlequotesplits.length - 1] += '"' + doublequotesplits[0];
            doublequotesplits = doublequotesplits.slice(1);
            if (singlequotesplits[singlequotesplits.length - 1].includes("'")) {
                singlequotesplits = singlequotesplits.slice(0, -1).concat(singlequotesplits[singlequotesplits.length - 1].split("'"));
            }
        }
        quotesplits = quotesplits.concat(singlequotesplits);
    }
    doublequotesplits.forEach(function(doublequotesplit) {
        if (quoted) {
            quotesplits.push(doublequotesplit);
        } else {
            quotesplits = quotesplits.concat(doublequotesplit.split("'"));
            quoted = !quoted;
        }
    });
    return quotesplits;
    
        /// --- BLOCK END 29
    
    }
    
    function load_line(line, currentlevel, multikey, multibackslash){
        /// --- BLOCK BEGIN 30
i = 1;
var quotesplits = class_var._get_split_on_quotes(line);
var quoted = false;
for (var quotesplit of quotesplits) {
    if (!quoted && quotesplit.includes('=')) {
        break;
    }
    i += (quotesplit.match(/=/g) || []).length;
    quoted = !quoted;
}
var pair = self_split(line, '=', i);
var strictly_valid = _strictly_valid_num(pair[pair.length - 1]);
if (_number_with_underscores.test(pair[pair.length - 1]) && pair[pair.length - 1][0] !== " ") {
    pair[pair.length - 1] = pair[pair.length - 1].replace(/_/g, '');
}

while (pair[pair.length - 1].length > 0 && (pair[pair.length - 1][0] !== ' ' && pair[pair.length - 1][0] !== '\t' &&
    pair[pair.length - 1][0] !== "'" && pair[pair.length - 1][0] !== '"' &&
    pair[pair.length - 1][0] !== '[' && pair[pair.length - 1][0] !== '{' &&
    pair[pair.length - 1].trim() !== 'true' && pair[pair.length - 1].trim() !== 'false')) {

    if (!isNaN(parseFloat(pair[pair.length-1])) && ! pair[pair.length-1].includes("1979") && ! pair[pair.length-1].includes("=")) {
        break
    }

    if (_load_date(pair[pair.length - 1]) !== null) {
        break;
    }
    if (TIME_RE.test(pair[pair.length - 1])) {
        break;
    }
    i++;
    var prev_val = pair[pair.length - 1];
    pair = self_split(line, '=', i);

    if (prev_val === pair[pair.length - 1]) {
        throw new Error("Invalid date or number");
    }
    if (strictly_valid) {
        strictly_valid = _strictly_valid_num(pair[pair.length - 1]);
    }
}
pair = [pair.slice(0, -1).join('=').trim(), pair[pair.length - 1].trim()];
if (pair[0].includes('.')) {
    if (pair[0].includes('"') || pair[0].includes("'")) {
        quotesplits = class_var._get_split_on_quotes(pair[0]);
        quoted = false;
        var levels = [];
        for (quotesplit of quotesplits) {
            if (quoted) {
                levels.push(quotesplit);
            } else {
                levels = levels.concat(quotesplit.split('.').map(level => level.trim()));
            }
            quoted = !quoted;
        }
    } else {
        levels = pair[0].split('.').map(level => level.trim());
    }
    while (levels[levels.length - 1] === "") {
        levels.pop();
    }
    for (var level of levels.slice(0, -1)) {
        if (level === "") {
            continue;
        }
        if (!(level in currentlevel)) {
            currentlevel[level] = class_var.get_empty_table();
        }
        currentlevel = currentlevel[level];
    }
    pair[0] = levels[levels.length - 1];
} else if ((pair[0][0] === '"' || pair[0][0] === "'") && (pair[0][pair[0].length - 1] === pair[0][0])) {
    pair[0] = _unescape(pair[0].substring(1, pair[0].length - 1));
}
var k, koffset;
_argument0 = pair[1];
[k, koffset] = class_var._load_line_multiline_str(_argument0);

if (k > -1) {
    while (k > -1 && pair[1][k + koffset] === '\\') {
        multibackslash = !multibackslash;
        k--;
    }
    if (multibackslash) {
        var multilinestr = pair[1].slice(0, -1);
    } else {
        var multilinestr = pair[1] + "\n";
    }
    multikey = pair[0];
} else {
    var tmp;
    tmp = class_var.load_value(pair[1].replace(), strictly_valid);
    value = tmp[0];
    vtype = tmp[1];
}

    if (currentlevel.hasOwnProperty(pair[0])){
        throw new Error("Duplicate keys!");
    }
    else{
        if (multikey !== null && multikey !== false) {
            var _return_value = [multikey, multilinestr, multibackslash];
            return _return_value;
        } else {
            currentlevel[pair[0]] = value;
        }
    }

        var _return_value = null;
        return _return_value;
    
        /// --- BLOCK END 30
    
    }
    
    function _load_line_multiline_str(p){
        /// --- BLOCK BEGIN 31
var poffset = 0;
    if (p.length < 3) {
        return [-1, poffset];
    }
    if (p[0] === '[' && (p.trim().slice(-1) !== ']' &&
    class_var._load_array_isstrarray(p))) {
        var newp = p.slice(1).trim().split(',');
        while (newp.length > 1 && newp[newp.length - 1][0] !== '"' && newp[newp.length - 1][0] !== "'") {
            newp = newp.slice(0, -2).concat([newp[newp.length - 2] + ',' + newp[newp.length - 1]]);
        }
        newp = newp[newp.length - 1];
        poffset = p.length - newp.length;
        p = newp;
    }
    if (p[0] !== '"' && p[0] !== "'") {
        return [-1, poffset];
    }
    if (p[1] !== p[0] || p[2] !== p[0]) {
        return [-1, poffset];
    }
    if (p.length > 5 && p[p.length - 1] === p[0] && p[p.length - 2] === p[0] && p[p.length - 3] === p[0]) {
        return [-1, poffset];
    }
    return [p.length - 1, poffset];    
        /// --- BLOCK END 31
    
    }
    
    function load_value(v, strictly_valid){
        function handle_remaining(){
            /// --- BLOCK BEGIN 32
if (parsed_date !== null) {
    return [parsed_date, "date"];
}
if (!strictly_valid) {
    throw new Error("Weirdness with leading zeroes or underscores in your number.");
}
var itype = "int";
var neg = false;
if (v[0] === '-') {
    neg = true;
    v = v.substring(1);
} else if (v[0] === '+') {
    v = v.substring(1);
}
v = v.replace(/_/g, '');
var lowerv = v.toLowerCase();
if (v.includes('.') || (!v.includes('x') && (v.includes('e') || v.includes('E')))) {
    if (v.includes('.') && v.split('.', 2)[1] === '') {
        throw new Error("This float is missing digits after the point");
    }
    if (!'0123456789'.includes(v[0])) {
        throw new Error("This float doesn't have a leading digit");
    }
    v = parseFloat(v);
    itype = "float";
} else if (lowerv.length === 3 && (lowerv === 'inf' || lowerv === 'nan')) {
    v = parseFloat(v);
    itype = "float";
}
if (itype === "int") {
    v = parseInt(v, 0);
}
if (neg) {
    return [0 - v, itype];
}
return [v, itype];        
            /// --- BLOCK END 32
        
        }
        
        /// --- BLOCK BEGIN 33
        if (!v) {
            throw new Error("Empty value is invalid");
        }
        if (v === 'true') {
            var _return_value = [true, "bool"];
            return _return_value;
        } else if (v.toLowerCase() === 'true') {
            throw new Error("Only all lowercase booleans allowed");
        } else if (v === 'false') {
            var _return_value = [false, "bool"];
            return _return_value;
        } else if (v.toLowerCase() === 'false') {
            throw new Error("Only all lowercase booleans allowed");
        } else if (v[0] === '"' || v[0] === "'") {
            var quotechar = v[0];
            var testv = v.slice(1).split(quotechar);
            var triplequote = false;
            var triplequotecount = 0;
            if (testv.length > 1 && testv[0] === '' && testv[1] === '') {
                testv = testv.slice(2);
                triplequote = true;
            }
            var closed = false;
            for (var tv of testv) {
                if (tv === '') {
                    if (triplequote) {
                        triplequotecount += 1;
                    } else {
                        closed = true;
                    }
                } else {
                    var oddbackslash = false;
                    try {
                        var i = -1;
                        var j = tv[tv.length + i];
                        while (j === '\\') {
                            oddbackslash = !oddbackslash;
                            i -= 1;
                            j = tv[i];
                        }
                    } catch (error) {
                        // Ignore IndexError
                    }
                    if (!oddbackslash) {
                        if (closed) {
                            throw new Error("Found tokens after a closed string. Invalid TOML.");
                        } else {
                            if (!triplequote || triplequotecount > 1) {
                                closed = true;
                            } else {
                                triplequotecount = 0;
                            }
                        }
                    }
                }
            }
    
            if (quotechar === '"') {
                var escapeseqs = v.split('\\').slice(1);
                var backslash = false;
                for (var i of escapeseqs) {
                    if (i === '') {
                        backslash = !backslash;
                    } else {
                        if (!_escapes.includes(i[0]) && (i[0] !== 'u' && i[0] !== 'U' && !backslash)) {
                            throw new Error("Reserved escape sequence used");
                        }
                        if (backslash) {
                            backslash = false;
                        }
                    }
                }
                for (var prefix of ["\\u", "\\U"]) {
                    if (v.includes(prefix)) {
                        var hexbytes = v.split(prefix);
                        v = _load_unicode_escapes(hexbytes[0], hexbytes.slice(1), prefix);
                    }
                }
                v = _unescape(v); // Assuming _unescape is similar to unescape
            }
            if (v.length > 1 && v[1] === quotechar && (v.length < 3 || v[1] === v[2])) {
                v = v.slice(2, -2);
            }
            var _return_value = [v.slice(1, -1), "str"];
            return _return_value;
        } else if (v[0] === '[') {
            var _return_value = [load_array(v), "array"]; // Assuming load_array is defined
            return _return_value;
        } else if (v[0] === '{') {
            var inline_object = get_empty_inline_table(); // Assuming get_empty_inline_table is defined
            load_inline_object(v, inline_object, false, false); // Assuming load_inline_object is defined
            var _return_value = [inline_object, "inline_object"];
            return _return_value;
        } else if (TIME_RE.test(v)) {
            var matches = TIME_RE.exec(v);
            var h = matches[1], m = matches[2], s = matches[3], ms = matches[5] || 0;
            var time = new Date(0, 0, 0, h, m, s, ms);
            var _return_value = [time, "time"];
            return _return_value;
        } else {
            var parsed_date = _load_date(v);
            return handle_remaining();
        }
    
        /// --- BLOCK END 33
    
    }
    
    function bounded_string(s){
        /// --- BLOCK BEGIN 34
if (s.length === 0) {
    return true;
}
if (s[s.length - 1] !== s[0]) {
    return false;
}
var i = -2;
var backslash = false;
while (s.length + i > 0) {
    if (s[s.length + i] === "\\") {
        backslash = !backslash;
        i -= 1;
    } else {
        break;
    }
}
return !backslash;    
        /// --- BLOCK END 34
    
    }
    
    function _load_array_isstrarray(a){
        /// --- BLOCK BEGIN 35
a = a.slice(1, -1).trim();
if (a !== '' && (a[0] === '"' || a[0] === "'")) {
    return true;
}
return false;    
        /// --- BLOCK END 35
    
    }
    
    function load_array(a){
        /// --- BLOCK BEGIN 36
        var retval = [];
        a = a.trim();
        if (!a.slice(1, -1).includes('[') || a.slice(1, -1).split('[')[0].trim() !== "") {
            var strarray = class_var['_load_array_isstrarray'](a);
            if (!a.slice(1, -1).trim().startsWith('{')) {
                a = a.slice(1, -1).split(',');
            } else {
                var new_a = [];
                var start_group_index = 1;
                var end_group_index = 2;
                var open_bracket_count = a[start_group_index] === '{' ? 1 : 0;
                var in_str = false;
                while (end_group_index < a.slice(1).length) {
                    if (a[end_group_index] === '"' || a[end_group_index] === "'") {
                        if (in_str) {
                            var backslash_index = end_group_index - 1;
                            while (backslash_index > -1 && a[backslash_index] === '\\') {
                                in_str = !in_str;
                                backslash_index -= 1;
                            }
                        }
                        in_str = !in_str;
                    }
                    if (!in_str && a[end_group_index] === '{') {
                        open_bracket_count += 1;
                    }
                    if (in_str || a[end_group_index] !== '}') {
                        end_group_index += 1;
                        continue;
                    } else if (a[end_group_index] === '}' && open_bracket_count > 1) {
                        open_bracket_count -= 1;
                        end_group_index += 1;
                        continue;
                    }
    
                    end_group_index += 1;
    
                    new_a.push(a.slice(start_group_index, end_group_index));
    
                    start_group_index = end_group_index + 1;
                    while (start_group_index < a.slice(1).length && a[start_group_index] !== '{') {
                        start_group_index += 1;
                    }
                    end_group_index = start_group_index + 1;
                }
                a = new_a;
            }
            var b = 0;
            if (strarray) {
                while (b < a.length - 1) {
                    var ab = a[b].trim();
                    while (!class_var['bounded_string'](ab) || (ab.length > 2 && ab[0] === ab[1] === ab[2] && ab[-2] !== ab[0] && ab[-3] !== ab[0])) {
                        a[b] = a[b] + ',' + a[b + 1];
                        ab = a[b].trim();
                        if (b < a.length - 2) {
                            a = a.slice(0, b + 1).concat(a.slice(b + 2));
                        } else {
                            a = a.slice(0, b + 1);
                        }
                    }
                    b += 1;
                }
            }
        } else {
            var al = Array.from(a.slice(1, -1));
            a = [];
            var openarr = 0;
            var j = 0;
            for (var i = 0; i < al.length; i++) {
                if (al[i] === '[') {
                    openarr += 1;
                } else if (al[i] === ']') {
                    openarr -= 1;
                } else if (al[i] === ',' && !openarr) {
                    a.push(al.slice(j, i).join(''));
                    j = i + 1;
                }
            }
            a.push(al.slice(j).join(''));
        }
        for (var i = 0; i < a.length; i++) {
            a[i] = a[i].trim();
            if (a[i] !== '') {
                var [nval, ntype] = class_var['load_value'](a[i], true);
                retval.push(nval);
            }
        }
        var _return_value = retval;
        return _return_value;
    
        /// --- BLOCK END 36
    
    }
    
    function preserve_comment(line_no, key, comment, beginline){
        /// --- BLOCK BEGIN 37
        return null;    
        /// --- BLOCK END 37
    
    }
    
    function embed_comments(idx, currentlevel){
        /// --- BLOCK BEGIN 38
        return null;    
        /// --- BLOCK END 38
    
    }
    
    var class_var = SkelClass('TomlDecoder');
    class_var.__init__ = __init__;
    class_var.get_empty_table = get_empty_table;
    class_var.get_empty_inline_table = get_empty_inline_table;
    class_var.load_inline_object = load_inline_object;
    class_var._get_split_on_quotes = _get_split_on_quotes;
    class_var.load_line = load_line;
    class_var._load_line_multiline_str = _load_line_multiline_str;
    class_var.load_value = load_value;
    class_var.bounded_string = bounded_string;
    class_var._load_array_isstrarray = _load_array_isstrarray;
    class_var.load_array = load_array;
    class_var.preserve_comment = preserve_comment;
    class_var.embed_comments = embed_comments;
    __init__(param_0);
    return class_var;
}


function TomlPreserveCommentDecoder(param_0){
    function __init__(_dict){
        /// --- BLOCK BEGIN 39
class_var.saved_comments = {};

return null;    
        /// --- BLOCK END 39
    
    }
    
    function preserve_comment(line_no, key, comment, beginline){
        /// --- BLOCK BEGIN 40
class_var.saved_comments[line_no] = [key, comment, beginline];    
        /// --- BLOCK END 40
    
    }
    
    function embed_comments(idx, currentlevel){
        /// --- BLOCK BEGIN 41
if (!(idx in class_var.saved_comments)) {
    return null;
}
var [key, comment, beginline] = class_var.saved_comments[idx];
currentlevel[key] = CommentValue(currentlevel[key], comment, beginline, class_var._dict);
    
        /// --- BLOCK END 41
    
    }
    
    var class_var = TomlDecoder(param_0);
    class_var._class_name = 'TomlPreserveCommentDecoder;' + class_var._class_name;
    class_var.__init__ = __init__;
    class_var.preserve_comment = preserve_comment;
    class_var.embed_comments = embed_comments;
    __init__(param_0);
    return class_var;
}


function dump(o, f, encoder){
    /// --- BLOCK BEGIN 42
if (!f.write) {
        throw new TypeError("You can only dump an object to a file descriptor");
    }
    var d = JSON.stringify(o, encoder);
    f.write(d);
    return d;

    /// --- BLOCK END 42

}

function dumps(o, encoder){
    /// --- BLOCK BEGIN 43
var retval = "";
    if (encoder === null) {
        encoder = new TomlEncoder(o.constructor, false);
    }
    var [addtoretval, sections] = encoder.dump_sections(o, "");
    retval += addtoretval;
    while (Object.keys(sections).length > 0) {
        var newsections = encoder.get_empty_table();
        for (var section in sections) {
            var [addtoretval, addtosections] = encoder.dump_sections(sections[section], section);
            if (addtoretval || (!addtoretval && Object.keys(addtosections).length === 0)) {
                if (retval && retval.slice(-2) !== "\n\n") {
                    retval += "\n";
                }
                retval += "[" + section + "]\n";
                if (addtoretval) {
                    retval += addtoretval;
                }
            }
            for (var s in addtosections) {
                newsections[section + "." + s] = addtosections[s];
            }
        }
        sections = newsections;
    }
    return retval;
    /// --- BLOCK END 43

}

function _dump_str(v){
    /// --- BLOCK BEGIN 44
    v = JSON.stringify(v);
    if (v[0] === 'u') {
        v = v.substring(1);
    }
    var singlequote = v.startsWith("'") || v.startsWith('`');
    if (singlequote || v.startsWith('"')) {
        v = v.substring(1, v.length - 1);
    }
    if (singlequote) {
        v = v.replace(/\\'/g, "'");
        v = v.replace(/"/g, '\\"');
    }
    v = v.split("\\x");
    while (v.length > 1) {
        var i = -1;
        if (!v[0]) {
            v = v.slice(1);
        }
        v[0] = v[0].replace(/\\\\/g, "\\");
        var joinx = v[0][v[0].length + i] !== "\\";
        while (v[0].slice(0, i) && v[0][v[0].length + i] === "\\") {
            joinx = !joinx;
            i -= 1;
        }
        var joiner = joinx ? "x" : "u00";
        v = [v[0] + joiner + v[1]].concat(v.slice(2));
    }
    return '"' + v[0] + '"';

    /// --- BLOCK END 44

}

function _dump_float(v){
    /// --- BLOCK BEGIN 45
    if (v === Infinity) {
        var _return_value = "inf";
        return _return_value;
    }
    if (v === 1000000) {
        var _return_value = v.toString() + ".0";
        return _return_value;
    }
    var _return_value = v.toString().replace("e+0", "e+").replace("e-0", "e-");
    return _return_value;

    /// --- BLOCK END 45

}

function _dump_time(v){
    /// --- BLOCK BEGIN 46
var utcoffset = v.utcoffset();
    if (utcoffset === null) {
        return v.toISOString();
    }
    // The TOML norm specifies that it's local time thus we drop the offset
    return v.toISOString().slice(0, -6);

    /// --- BLOCK END 46

}

function _dump_bool(v){
    /// --- BLOCK BEGIN 47
    return String(v).toLowerCase();
    /// --- BLOCK END 47

}

function _dump_int(v){
    /// --- BLOCK BEGIN 48
return v;
    /// --- BLOCK END 48

}

function _dump_datetime(v){
    /// --- BLOCK BEGIN 49
return v.toISOString().replace('+00:00', 'Z');
    /// --- BLOCK END 49

}

function _dump_date(v){
    /// --- BLOCK BEGIN 50
    return v.toISOString();
    /// --- BLOCK END 50

}

function TomlEncoder(param_0, param_1){
    function __init__(_dict, preserve){
        /// --- BLOCK BEGIN 51
class_var._dict = _dict;
class_var.preserve = preserve;
class_var.dump_funcs = {
    "str": _dump_str,
    "list": class_var.dump_list,
    "bool": _dump_bool,
    "int": _dump_int,
    "float": _dump_float,
};    
        /// --- BLOCK END 51
    
    }
    
    function get_empty_table(){
        /// --- BLOCK BEGIN 52
return class_var._dict();    
        /// --- BLOCK END 52
    
    }
    
    function dump_list(v){
        /// --- BLOCK BEGIN 53
var retval = "[";
    for (var u of v) {
        retval += " " + class_var.dump_value(u) + ",";
    }
    retval += "]";
    return retval;    
        /// --- BLOCK END 53
    
    }
    
    function dump_inline_table(section){
        /// --- BLOCK BEGIN 54
var retval = "";
    if (section instanceof Object && !Array.isArray(section)) {
        var val_list = [];
        for (var k in section) {
            if (section.hasOwnProperty(k)) {
                var v = section[k];
                var val = class_var.dump_inline_table(v);
                val_list.push(k + " = " + val);
            }
        }
        retval += "{ " + val_list.join(", ") + " }\n";
        return retval;
    } else {
        return String(class_var.dump_value(section));
    }
    
        /// --- BLOCK END 54
    
    }
    
    function dump_value(v){
        /// --- BLOCK BEGIN 55
var dump_fn = null;
for (var t in class_var.dump_funcs) {
    if (user_check_type(v, t)) {
        dump_fn = class_var.dump_funcs[t];
        break;
    }
}
if (dump_fn === null && v !== null && typeof v === 'object' && typeof v[Symbol.iterator] === 'function') {
    dump_fn = class_var.dump_funcs['list'];
}
if (dump_fn === null) {
    dump_fn = class_var.dump_funcs['str'];
}

return (typeof dump_fn === 'function') ? dump_fn(v) : class_var.dump_funcs['str'](v);
    
        /// --- BLOCK END 55
    
    }
    
    function dump_sections(o, sup){
        /// --- BLOCK BEGIN 56
        var retstr = "";
        if (sup !== "" && sup.slice(-1) !== ".") {
            sup += '.';
        }
        var retdict = class_var._dict();
        var arraystr = "";
        for (var section in o) {
            section = String(section);
                var qsection = section;
                if (!/^[A-Za-z0-9_-]+$/.test(section)) {
                    qsection = _dump_str(section);
                }
        
                if (o[section]._class_name === "CommentValue" || (!(o[section] instanceof Object) || o[section].constructor.name === "Date") || Array.isArray(o[section])) {
                    var arrayoftables = false;
                    if (Array.isArray(o[section])) {
                        for (var a of o[section]) {
                            if (a instanceof Object && !Array.isArray(a)) {
                                arrayoftables = true;
                            }
                        }
                    }
        
                    if (arrayoftables) {
                        for (var a of o[section]) {
                            var arraytabstr = "\n";
                            arraystr += "[[" + sup + qsection + "]]\n";
                            var [s, d] = class_var.dump_sections(a, sup + qsection);
                            if (s) {
                                if (s[0] === "[") {
                                    arraytabstr += s;
                                } else {
                                    arraystr += s;
                                }
                            }
                            while (Object.keys(d).length !== 0) {
                                var newd = class_var._dict();
                                for (var dsec in d) {
                                    var [s1, d1] = class_var.dump_sections(d[dsec], sup + qsection + "." + dsec);
                                    if (s1) {
                                        arraytabstr += ("[" + sup + qsection + "." + dsec + "]\n");
                                        arraytabstr += s1;
                                    }
                                    for (var s1 in d1) {
                                        newd[dsec + "." + s1] = d1[s1];
                                    }
                                }
                                d = newd;
                            }
                            arraystr += arraytabstr;
                        }
                    } else {
                        if (o[section] !== null) {
                            retstr += (qsection + " = " + String(class_var.dump_value(o[section])) + '\n');
                        }
                    }
                } else if (class_var.preserve && (o[section] instanceof Object)) {
                    retstr += (qsection + " = " + class_var.dump_inline_table(o[section]));
                } else {
                    retdict[qsection] = o[section];
                }
        }
        retstr += arraystr;
        return [retstr, retdict];
    
        /// --- BLOCK END 56
    
    }
    
    var class_var = SkelClass('TomlEncoder');
    class_var.__init__ = __init__;
    class_var.get_empty_table = get_empty_table;
    class_var.dump_list = dump_list;
    class_var.dump_inline_table = dump_inline_table;
    class_var.dump_value = dump_value;
    class_var.dump_sections = dump_sections;
    __init__(param_0, param_1);
    return class_var;
}


function TomlPreserveInlineDictEncoder(param_0){
    function __init__(_dict){
        /// --- BLOCK BEGIN 57
        
        // pass    
        /// --- BLOCK END 57
    
    }
    
    var class_var = TomlEncoder(param_0, true);
    class_var._class_name = 'TomlPreserveInlineDictEncoder;' + class_var._class_name;
    class_var.__init__ = __init__;
    __init__(param_0);
    return class_var;
}


function TomlArraySeparatorEncoder(param_0, param_1, param_2){
    function __init__(_dict, preserve, separator){
        /// --- BLOCK BEGIN 58
if (separator.trim() === "") {
    separator = "," + separator;
} else if (separator.trim().replace(/[\s,]/g, '')) {
    throw new Error("Invalid separator for arrays");
}
class_var.separator = separator;    
        /// --- BLOCK END 58
    
    }
    
    function dump_list(v){
        /// --- BLOCK BEGIN 59
var t = [];
    var retval = "[";
    for (var u of v) {
        t.push(class_var.dump_value(u));
    }
    while (t.length !== 0) {
        var s = [];
        for (var u of t) {
            if (Array.isArray(u)) {
                for (var r of u) {
                    s.push(r);
                }
            } else {
                retval += " " + String(u) + class_var.separator;
            }
        }
        t = s;
    }
    retval += "]";
    return retval;
    
        /// --- BLOCK END 59
    
    }
    
    var class_var = TomlEncoder(param_0, param_1);
    class_var._class_name = 'TomlArraySeparatorEncoder;' + class_var._class_name;
    class_var.__init__ = __init__;
    class_var.dump_list = dump_list;
    __init__(param_0, param_1, param_2);
    return class_var;
}


function TomlNumpyEncoder(param_0, param_1){
    function __init__(_dict, preserve){
        /// --- BLOCK BEGIN 60
class_var.dump_funcs["float16"] = _dump_float;
class_var.dump_funcs["float32"] = _dump_float;
class_var.dump_funcs["float64"] = _dump_float;
class_var.dump_funcs["int16"] = class_var._dump_int;
class_var.dump_funcs["int32"] = class_var._dump_int;
class_var.dump_funcs["int64"] = class_var._dump_int;    
        /// --- BLOCK END 60
    
    }
    
    function _dump_int(v){
        /// --- BLOCK BEGIN 61
return v.toString();    
        /// --- BLOCK END 61
    
    }
    
    var class_var = TomlEncoder(param_0, param_1);
    class_var._class_name = 'TomlNumpyEncoder;' + class_var._class_name;
    class_var.__init__ = __init__;
    class_var._dump_int = _dump_int;
    __init__(param_0, param_1);
    return class_var;
}


function TomlPreserveCommentEncoder(param_0, param_1){
    function __init__(_dict, preserve){
        /// --- BLOCK BEGIN 62
class_var.dump_funcs["CommentValue"] = function(v) { return v.dump(class_var.dump_value); };    
        /// --- BLOCK END 62
    
    }
    
    var class_var = TomlEncoder(param_0, param_1);
    class_var._class_name = 'TomlPreserveCommentEncoder;' + class_var._class_name;
    class_var.__init__ = __init__;
    __init__(param_0, param_1);
    return class_var;
}


function TomlPathlibEncoder(...args){
    function _dump_pathlib_path(v){
        /// --- BLOCK BEGIN 63
return _dump_str(String(v));    
        /// --- BLOCK END 63
    
    }
    
    function dump_value(v){
        /// --- BLOCK BEGIN 64
if (Number(process.versions.node.split('.')[0]) >= 12) {
    pathlib = require('path');
    if (user_check_type(v, pathlib.PurePath)) {
        v = String(v);
    }
}
return TomlEncoder(TomlPathlibEncoder, class_var).dump_value(v);    
        /// --- BLOCK END 64
    
    }
    
    var class_var = TomlEncoder(...args);
    class_var._class_name = 'TomlPathlibEncoder;' + class_var._class_name;
    class_var._dump_pathlib_path = _dump_pathlib_path;
    class_var.dump_value = dump_value;
    return class_var;
}


function TomlOrderedDecoder(){
    function __init__(){
        /// --- BLOCK BEGIN 65
        
        // pass    
        /// --- BLOCK END 65
    
    }
    
    var class_var = TomlDecoder('Error: Type not support');
    class_var._class_name = 'TomlOrderedDecoder;' + class_var._class_name;
    class_var.__init__ = __init__;
    __init__();
    return class_var;
}


function TomlOrderedEncoder(){
    function __init__(){
        /// --- BLOCK BEGIN 66
        
        // pass    
        /// --- BLOCK END 66
    
    }
    
    var class_var = TomlEncoder('Error: Type not support');
    class_var._class_name = 'TomlOrderedEncoder;' + class_var._class_name;
    class_var.__init__ = __init__;
    __init__();
    return class_var;
}


function TomlTz(param_0){
    function __init__(toml_offset){
        /// --- BLOCK BEGIN 67
if (toml_offset === "Z") {
    class_var._raw_offset = "+00:00";
} else {
    class_var._raw_offset = toml_offset;
}
class_var._sign = class_var._raw_offset[0] === '-' ? -1 : 1;
class_var._hours = parseInt(class_var._raw_offset.substring(1, 3));
if (class_var._raw_offset.substring(1, 3).includes("_")) {
    throw new Error("invalid literal for int() with base 10: '" + class_var._raw_offset.substring(1, 3) + "'");
}
class_var._minutes = parseInt(class_var._raw_offset.substring(4, 6));
if (class_var._raw_offset.substring(4, 6).includes("_")) {
    throw new Error("invalid literal for int() with base 10: '" + class_var._raw_offset.substring(4, 6) + "'");
}
    
        /// --- BLOCK END 67
    
    }
    
    function __getinitargs__(){
        /// --- BLOCK BEGIN 68
return [class_var._raw_offset];    
        /// --- BLOCK END 68
    
    }
    
    function __deepcopy__(memo){
        /// --- BLOCK BEGIN 69
return new class_var.constructor(class_var._raw_offset);    
        /// --- BLOCK END 69
    
    }
    
    function tzname(dt){
        /// --- BLOCK BEGIN 70
return "UTC" + class_var._raw_offset;    
        /// --- BLOCK END 70
    
    }
    
    function utcoffset(dt){
        /// --- BLOCK BEGIN 71
return class_var._sign * (class_var._hours * 3600000 + class_var._minutes * 60000);    
        /// --- BLOCK END 71
    
    }
    
    function dst(dt){
        /// --- BLOCK BEGIN 72
return 0;    
        /// --- BLOCK END 72
    
    }
    
    var class_var = {};
    class_var.__init__ = __init__;
    class_var.__getinitargs__ = __getinitargs__;
    class_var.__deepcopy__ = __deepcopy__;
    class_var.tzname = tzname;
    class_var.utcoffset = utcoffset;
    class_var.dst = dst;
    __init__(param_0);
    return class_var;
}
