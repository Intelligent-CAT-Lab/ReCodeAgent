var {_example_html, _invalid_charrefs, _invalid_codepoints, _charref_regular_exp, _declname, _declstringlit, _commentclose, _markedsectionclose, _msmarkedsectionclose, interesting_normal, incomplete, entityref, charref, starttagopen, piclose, commentclose, tagfind_tolerant, attrfind_tolerant, locatestarttagend_tolerant, endendtag, endtagfind, _html5} = require('./tracer_skip.js');
var tool_functions = {"_example_html":_example_html, "_invalid_charrefs":_invalid_charrefs, "_invalid_codepoints":_invalid_codepoints, "_charref_regular_exp":_charref_regular_exp, "_declname":_declname, "_declstringlit":_declstringlit, "_commentclose":_commentclose, "_markedsectionclose":_markedsectionclose, "_msmarkedsectionclose":_msmarkedsectionclose, "interesting_normal":interesting_normal, "incomplete":incomplete, "entityref":entityref, "charref":charref, "starttagopen":starttagopen, "piclose":piclose, "commentclose":commentclose, "tagfind_tolerant":tagfind_tolerant, "attrfind_tolerant":attrfind_tolerant, "locatestarttagend_tolerant":locatestarttagend_tolerant, "endendtag":endendtag, "endtagfind":endtagfind, "_html5":_html5};


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

function escape(s, quote){
    /// --- BLOCK BEGIN 1
s = s.replace(/&/g, "&amp;");
    // Must be done first!
    s = s.replace(/</g, "&lt;");
    s = s.replace(/>/g, "&gt;");
    if (quote) {
        s = s.replace(/"/g, "&quot;");
        s = s.replace(/'/g, "&#x27;");
    }
    return s;

    /// --- BLOCK END 1

}

function _replace_charref(s){
    /// --- BLOCK BEGIN 2
if (s[0] === '#') {
    // numeric charref
    var num;
    if (s[1] === 'x' || s[1] === 'X') {
        num = parseInt(s.slice(2).replace(';', ''), 16);
    } else {
        num = parseInt(s.slice(1).replace(';', ''));
    }
    if (('contains' in tool_functions._invalid_charrefs && tool_functions._invalid_charrefs.contains(num)) || (!('contains' in tool_functions._invalid_charrefs) && num in tool_functions._invalid_charrefs)) {
        return tool_functions._invalid_charrefs[num];
    }
    if (0xD800 <= num && num <= 0xDFFF || num > 0x10FFFF) {
        return '\uFFFD';
    }
    if (('contains' in tool_functions._invalid_codepoints && tool_functions._invalid_codepoints.contains(num)) || (!('contains' in tool_functions._invalid_codepoints) && num in tool_functions._invalid_codepoints)) {
        return '';
    }
    return String.fromCharCode(num);
} else {
    // named charref
    if (('contains' in tool_functions._html5 && tool_functions._html5.contains(s)) || (!('contains' in tool_functions._html5) && s in tool_functions._html5)) {
        return tool_functions._html5[s];
    }
    // find the longest matching name (as defined by the standard)
    for (var x = s.length - 1; x > 1; x--) {
        if (('contains' in tool_functions._html5 && tool_functions._html5.contains(s.substring(0, x))) || (!('contains' in tool_functions._html5) && s.substring(0, x) in tool_functions._html5)) {
            return tool_functions._html5[s.substring(0, x)] + s.substring(x);
        }
    }
    return '&' + s;
}
    /// --- BLOCK END 2

}

function unescape(s){
    /// --- BLOCK BEGIN 3
    if (!s.includes('&')) {
        return s;
    }
    var start = 0;
    while (true) {
        var match = _charref_regular_exp.exec(s);
        if (!match) {
            break;
        }
        var replacement = _replace_charref(match[1]);
        s = s.substring(0, match.index) + replacement + s.substring(match.index + match[0].length);
        start = match.index + replacement.length;
        _charref_regular_exp.lastIndex = start;
    }
    return s;
    /// --- BLOCK END 3

}

function ParserBase(){
    function __init__(){
        /// --- BLOCK BEGIN 4
return null;    
        /// --- BLOCK END 4
    
    }
    
    function reset(){
        /// --- BLOCK BEGIN 5
class_var.lineno = 1;
class_var.offset = 0;    
        /// --- BLOCK END 5
    
    }
    
    function getpos(){
        /// --- BLOCK BEGIN 6
return [class_var.lineno, class_var.offset];    
        /// --- BLOCK END 6
    
    }
    
    function updatepos(i, j){
        /// --- BLOCK BEGIN 7
        if (i >= j) {
            return j;
        }
        var rawdata = class_var.rawdata;
        var nlines = (rawdata.substring(i, j).match(/\n/g) || []).length;
        if (nlines) {
            class_var.lineno += nlines;
            var pos = rawdata.lastIndexOf("\n", j);
            class_var.offset = j - (pos + 1);
        } else {
            class_var.offset += j - i;
        }
        return j;    
        /// --- BLOCK END 7
    
    }
    
    function parse_declaration(i){
        /// --- BLOCK BEGIN 8
var rawdata = class_var.rawdata;
var j = i + 2;
if (rawdata.substring(i, j) !== "<!") {
    throw new Error("unexpected call to parse_declaration");
}
if (rawdata.substring(j, j + 1) === ">") {
    // the empty comment <!>
    return j + 1;
}
if (["-", ""].includes(rawdata.substring(j, j + 1))) {
    // Start of comment followed by buffer boundary,
    // or just a buffer boundary.
    return -1;
}
// A simple, practical version could look like: ((name|stringlit) S*) + '>'
var n = rawdata.length;
if (rawdata.substring(j, j + 2) === '--') { //comment
    // Locate --.*-- as the body of the comment
    return class_var.parse_comment(i, 1);
} else if (rawdata[j] === '[') { //marked section
    // Locate [statusWord [...arbitrary SGML...]] as the body of the marked section
    // Where statusWord is one of TEMP, CDATA, IGNORE, INCLUDE, RCDATA
    // Note that this is extended by Microsoft Office "Save as Web" function
    // to include [if...] and [endif].
    return class_var.parse_marked_section(i, 1);
} else { //all other declaration elements
    var decltype_j = class_var._scan_name(j, i);
    var decltype = decltype_j[0];
    j = decltype_j[1];
}
if (j < 0) {
    return j;
}
if (decltype === "doctype") {
    class_var._decl_otherchars = '';
}
while (j < n) {
    var c = rawdata[j];
    if (c === ">") {
        // end of declaration syntax
        var data = rawdata.substring(i + 2, j);
        if (decltype === "doctype") {
            class_var.handle_decl(data);
        } else {
            // According to the HTML5 specs sections "8.2.4.44 Bogus
            // comment state" and "8.2.4.45 Markup declaration open
            // state", a comment token should be emitted.
            // Calling unknown_decl provides more flexibility though.
            class_var.unknown_decl(data);
        }
        return j + 1;
    }
    if (c === "\"" || c === "'") {
        var m = _declstringlit.exec(rawdata.substring(j));
        if (!m) {
            return -1; // incomplete
        }
        j += m[0].length;
    } else if ("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".includes(c)) {
        var name_j = class_var._scan_name(j, i);
        name = name_j[0];
        j = name_j[1];
    } else if (class_var._decl_otherchars.includes(c)) {
        j = j + 1;
    } else if (c === "[") {
        // this could be handled in a separate doctype parser
        if (decltype === "doctype") {
            j = class_var._parse_doctype_subset(j + 1, i);
        } else if (["attlist", "linktype", "link", "element"].includes(decltype)) {
            // must tolerate []'d groups in a content model in an element declaration
            // also in data attribute specifications of attlist declaration
            // also link type declaration subsets in linktype declarations
            // also link attribute specification lists in link declarations
            throw new Error("unsupported '[' char in " + decltype + " declaration");
        } else {
            throw new Error("unexpected '[' char in declaration");
        }
    } else {
        throw new Error("unexpected " + rawdata[j] + " char in declaration");
    }
    if (j < 0) {
        return j;
    }
}
return -1; // incomplete    
        /// --- BLOCK END 8
    
    }
    
    function parse_marked_section(i, report){
        /// --- BLOCK BEGIN 9
var rawdata = class_var.rawdata;
    if (rawdata.substring(i, i + 3) !== '<![') {
        throw new Error("unexpected call to parse_marked_section()");
    }
    var sectName_j = class_var._scan_name(i + 3, i);
    var sectName = sectName_j[0];
    var j = sectName_j[1];
    if (j < 0) {
        return j;
    }
    var standardSections = new Set(["temp", "cdata", "ignore", "include", "rcdata"]);
    var msOfficeSections = new Set(["if", "else", "endif"]);
    var match;
    if (standardSections.has(sectName)) {
        // look for standard ]]> ending
        match = _markedsectionclose.exec(rawdata.substring(i + 3));
    } else if (msOfficeSections.has(sectName)) {
        // look for MS Office ]> ending
        match = _msmarkedsectionclose.exec(rawdata.substring(i + 3));
    } else {
        throw new Error('unknown status keyword ' + rawdata.substring(i + 3, j) + ' in marked section');
    }
    if (!match) {
        return -1;
    }
    if (report) {
        j = match.index + i + 3;
        class_var.unknown_decl(rawdata.substring(i + 3, j));
    }
    return match.index + match[0].length + i + 3;        
            
            
            
            
        /// --- BLOCK END 9
    
    }
    
    function parse_comment(i, report){
        /// --- BLOCK BEGIN 10
var rawdata = class_var.rawdata;
        if (rawdata.substring(i, i + 4) !== '<!--') {
            throw new Error('unexpected call to parse_comment()');
        }
        var match = _commentclose.exec(rawdata.substring(i + 4));
        if (!match) {
            return -1;
        }
        if (report) {
            var j = match.index;
            class_var.handle_comment(rawdata.substring(i + 4, i + 4 + j));
        }
        return i + 4 + match.index + match[0].length;    
        /// --- BLOCK END 10
    
    }
    
    function _parse_doctype_subset(i, declstartpos){
        /// --- BLOCK BEGIN 11
var rawdata = class_var.rawdata;
var n = rawdata.length;
var j = i;
while (j < n) {
    var c = rawdata[j];
    if (c === "<") {
        var s = rawdata.substring(j, j + 2);
        if (s === "<") {
            // end of buffer; incomplete
            return -1;
        }
        if (s !== "<!") {
            class_var.updatepos(declstartpos, j + 1);
            throw new AssertionError("unexpected char in internal subset (in " + s + ")");
        }
        if (j + 2 === n) {
            // end of buffer; incomplete
            return -1;
        }
        if (j + 4 > n) {
            // end of buffer; incomplete
            return -1;
        }
        if (rawdata.substring(j, j + 4) === "<!--") {
            j = class_var.parse_comment(j, 0);
            if (j < 0) {
                return j;
            }
            continue;
        }
        var result = class_var._scan_name(j + 2, declstartpos);
        var name = result[0];
        j = result[1];
        if (j === -1) {
            return -1;
        }
        if (!["attlist", "element", "entity", "notation"].includes(name)) {
            class_var.updatepos(declstartpos, j + 2);
            throw new AssertionError("unknown declaration " + name + " in internal subset");
        }
        // handle the individual names
        var meth = class_var["_parse_doctype_" + name];
        j = meth(j, declstartpos);
        if (j < 0) {
            return j;
        }
    } else if (c === "%") {
        // parameter entity reference
        if (j + 1 === n) {
            // end of buffer; incomplete
            return -1;
        }
        var result = class_var._scan_name(j + 1, declstartpos);
        s = result[0];
        j = result[1];
        if (j < 0) {
            return j;
        }
        if (rawdata[j] === ";") {
            j = j + 1;
        }
    } else if (c === "]") {
        j = j + 1;
        while (j < n && /\s/.test(rawdata[j])) {
            j = j + 1;
        }
        if (j < n) {
            if (rawdata[j] === ">") {
                return j;
            }
            class_var.updatepos(declstartpos, j);
            throw new AssertionError("unexpected char after internal subset");
        } else {
            return -1;
        }
    } else if (/\s/.test(c)) {
        j = j + 1;
    } else {
        class_var.updatepos(declstartpos, j);
        throw new AssertionError("unexpected char " + c + " in internal subset");
    }
}
// end of buffer reached
return -1;    
        /// --- BLOCK END 11
    
    }
    
    function _parse_doctype_element(i, declstartpos){
        /// --- BLOCK BEGIN 12
var [name, j] = class_var._scan_name(i, declstartpos);
if (j === -1) {
    return -1;
}
// style content model; just skip until '>'
var rawdata = class_var.rawdata;
if (rawdata.substring(j).includes('>')) {
    return rawdata.indexOf(">", j) + 1;
}
return -1;    
        /// --- BLOCK END 12
    
    }
    
    function _parse_doctype_attlist(i, declstartpos){
        /// --- BLOCK BEGIN 13
return null;    
        /// --- BLOCK END 13
    
    }
    
    function _parse_doctype_notation(i, declstartpos){
        /// --- BLOCK BEGIN 14
var name_j = class_var._scan_name(i, declstartpos);
var name = name_j[0];
var j = name_j[1];
if (j < 0) {
    return j;
}
var rawdata = class_var.rawdata;
while (true) {
    var c = rawdata.substring(j, j + 1);
    if (!c) {
        // end of buffer; incomplete
        return -1;
    }
    if (c === '>') {
        return j + 1;
    }
    if (c === "'" || c === '"') {
        var m = _declstringlit.exec(rawdata.substring(j));
        if (!m) {
            return -1;
        }
        j += m[0].length;
    } else {
        var name_j = class_var._scan_name(j, declstartpos);
        name = name_j[0];
        j = name_j[1];
        if (j < 0) {
            return j;
        }
    }
}    
        /// --- BLOCK END 14
    
    }
    
    function _parse_doctype_entity(i, declstartpos){
        /// --- BLOCK BEGIN 15
var rawdata = class_var.rawdata;
    if (rawdata.substring(i, i + 1) === "%") {
        var j = i + 1;
        while (true) {
            var c = rawdata.substring(j, j + 1);
            if (!c) {
                return -1;
            }
            if (/\s/.test(c)) {
                j++;
            } else {
                break;
            }
        }
    } else {
        j = i;
    }
    var name_j = class_var._scan_name(j, declstartpos);
    var name = name_j[0];
    j = name_j[1];
    if (j < 0) {
        return j;
    }
    while (true) {
        c = rawdata.substring(j, j + 1);
        if (!c) {
            return -1;
        }
        if ("'\"".includes(c)) {
            var m = _declstringlit.match(rawdata, j);
            if (m) {
                j = m.end();
            } else {
                return -1; // incomplete
            }
        } else if (c === ">") {
            return j + 1;
        } else {
            name_j = class_var._scan_name(j, declstartpos);
            name = name_j[0];
            j = name_j[1];
            if (j < 0) {
                return j;
            }
        }
    }    
        /// --- BLOCK END 15
    
    }
    
    function _scan_name(i, declstartpos){
        /// --- BLOCK BEGIN 16
var rawdata = class_var.rawdata;
var n = rawdata.length;
if (i === n) {
    return SCAN_NAME_DEFAULT;
}
var m = _declname.exec(rawdata.substring(i));
if (m) {
    var s = m[0];
    var name = s.trim();
    if ((i + s.length) === n) {
        return SCAN_NAME_DEFAULT;
    }
    return [name.toLowerCase(), m.index + i + s.length];
} else {
    class_var.updatepos(declstartpos, i);
    throw new Error(
        "expected name token at " + JSON.stringify(rawdata.substring(declstartpos, declstartpos + 20))
    );
}    
        /// --- BLOCK END 16
    
    }
    
    function unknown_decl(data){
        /// --- BLOCK BEGIN 17
        // pass    
        /// --- BLOCK END 17
    
    }
    
    var class_var = SkelClass('ParserBase');
    class_var.__init__ = __init__;
    class_var.reset = reset;
    class_var.getpos = getpos;
    class_var.updatepos = updatepos;
    class_var.parse_declaration = parse_declaration;
    class_var.parse_marked_section = parse_marked_section;
    class_var.parse_comment = parse_comment;
    class_var._parse_doctype_subset = _parse_doctype_subset;
    class_var._parse_doctype_element = _parse_doctype_element;
    class_var._parse_doctype_attlist = _parse_doctype_attlist;
    class_var._parse_doctype_notation = _parse_doctype_notation;
    class_var._parse_doctype_entity = _parse_doctype_entity;
    class_var._scan_name = _scan_name;
    class_var.unknown_decl = unknown_decl;
    __init__();
    return class_var;
}


function HTMLParser(param_0){
    function __init__(convert_charrefs){
        /// --- BLOCK BEGIN 18
class_var.CDATA_CONTENT_ELEMENTS = CDATA_CONTENT_ELEMENTS;
class_var.convert_charrefs = convert_charrefs;
class_var.reset();    
        /// --- BLOCK END 18
    
    }
    
    function reset(){
        /// --- BLOCK BEGIN 19
class_var.rawdata = '';
class_var.lasttag = '???';
class_var.interesting = interesting_normal;
class_var.cdata_elem = null;
// ParserBase.reset(this); // This line is commented out as it refers to a method not defined in the provided context
class_var.lineno = 1;
class_var.offset = 0;    
        /// --- BLOCK END 19
    
    }
    
    function feed(data){
        /// --- BLOCK BEGIN 20
class_var.rawdata = class_var.rawdata + data;
class_var.goahead(0);    
        /// --- BLOCK END 20
    
    }
    
    function close(){
        /// --- BLOCK BEGIN 21
class_var.goahead(1);    
        /// --- BLOCK END 21
    
    }
    
    function get_starttag_text(){
        /// --- BLOCK BEGIN 22
return class_var.__starttag_text;    
        /// --- BLOCK END 22
    
    }
    
    function set_cdata_mode(elem){
        /// --- BLOCK BEGIN 23
class_var.cdata_elem = elem.toLowerCase();
class_var.interesting = new RegExp('</\\s*' + class_var.cdata_elem + '\\s*>', 'i');
    
        /// --- BLOCK END 23
    
    }
    
    function clear_cdata_mode(){
        /// --- BLOCK BEGIN 24
class_var.interesting = interesting_normal;
class_var.cdata_elem = null;
return null;
    
        /// --- BLOCK END 24
    
    }
    
    function goahead(end){
        function handle_leftangle(){
            /// --- BLOCK BEGIN 25
            if (/^<[a-zA-Z]/.test(rawdata.substring(i))) { // < + letter
                k = class_var.parse_starttag(i);
            } else if (rawdata.startsWith("</", i)) {
                k = class_var.parse_endtag(i);
            } else if (rawdata.startsWith("<!--", i)) {
                k = class_var.parse_comment(i, 1);
            } else if (rawdata.startsWith("<?", i)) {
                k = class_var.parse_pi(i);
            } else if (rawdata.startsWith("<!", i)) {
                k = class_var.parse_html_declaration(i);
            } else if ((i + 1) < n) {
                class_var.handle_data("<");
                k = i + 1;
            } else {
                return "break";
            }
            if (k < 0) {
                if (!end) {
                    return "break";
                }
                k = rawdata.indexOf('>', i + 1);
                if (k < 0) {
                    k = rawdata.indexOf('<', i + 1);
                    if (k < 0) {
                        k = i + 1;
                    }
                } else {
                    k += 1;
                }
                if (class_var.convert_charrefs && !class_var.cdata_elem) {
                    class_var.handle_data(unescape(rawdata.substring(i, k)));
                } else {
                    class_var.handle_data(rawdata.substring(i, k));
                }
            }
            i = class_var.updatepos(i, k);        
            /// --- BLOCK END 25
        
        }
        
        function handle_charref(){
            /// --- BLOCK BEGIN 26
var match = charref.exec(rawdata.substring(i));
if (match) {
    var name = match[0].slice(2, -1);
    class_var.handle_charref(name);
    var k = match.index + i + match[0].length;
    if (!rawdata.startsWith(';', k - 1)) {
        k = k - 1;
    }
    i = class_var.updatepos(i, k);
    return "continue";
} else {
    if (rawdata.substring(i).includes(";")) {  // bail by consuming &#
        class_var.handle_data(rawdata.substring(i, i + 2));
        i = class_var.updatepos(i, i + 2);
    }
    return "break";
}        
            /// --- BLOCK END 26
        
        }
        
        function handle_entityref(){
            /// --- BLOCK BEGIN 27
var match = entityref.exec(rawdata.substring(i));
if (match) {
    var name = match[1];
    class_var.handle_entityref(name);
    var k = i + match[0].length;
    if (!rawdata.startsWith(';', k - 1)) {
        k = k - 1;
    }
    i = class_var.updatepos(i, k);
    return "continue";
}
match = incomplete.exec(rawdata.substring(i));
if (match) {
    if (end && match[0] === rawdata.substring(i)) {
        k = i + match[0].length;
        if (k <= i) {
            k = n;
        }
        i = class_var.updatepos(i, i + 1);
    }
    return "break";
} else if ((i + 1) < n) {
    class_var.handle_data("&");
    i = class_var.updatepos(i, i + 1);
} else {
    return "break";
}        
            /// --- BLOCK END 27
        
        }
        
        /// --- BLOCK BEGIN 28
var rawdata = class_var.rawdata;
var i = 0;
var n = rawdata.length;
while (i < n) {
    if (class_var.convert_charrefs && !class_var.cdata_elem) {
        var j = rawdata.indexOf('<', i);
        if (j < 0) {
            var amppos = rawdata.lastIndexOf('&', Math.max(i, n - 34));
            if (amppos >= 0 && !/[\s;]/.test(rawdata.substring(amppos))) {
                break;
            }
            j = n;
        }
    } else {
        var match = class_var.interesting.exec(rawdata.substring(i));
        if (match) {
            j = match.index + i;
        } else {
            if (class_var.cdata_elem) {
                break;
            }
            j = n;
        }
    }
    if (i < j) {
        if (class_var.convert_charrefs && !class_var.cdata_elem) {
            class_var.handle_data(unescape(rawdata.substring(i, j)));
        } else {
            class_var.handle_data(rawdata.substring(i, j));
        }
    }
    i = class_var.updatepos(i, j);
    if (i == n) break;
    if (rawdata.startsWith('<', i)) {
        var act = handle_leftangle();
        if (act === "break") {
            break;
        } else if (act === "continue") {
            continue;
        } else {
            // pass
        }
    } else if (rawdata.startsWith("&#", i)) {
        var _act = handle_charref();
        if (_act === "break") {
            break;
        } else if (_act === "continue") {
            continue;
        } else {
            // pass
        }
    } else if (rawdata.startsWith('&', i)) {
        var _act = handle_entityref();
        if (_act === "break") {
            break;
        } else if (_act === "continue") {
            continue;
        } else {
            // pass
        }
    } else {
        throw new Error("interesting.search() lied");
    }
}
if (end && i < n && !class_var.cdata_elem) {
    if (class_var.convert_charrefs && !class_var.cdata_elem) {
        class_var.handle_data(unescape(rawdata.substring(i, n)));
    } else {
        class_var.handle_data(rawdata.substring(i, n));
    }
    i = class_var.updatepos(i, n);
}
class_var.rawdata = rawdata.substring(i);    
        /// --- BLOCK END 28
    
    }
    
    function parse_html_declaration(i){
        /// --- BLOCK BEGIN 29
var rawdata = class_var.rawdata;
    if (rawdata.substring(i, i + 2) !== '<!') {
        throw new Error('unexpected call to parse_html_declaration()');
    }
    if (rawdata.substring(i, i + 4) === '<!--') {
        // this case is actually already handled in goahead()
        return class_var.parse_comment(i, 1);
    } else if (rawdata.substring(i, i + 3) === '<![') {
        return class_var.parse_marked_section(i, 1);
    } else if (rawdata.substring(i, i + 9).toLowerCase() === '<!doctype') {
        // find the closing >
        var gtpos = rawdata.indexOf('>', i + 9);
        if (gtpos === -1) {
            return -1;
        }
        class_var.handle_decl(rawdata.substring(i + 2, gtpos));
        return gtpos + 1;
    } else {
        return class_var.parse_bogus_comment(i, 1);
    }    
        /// --- BLOCK END 29
    
    }
    
    function parse_bogus_comment(i, report){
        /// --- BLOCK BEGIN 30
var rawdata = class_var.rawdata;
    if (!(rawdata.substring(i, i + 2) === '<!' || rawdata.substring(i, i + 2) === '</')) {
        throw new Error('unexpected call to parse_comment()');
    }
    var pos = rawdata.indexOf('>', i + 2);
    if (pos === -1) {
        return -1;
    }
    if (report) {
        class_var.handle_comment(rawdata.substring(i + 2, pos));
    }
    return pos + 1;
    
        /// --- BLOCK END 30
    
    }
    
    function parse_pi(i){
        /// --- BLOCK BEGIN 31
var rawdata = class_var.rawdata;
    if (rawdata.substring(i, i + 2) !== '<?') {
        throw new Error('unexpected call to parse_pi()');
    }
    var match = tool_functions.piclose.exec(rawdata.substring(i + 2));
    if (!match) {
        return -1;
    }
    var j = match.index + i + 2;
    class_var.handle_pi(rawdata.substring(i + 2, j));
    j = match.index + match[0].length + i + 2;
    return j;
    
        /// --- BLOCK END 31
    
    }
    
    function parse_starttag(i){
        /// --- BLOCK BEGIN 32
        class_var.__starttag_text = null;
        var endpos = class_var.check_for_whole_start_tag(i);
        if (endpos < 0) {
            var _return_value = endpos;
            return _return_value;
        }
        rawdata = class_var.rawdata;
        class_var.__starttag_text = rawdata.substring(i, endpos);
    
        // Now parse the data between i+1 and j into a tag and attrs
    
        var attrs = [];
        var match = tagfind_tolerant.exec(rawdata.substring(i + 1));
        if (!match) throw new Error('unexpected call to parse_starttag()');
        k = match.index + match[0].length + i + 1;
        class_var.lasttag = tag = match[1].toLowerCase();
        while (k < endpos) {
            var m = rawdata.slice(k-1).match(attrfind_tolerant);
            if (m[2] == undefined) {
                break;
            }
            var attrname = m[1], rest = m[2], attrvalue = m[3];
            if (!rest) {
                attrvalue = null;
            } else if ((attrvalue[0] == "'" && attrvalue[attrvalue.length-1] == "'") || (attrvalue[0] == '"' && attrvalue[attrvalue.length-1] == '"')) {
                attrvalue = attrvalue.slice(1, -1);
            }
            if (attrvalue) {
                attrvalue = unescape(attrvalue);
            }
            attrs.push([attrname.toLowerCase(), attrvalue]);
            k += m[0].length;
        }
    
        var end = rawdata.substring(k, endpos).trim();
        if (end !== ">" && end !== "/>") {
            class_var.handle_data(rawdata.substring(i, endpos));
            var _return_value = endpos;
            return _return_value;
        }
        if (end.endsWith('/>')) {
            // XHTML-style empty tag: <span attr="value" />
            class_var.handle_startendtag(tag, attrs);
        } else {
            class_var.handle_starttag(tag, attrs);
            if (tag === "script" || tag === "style") {
                class_var.set_cdata_mode(tag);
            }
        }
        var _return_value = endpos;
        return _return_value;
            
            
            
            
        /// --- BLOCK END 32
    
    }
    
    function check_for_whole_start_tag(i){
        /// --- BLOCK BEGIN 33
var rawdata = class_var.rawdata;
var m = locatestarttagend_tolerant.exec(rawdata.substring(i));
if (m) {
    var j = i + m.index + m[0].length;
    var next = rawdata.substring(j, j + 1);
    if (next === ">") {
        return j + 1;
    }
    if (next === "/") {
        if (rawdata.startsWith("/>", j)) {
            return j + 2;
        }
        if (rawdata.startsWith("/", j)) {
            // buffer boundary
            return -1;
        }
        // else bogus input
        if (j > i) {
            return j;
        } else {
            return i + 1;
        }
    }
    if (next === "") {
        // end of input
        return -1;
    }
    if ("abcdefghijklmnopqrstuvwxyz=/ABCDEFGHIJKLMNOPQRSTUVWXYZ".includes(next)) {
        // end of input in or before attribute value, or we have the
        // '/' from a '/>' ending
        return -1;
    }
    if (j > i) {
        return j;
    } else {
        return i + 1;
    }
}
throw new Error("we should not get here!");    
        /// --- BLOCK END 33
    
    }
    
    function parse_endtag(i){
        /// --- BLOCK BEGIN 34
var rawdata = class_var.rawdata;
    if (rawdata.substring(i, i+2) !== "</") throw new Error("unexpected call to parse_endtag");
    var match = endendtag.exec(rawdata.substring(i+1));
    if (!match) {
        return -1;
    }
    var gtpos = match.index + match[0].length + i + 1;
    match = endtagfind.exec(rawdata.substring(i));
    if (!match) {
        if (class_var.cdata_elem !== null) {
            class_var.handle_data(rawdata.substring(i, gtpos));
            return gtpos;
        }
        var namematch = tagfind_tolerant.exec(rawdata.substring(i+2));
        if (!namematch) {
            if (rawdata.substring(i, i+3) === '</>') {
                return i+3;
            } else {
                return class_var.parse_bogus_comment(i, 1);
            }
        }
        var tagname = namematch[1].toLowerCase();
        gtpos = rawdata.indexOf('>', namematch.index + namematch[0].length + i + 2);
        class_var.handle_endtag(tagname);
        return gtpos + 1;
    }
    var elem = match[1].toLowerCase();
    if (class_var.cdata_elem !== null) {
        if (elem !== class_var.cdata_elem) {
            class_var.handle_data(rawdata.substring(i, gtpos));
            return gtpos;
        }
    }
    class_var.handle_endtag(elem);
    class_var.clear_cdata_mode();
    return gtpos;    
        /// --- BLOCK END 34
    
    }
    
    function handle_startendtag(tag, attrs){
        /// --- BLOCK BEGIN 35
class_var.handle_starttag(tag, attrs);
class_var.handle_endtag(tag);    
        /// --- BLOCK END 35
    
    }
    
    function handle_starttag(tag, attrs){
        /// --- BLOCK BEGIN 36
        return null;    
        /// --- BLOCK END 36
    
    }
    
    function handle_endtag(tag){
        /// --- BLOCK BEGIN 37
        return null;    
        /// --- BLOCK END 37
    
    }
    
    function handle_charref(name){
        /// --- BLOCK BEGIN 38
        return null;    
        /// --- BLOCK END 38
    
    }
    
    function handle_entityref(name){
        /// --- BLOCK BEGIN 39
        return null;    
        /// --- BLOCK END 39
    
    }
    
    function handle_data(data){
        /// --- BLOCK BEGIN 40
        return null;    
        /// --- BLOCK END 40
    
    }
    
    function handle_comment(data){
        /// --- BLOCK BEGIN 41
        return null;    
        /// --- BLOCK END 41
    
    }
    
    function handle_decl(decl){
        /// --- BLOCK BEGIN 42
        return null;    
        /// --- BLOCK END 42
    
    }
    
    function handle_pi(data){
        /// --- BLOCK BEGIN 43
        return null;    
        /// --- BLOCK END 43
    
    }
    
    function unknown_decl(data){
        /// --- BLOCK BEGIN 44
        return null;    
        /// --- BLOCK END 44
    
    }
    
    var class_var = ParserBase();
    class_var._class_name = 'HTMLParser;' + class_var._class_name;
    class_var.__init__ = __init__;
    class_var.reset = reset;
    class_var.feed = feed;
    class_var.close = close;
    class_var.get_starttag_text = get_starttag_text;
    class_var.set_cdata_mode = set_cdata_mode;
    class_var.clear_cdata_mode = clear_cdata_mode;
    class_var.goahead = goahead;
    class_var.parse_html_declaration = parse_html_declaration;
    class_var.parse_bogus_comment = parse_bogus_comment;
    class_var.parse_pi = parse_pi;
    class_var.parse_starttag = parse_starttag;
    class_var.check_for_whole_start_tag = check_for_whole_start_tag;
    class_var.parse_endtag = parse_endtag;
    class_var.handle_startendtag = handle_startendtag;
    class_var.handle_starttag = handle_starttag;
    class_var.handle_endtag = handle_endtag;
    class_var.handle_charref = handle_charref;
    class_var.handle_entityref = handle_entityref;
    class_var.handle_data = handle_data;
    class_var.handle_comment = handle_comment;
    class_var.handle_decl = handle_decl;
    class_var.handle_pi = handle_pi;
    class_var.unknown_decl = unknown_decl;
    __init__(param_0);
    return class_var;
}
