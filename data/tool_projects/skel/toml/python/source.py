
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

from datetime import tzinfo, timedelta

from collections import OrderedDict

import datetime

import re

import sys

from decimal import Decimal

import datetime

import io

from os import linesep

import re

import sys

import tracer_skip as tool_functions

import copy

def func_dict(*args):
    class_var = SkelClass(class_name='dict', super_class=dict)
    return class_var


def _detect_pathlib_path(p):
    ### --- BLOCK BEGIN 1
    if (3, 4) <= sys.version_info:
        import pathlib
        if user_check_type(p, pathlib.PurePath):
            return True
    return False
    ### --- BLOCK END 1



def _ispath(p):
    ### --- BLOCK BEGIN 2
    if user_check_type(p, (bytes, str)):
        return True
    return _detect_pathlib_path(p)
    ### --- BLOCK END 2



def _getpath(p):
    ### --- BLOCK BEGIN 3
    if (3, 6) <= sys.version_info:
        import os
        return os.fspath(p)
    if _detect_pathlib_path(p):
        return str(p)
    return p
    ### --- BLOCK END 3



def TomlDecodeError(param_0, param_1, param_2):
    """Base toml Exception / Error."""
    def __init__(msg, doc, pos):
        ### --- BLOCK BEGIN 4
        lineno = doc.count('\n', 0, pos) + 1
        colno = pos - doc.rfind('\n', 0, pos)
        emsg = '{} (line {} column {} char {})'.format(msg, lineno, colno, pos)
        ValueError.__init__(class_var, emsg)
        class_var.msg = msg
        class_var.doc = doc
        class_var.pos = pos
        class_var.lineno = lineno
        class_var.colno = colno
        ### --- BLOCK END 4
    
    
    
    class_var = ValueError()
    class_var._class_name = 'TomlDecodeError;' + class_var._class_name
    class_var.__init__ = __init__
    __init__(param_0, param_1, param_2)
    return class_var


def CommentValue(param_0, param_1, param_2, param_3):
    def __init__(val, comment, beginline, _dict):
        ### --- BLOCK BEGIN 5
        class_var.val = val
        separator = "\n" if beginline else " "
        class_var.comment = separator + comment
        class_var._dict = _dict
        ### --- BLOCK END 5
    
    
    
    def __getitem__(key):
        ### --- BLOCK BEGIN 6
        return class_var.val[key]
        ### --- BLOCK END 6
    
    
    
    def __setitem__(key, value):
        ### --- BLOCK BEGIN 7
        class_var.val[key] = value
        ### --- BLOCK END 7
    
    
    
    def dump(dump_value_func):
        ### --- BLOCK BEGIN 8
        retstr = dump_value_func(class_var.val)
        if user_check_type(class_var.val, class_var._dict):
            return class_var.comment + "\n" + str(retstr)
        else:
            return str(retstr) + class_var.comment
        ### --- BLOCK END 8
    
    
    
    class_var = SkelClass('CommentValue')
    class_var.__init__ = __init__
    class_var.__getitem__ = __getitem__
    class_var.__setitem__ = __setitem__
    class_var.dump = dump
    __init__(param_0, param_1, param_2, param_3)
    return class_var


def _strictly_valid_num(n):
    ### --- BLOCK BEGIN 9
    n = n.strip()
    if not n:
        return False
    if n[0] == '_':
        return False
    if n[-1] == '_':
        return False
    if "_." in n or "._" in n:
        return False
    if len(n) == 1:
        return True
    if n[0] == '0' and n[1] not in ['.', 'o', 'b', 'x']:
        return False
    if n[0] == '+' or n[0] == '-':
        n = n[1:]
        if len(n) > 1 and n[0] == '0' and n[1] != '.':
            return False
    if '__' in n:
        return False
    return True
    ### --- BLOCK END 9



def load(f, _dict, decoder):
    """Parses named file or files as toml and returns a dictionary

    Args:
        f: Path to the file to open, array of files to read into single dict
           or a file descriptor
        _dict: (optional) Specifies the class of the returned toml dictionary
        decoder: The decoder to use

    Returns:
        Parsed toml file represented as a dictionary

    Raises:
        TypeError -- When f is invalid type
        TomlDecodeError: Error while decoding toml
        IOError / FileNotFoundError -- When an array with no valid (existing)
        (Python 2 / Python 3)          file paths is passed
    """
    ### --- BLOCK BEGIN 10
    pass
    # Not Reachable
    ### --- BLOCK END 10



def loads(s, _dict, decoder):
    def handle_keyname():
        nonlocal key, openstring, openstrchar, keyname, dottedkey, prev_key
        ### --- BLOCK BEGIN 11
        key += item
        if item == '\n':
            raise TomlDecodeError("Key name found without value. Reached end of line.", original, i)
        if openstring:
            if item == openstrchar:
                oddbackslash = False
                k = 1
                while i >= k and sl[i - k] == '\\':
                    oddbackslash = not oddbackslash
                    k += 1
                if not oddbackslash:
                    keyname = 2
                    openstring = False
                    openstrchar = ""
            return "continue"
        elif keyname == 1:
            if item.isspace():
                keyname = 2
                return "continue"
            elif item == '.':
                dottedkey = True
                return "continue"
            elif item.isalnum() or item == '_' or item == '-':
                return "continue"
            elif (dottedkey and sl[i - 1] == '.' and
            (item == '"' or item == "'")):
                openstring = True
                openstrchar = item
                return "continue"
        elif keyname == 2:
            if item.isspace():
                if dottedkey:
                    nextitem = sl[i + 1]
                    if not nextitem.isspace() and nextitem != '.':
                        keyname = 1
                return "continue"
            if item == '.':
                dottedkey = True
                nextitem = sl[i + 1]
                if not nextitem.isspace() and nextitem != '.':
                    keyname = 1
                return "continue"
        if item == '=':
            keyname = 0
            prev_key = key[:-1].rstrip()
            key = ''
            dottedkey = False
        else:
            raise TomlDecodeError("Found invalid character in key name: '" +
            item + "'. Try quoting the key name.",
            original, i)
        ### --- BLOCK END 11
    
    
    
    def handle_single_quote_1():
        nonlocal multilinestr, openstring, openstrchar
        ### --- BLOCK BEGIN 12
        k = 1
        try:
            while sl[i - k] == "'":
                k += 1
                if k == 3:
                    break
        except IndexError:
            pass
        if k == 3:
            multilinestr = not multilinestr
            openstring = multilinestr
        else:
            openstring = not openstring
        if openstring:
            openstrchar = "'"
        else:
            openstrchar = ""
        ### --- BLOCK END 12
    
    
    
    def handle_single_quote_2():
        nonlocal multilinestr, openstring, openstrchar
        ### --- BLOCK BEGIN 13
        oddbackslash = False
        k = 1
        tripquote = False
        try:
            while sl[i - k] == '"':
                k += 1
                if k == 3:
                    tripquote = True
                    break
            if k == 1 or (k == 3 and tripquote):
                while sl[i - k] == '\\':
                    oddbackslash = not oddbackslash
                    k += 1
        except IndexError:
            pass
        if not oddbackslash:
            if tripquote:
                multilinestr = not multilinestr
                openstring = multilinestr
            else:
                openstring = not openstring
        if openstring:
            openstrchar = '"'
        else:
            openstrchar = ""
        ### --- BLOCK END 13
    
    
    
    def handle_comment():
        ### --- BLOCK BEGIN 14
        j = i
        comment = ""
        try:
            while sl[j] != '\n':
                comment += s[j]
                sl[j] = ' '
                j += 1
        except IndexError:
            return "break"
        if not openarr:
            decoder.preserve_comment(line_no, prev_key, comment, beginline)
        ### --- BLOCK END 14
    
    
    
    def handle_backslash():
        nonlocal multilinestr, keyname, openstring, key, beginline, line_no
        ### --- BLOCK BEGIN 15
        if item == '\n':
            if openstring or multilinestr:
                if not multilinestr:
                    raise TomlDecodeError("Unbalanced quotes", original, i)
                if ((sl[i - 1] == "'" or sl[i - 1] == '"') and (
                sl[i - 2] == sl[i - 1])):
                    sl[i] = sl[i - 1]
                    if sl[i - 3] == sl[i - 1]:
                        sl[i - 3] = ' '
            elif openarr:
                sl[i] = ' '
            else:
                beginline = True
            line_no += 1
        elif beginline and sl[i] != ' ' and sl[i] != '\t':
            beginline = False
            if not keygroup and not arrayoftables:
                if sl[i] == '=':
                    raise TomlDecodeError("Found empty keyname. ", original, i)
                keyname = 1
                key += item
        ### --- BLOCK END 15
    
    
    
    def handle_bracket():
        nonlocal openarr, keygroup, arrayoftables
        ### --- BLOCK BEGIN 16
        if item == '[' and (not openstring and not keygroup and
        not arrayoftables):
            if beginline:
                if len(sl) > i + 1 and sl[i + 1] == '[':
                    arrayoftables = True
                else:
                    keygroup = True
            else:
                openarr += 1
        if item == ']' and not openstring:
            if keygroup:
                keygroup = False
            elif arrayoftables:
                if sl[i - 1] == ']':
                    arrayoftables = False
            else:
                openarr -= 1
        ### --- BLOCK END 16
    
    
    
    def handle_remaining():
        def handle_multikey():
            nonlocal multibackslash, multilinestr, multikey
            ### --- BLOCK BEGIN 17
            if multibackslash:
                multilinestr += line
            else:
                multilinestr += line
            multibackslash = False
            closed = False
            if multilinestr[0] == '[':
                closed = line[-1] == ']'
            elif len(line) > 2:
                closed = (line[-1] == multilinestr[0] and
                line[-2] == multilinestr[0] and
                line[-3] == multilinestr[0])
            if closed:
                try:
                    value, vtype = decoder.load_value(multilinestr, True)
                except ValueError as err:
                    raise TomlDecodeError(str(err), original, pos)
                currentlevel[multikey] = value
                multikey = None
                multilinestr = ""
            else:
                k = len(multilinestr) - 1
                while k > -1 and multilinestr[k] == '\\':
                    multibackslash = not multibackslash
                    k -= 1
                if multibackslash:
                    multilinestr = multilinestr[:-1]
                else:
                    multilinestr += "\n"
            return "continue"
            ### --- BLOCK END 17
        
        
        
        def handle_start_bracket():
            def handle_groupname():
                ### --- BLOCK BEGIN 18
                i = 0
                while i < len(groups):
                    groups[i] = groups[i].strip()
                    if len(groups[i]) > 0 and (groups[i][0] == '"' or
                    groups[i][0] == "'"):
                        groupstr = groups[i]
                        j = i + 1
                        while ((not groupstr[0] == groupstr[-1]) or
                        len(groupstr) == 1):
                            j += 1
                            if j > len(groups) + 2:
                                raise TomlDecodeError("Invalid group name '" +
                                groupstr + "' Something " +
                                "went wrong.", original, pos)
                            groupstr = '.'.join(groups[i:j]).strip()
                        groups[i] = groupstr[1:-1]
                        groups[i + 1:j] = []
                    else:
                        if not _groupname_re.match(groups[i]):
                            raise TomlDecodeError("Invalid group name '" +
                            groups[i] + "'. Try quoting it.",
                            original, pos)
                    i += 1
                ### --- BLOCK END 18
            
            
            
            nonlocal line, currentlevel, arrayoftables
            ### --- BLOCK BEGIN 19
            arrayoftables = False
            if len(line) == 1:
                raise TomlDecodeError("Opening key group bracket on line by itself.", original, pos)
            if line[1] == '[':
                arrayoftables = True
                line = line[2:]
                splitstr = ']]'
            else:
                line = line[1:]
                splitstr = ']'
            i = 1
            quotesplits = decoder._get_split_on_quotes(line)
            quoted = False
            for quotesplit in quotesplits:
                if not quoted and splitstr in quotesplit:
                    break
                i += quotesplit.count(splitstr)
                quoted = not quoted
            line = line.split(splitstr, i)
            if len(line) < i + 1 or line[-1].strip() != "":
                raise TomlDecodeError("Key group not on a line by itself.",
                original, pos)
            groups = splitstr.join(line[:-1]).split('.')
            handle_groupname()
            currentlevel = retval
            for i in range(len(groups)):
                group = groups[i]
                if group == "":
                    raise TomlDecodeError("Can't have a keygroup with an empty name", original, pos)
                try:
                    currentlevel[group]
                    if i == len(groups) - 1:
                        if group in implicitgroups:
                            implicitgroups.remove(group)
                            if arrayoftables:
                                raise TomlDecodeError("An implicitly defined table can't be an array",
                                original, pos)
                        elif arrayoftables:
                            currentlevel[group].append(decoder.get_empty_table()
                            )
                        else:
                            raise TomlDecodeError("What? " + group +
                            " already exists?" +
                            str(currentlevel),
                            original, pos)
                except TypeError:
                    currentlevel = currentlevel[-1]
                    if group not in currentlevel:
                        currentlevel[group] = decoder.get_empty_table()
                        if i == len(groups) - 1 and arrayoftables:
                            currentlevel[group] = [decoder.get_empty_table()]
                except KeyError:
                    if i != len(groups) - 1:
                        implicitgroups.append(group)
                    currentlevel[group] = decoder.get_empty_table()
                    if i == len(groups) - 1 and arrayoftables:
                        currentlevel[group] = [decoder.get_empty_table()]
                currentlevel = currentlevel[group]
                if arrayoftables:
                    try:
                        currentlevel = currentlevel[-1]
                    except KeyError:
                        pass
            ### --- BLOCK END 19
        
        
        
        nonlocal currentlevel
        ### --- BLOCK BEGIN 20
        s = ''.join(sl)
        s = s.split('\n')
        multikey = None
        multilinestr = ""
        multibackslash = False
        pos = 0
        for idx, line in enumerate(s):
            if idx > 0:
                pos += len(s[idx - 1]) + 1
            decoder.embed_comments(idx, currentlevel)
            if not multilinestr or multibackslash or '\n' not in multilinestr:
                line = line.strip()
            if line == "" and (not multikey or multibackslash):
                continue
            if multikey:
                act = handle_multikey()
                if act == "continue":
                    continue
            if line[0] == '[':
                handle_start_bracket()
            elif line[0] == "{":
                if line[-1] != "}":
                    raise TomlDecodeError("Line breaks are not allowed in inline objects", original, pos)
                try:
                    decoder.load_inline_object(line, currentlevel, multikey,
                    multibackslash)
                except ValueError as err:
                    raise TomlDecodeError(str(err), original, pos)
            elif "=" in line:
                try:
                    ret = decoder.load_line(line, currentlevel, multikey, multibackslash)
                except ValueError as err:
                    raise TomlDecodeError(str(err), original, pos)
                if ret is not None:
                    multikey, multilinestr, multibackslash = ret
        return retval
        ### --- BLOCK END 20
    
    
    
    """Parses string as toml

    Args:
        s: String to be parsed
        _dict: (optional) Specifies the class of the returned toml dictionary

    Returns:
        Parsed toml file represented as a dictionary

    Raises:
        TypeError: When a non-string is passed
        TomlDecodeError: Error while decoding toml
    """
    ### --- BLOCK BEGIN 21
    implicitgroups = []
    if decoder is None:
        decoder = TomlDecoder(_dict)
    retval = decoder.get_empty_table()
    currentlevel = retval
    if not isinstance(s, str):
        raise TypeError("Expecting something like a string")
    if not isinstance(s, str):
        s = s.decode('utf8')
    original = s
    sl = list(s)
    openarr = 0
    openstring = False
    openstrchar = ""
    multilinestr = False
    arrayoftables = False
    beginline = True
    keygroup = False
    dottedkey = False
    keyname = 0
    key = ''
    prev_key = ''
    line_no = 1
    for i, item in enumerate(sl):
        if item == '\r' and len(sl) > (i + 1) and sl[i + 1] == '\n':
            sl[i] = ' '
            continue
        if keyname:
            act = handle_keyname()
            if act == "continue":
                continue
        if item == "'" and openstrchar != '"':
            handle_single_quote_1()
        if item == '"' and openstrchar != "'":
            handle_single_quote_2()
        if item == '#' and (not openstring and not keygroup and
        not arrayoftables):
            act = handle_comment()
            if act == "break":
                break
        handle_bracket()
        handle_backslash()
    if keyname:
        raise TomlDecodeError("Key name found without value. Reached end of file.", original, len(s))
    if openstring:  # reached EOF and have an unterminated string
        raise TomlDecodeError("Unterminated string found. Reached end of file.", original, len(s))
    return handle_remaining()
    ### --- BLOCK END 21



def _load_date(val):
    ### --- BLOCK BEGIN 22
    microsecond = 0
    tz = None
    try:
        if len(val) > 19:
            if val[19] == '.':
                if val[-1].upper() == 'Z':
                    subsecondval = val[20:-1]
                    tzval = "Z"
                else:
                    subsecondvalandtz = val[20:]
                    if '+' in subsecondvalandtz:
                        splitpoint = subsecondvalandtz.index('+')
                        subsecondval = subsecondvalandtz[:splitpoint]
                        tzval = subsecondvalandtz[splitpoint:]
                    elif '-' in subsecondvalandtz:
                        splitpoint = subsecondvalandtz.index('-')
                        subsecondval = subsecondvalandtz[:splitpoint]
                        tzval = subsecondvalandtz[splitpoint:]
                    else:
                        tzval = None
                        subsecondval = subsecondvalandtz
                if tzval is not None:
                    tz = TomlTz(tzval)
                microsecond = int(int(subsecondval) *
                (10 ** (6 - len(subsecondval))))
            else:
                tz = TomlTz(val[19:].upper())
    except ValueError:
        tz = None
    if "-" not in val[1:]:
        return None
    try:
        if len(val) == 10:
            d = datetime.date(
            int(val[:4]), int(val[5:7]),
            int(val[8:10]))
        else:
            d = datetime.datetime(
            int(val[:4]), int(val[5:7]),
            int(val[8:10]), int(val[11:13]),
            int(val[14:16]), int(val[17:19]), microsecond, tz)
    except ValueError:
        return None
    return d
    ### --- BLOCK END 22



def _load_unicode_escapes(v, hexbytes, prefix):
    ### --- BLOCK BEGIN 23
    skip = False
    i = len(v) - 1
    while i > -1 and v[i] == '\\':
        skip = not skip
        i -= 1
    for hx in hexbytes:
        if skip:
            skip = False
            i = len(hx) - 1
            while i > -1 and hx[i] == '\\':
                skip = not skip
                i -= 1
            v += prefix
            v += hx
            continue
        hxb = ""
        i = 0
        hxblen = 4
        if prefix == "\\U":
            hxblen = 8
        hxb = ''.join(hx[i:i + hxblen]).lower()
        if hxb.strip('0123456789abcdef'):
            raise ValueError("Invalid escape sequence: " + hxb)
        if hxb[0] == "d" and hxb[1].strip('01234567'):
            raise ValueError("Invalid escape sequence: " + hxb +
            ". Only scalar unicode points are allowed.")
        v += unichr(int(hxb, 16))
        v += str(hx[len(hxb):])
    return v
    ### --- BLOCK END 23



def _unescape(v):
    """Unescape characters in a TOML string."""
    ### --- BLOCK BEGIN 24
    i = 0
    backslash = False
    while i < len(v):
        if backslash:
            backslash = False
            if v[i] in _escapes:
                v = v[:i - 1] + _escape_to_escapedchars[v[i]] + v[i + 1:]
            elif v[i] == '\\':
                v = v[:i - 1] + v[i:]
            elif v[i] == 'u' or v[i] == 'U':
                i += 1
            else:
                raise ValueError("Reserved escape sequence used")
            continue
        elif v[i] == '\\':
            backslash = True
        i += 1
    return v
    ### --- BLOCK END 24



def InlineTableDict(*args):
    """Sentinel subclass of dict for inline tables."""
    class_var = SkelClass('InlineTableDict')
    return class_var


def DynamicInlineTableDict(*args):
    """Concrete sentinel subclass for inline tables.
    It is a subclass of _dict which is passed in dynamically at load
    time

    It is also a subclass of InlineTableDict
    """
    class_var = SkelClass(class_name='dict', super_class=dict)
    return class_var


def TomlDecoder(param_0):
    def __init__(_dict):
        ### --- BLOCK BEGIN 25
        class_var._dict = _dict
        ### --- BLOCK END 25
    
    
    
    def get_empty_table():
        ### --- BLOCK BEGIN 26
        return class_var._dict()
        ### --- BLOCK END 26
    
    
    
    def get_empty_inline_table():
        ### --- BLOCK BEGIN 27
        return DynamicInlineTableDict()
        ### --- BLOCK END 27
    
    
    
    def load_inline_object(line, currentlevel, multikey, multibackslash):
        ### --- BLOCK BEGIN 28
        candidate_groups = line[1:-1].split(",")
        groups = []
        if len(candidate_groups) == 1 and not candidate_groups[0].strip():
            candidate_groups.pop()
        while len(candidate_groups) > 0:
            candidate_group = candidate_groups.pop(0)
            try:
                _, value = candidate_group.split('=', 1)
            except ValueError:
                raise ValueError("Invalid inline table encountered")
            value = value.strip()
            if ((value[0] == value[-1] and value[0] in ('"', "'")) or (
            value[0] in '-0123456789' or
            value in ('true', 'false') or
            (value[0] == "[" and value[-1] == "]") or
            (value[0] == '{' and value[-1] == '}'))):
                groups.append(candidate_group)
            elif len(candidate_groups) > 0:
                candidate_groups[0] = (candidate_group + "," +
                candidate_groups[0])
            else:
                raise ValueError("Invalid inline table value encountered")
        for group in groups:
            status = class_var.load_line(group, currentlevel, multikey, multibackslash)
            if status is not None:
                break
        ### --- BLOCK END 28
    
    
    
    def _get_split_on_quotes(line):
        ### --- BLOCK BEGIN 29
        doublequotesplits = line.split('"')
        quoted = False
        quotesplits = []
        if len(doublequotesplits) > 1 and "'" in doublequotesplits[0]:
            singlequotesplits = doublequotesplits[0].split("'")
            doublequotesplits = doublequotesplits[1:]
            while len(singlequotesplits) % 2 == 0 and len(doublequotesplits):
                singlequotesplits[-1] += '"' + doublequotesplits[0]
                doublequotesplits = doublequotesplits[1:]
                if "'" in singlequotesplits[-1]:
                    singlequotesplits = (singlequotesplits[:-1] +
                    singlequotesplits[-1].split("'"))
            quotesplits += singlequotesplits
        for doublequotesplit in doublequotesplits:
            if quoted:
                quotesplits.append(doublequotesplit)
            else:
                quotesplits += doublequotesplit.split("'")
                quoted = not quoted
        return quotesplits
        ### --- BLOCK END 29
    
    
    
    def load_line(line, currentlevel, multikey, multibackslash):
        ### --- BLOCK BEGIN 30
        i = 1
        quotesplits = class_var._get_split_on_quotes(line)
        quoted = False
        for quotesplit in quotesplits:
            if not quoted and '=' in quotesplit:
                break
            i += quotesplit.count('=')
            quoted = not quoted
        pair = line.split('=', i)
        strictly_valid = _strictly_valid_num(pair[-1])
        if _number_with_underscores.match(pair[-1]):
            pair[-1] = pair[-1].replace('_', '')
        while len(pair[-1]) and (pair[-1][0] != ' ' and pair[-1][0] != '\t' and
        pair[-1][0] != "'" and pair[-1][0] != '"' and
        pair[-1][0] != '[' and pair[-1][0] != '{' and
        pair[-1].strip() != 'true' and
        pair[-1].strip() != 'false'):
            try:
                float(pair[-1])
                break
            except ValueError:
                pass
            if _load_date(pair[-1]) is not None:
                break
            if TIME_RE.match(pair[-1]):
                break
            i += 1
            prev_val = pair[-1]
            pair = line.split('=', i)
            if prev_val == pair[-1]:
                raise ValueError("Invalid date or number")
            if strictly_valid:
                strictly_valid = _strictly_valid_num(pair[-1])
        pair = ['='.join(pair[:-1]).strip(), pair[-1].strip()]
        if '.' in pair[0]:
            if '"' in pair[0] or "'" in pair[0]:
                quotesplits = class_var._get_split_on_quotes(pair[0])
                quoted = False
                levels = []
                for quotesplit in quotesplits:
                    if quoted:
                        levels.append(quotesplit)
                    else:
                        levels += [level.strip() for level in
                        quotesplit.split('.')]
                    quoted = not quoted
            else:
                levels = pair[0].split('.')
            while levels[-1] == "":
                levels = levels[:-1]
            for level in levels[:-1]:
                if level == "":
                    continue
                if level not in currentlevel:
                    currentlevel[level] = class_var.get_empty_table()
                currentlevel = currentlevel[level]
            pair[0] = levels[-1].strip()
        elif (pair[0][0] == '"' or pair[0][0] == "'") and \
                        (pair[0][-1] == pair[0][0]):
            pair[0] = _unescape(pair[0][1:-1])
        k, koffset = class_var._load_line_multiline_str(pair[1])
        if k > -1:
            while k > -1 and pair[1][k + koffset] == '\\':
                multibackslash = not multibackslash
                k -= 1
            if multibackslash:
                multilinestr = pair[1][:-1]
            else:
                multilinestr = pair[1] + "\n"
            multikey = pair[0]
        else:
            value, vtype = class_var.load_value(pair[1], strictly_valid)
        try:
            currentlevel[pair[0]]
            raise ValueError("Duplicate keys!")
        except TypeError:
            raise ValueError("Duplicate keys!")
        except KeyError:
            if multikey:
                return multikey, multilinestr, multibackslash
            else:
                currentlevel[pair[0]] = value
        ### --- BLOCK END 30
    
    
    
    def _load_line_multiline_str(p):
        ### --- BLOCK BEGIN 31
        poffset = 0
        if len(p) < 3:
            return -1, poffset
        if p[0] == '[' and (p.strip()[-1] != ']' and
        class_var._load_array_isstrarray(p)):
            newp = p[1:].strip().split(',')
            while len(newp) > 1 and newp[-1][0] != '"' and newp[-1][0] != "'":
                newp = newp[:-2] + [newp[-2] + ',' + newp[-1]]
            newp = newp[-1]
            poffset = len(p) - len(newp)
            p = newp
        if p[0] != '"' and p[0] != "'":
            return -1, poffset
        if p[1] != p[0] or p[2] != p[0]:
            return -1, poffset
        if len(p) > 5 and p[-1] == p[0] and p[-2] == p[0] and p[-3] == p[0]:
            return -1, poffset
        return len(p) - 1, poffset
        ### --- BLOCK END 31
    
    
    
    def load_value(v, strictly_valid):
        def handle_remaining():
            nonlocal v
            ### --- BLOCK BEGIN 32
            if parsed_date is not None:
                return (parsed_date, "date")
            if not strictly_valid:
                raise ValueError("Weirdness with leading zeroes or "
                "underscores in your number.")
            itype = "int"
            neg = False
            if v[0] == '-':
                neg = True
                v = v[1:]
            elif v[0] == '+':
                v = v[1:]
            v = v.replace('_', '')
            lowerv = v.lower()
            if '.' in v or ('x' not in v and ('e' in v or 'E' in v)):
                if '.' in v and v.split('.', 1)[1] == '':
                    raise ValueError("This float is missing digits after "
                    "the point")
                if v[0] not in '0123456789':
                    raise ValueError("This float doesn't have a leading "
                    "digit")
                v = float(v)
                itype = "float"
            elif len(lowerv) == 3 and (lowerv == 'inf' or lowerv == 'nan'):
                v = float(v)
                itype = "float"
            if itype == "int":
                v = int(v, 0)
            if neg:
                return (0 - v, itype)
            return (v, itype)
            ### --- BLOCK END 32
        
        
        
        ### --- BLOCK BEGIN 33
        if not v:
            raise ValueError("Empty value is invalid")
        if v == 'true':
            return (True, "bool")
        elif v.lower() == 'true':
            raise ValueError("Only all lowercase booleans allowed")
        elif v == 'false':
            return (False, "bool")
        elif v.lower() == 'false':
            raise ValueError("Only all lowercase booleans allowed")
        elif v[0] == '"' or v[0] == "'":
            quotechar = v[0]
            testv = v[1:].split(quotechar)
            triplequote = False
            triplequotecount = 0
            if len(testv) > 1 and testv[0] == '' and testv[1] == '':
                testv = testv[2:]
                triplequote = True
            closed = False
            for tv in testv:
                if tv == '':
                    if triplequote:
                        triplequotecount += 1
                    else:
                        closed = True
                else:
                    oddbackslash = False
                    try:
                        i = -1
                        j = tv[i]
                        while j == '\\':
                            oddbackslash = not oddbackslash
                            i -= 1
                            j = tv[i]
                    except IndexError:
                        pass
                    if not oddbackslash:
                        if closed:
                            raise ValueError("Found tokens after a closed " +
                            "string. Invalid TOML.")
                        else:
                            if not triplequote or triplequotecount > 1:
                                closed = True
                            else:
                                triplequotecount = 0
            if quotechar == '"':
                escapeseqs = v.split('\\')[1:]
                backslash = False
                for i in escapeseqs:
                    if i == '':
                        backslash = not backslash
                    else:
                        if i[0] not in _escapes and (i[0] != 'u' and
                        i[0] != 'U' and
                        not backslash):
                            raise ValueError("Reserved escape sequence used")
                        if backslash:
                            backslash = False
                for prefix in ["\\u", "\\U"]:
                    if prefix in v:
                        hexbytes = v.split(prefix)
                        v = _load_unicode_escapes(hexbytes[0], hexbytes[1:], prefix)
                v = _unescape(v)
            if len(v) > 1 and v[1] == quotechar and (len(v) < 3 or
            v[1] == v[2]):
                v = v[2:-2]
            return (v[1:-1], "str")
        elif v[0] == '[':
            return (class_var.load_array(v), "array")
        elif v[0] == '{':
            inline_object = class_var.get_empty_inline_table()
            class_var.load_inline_object(v, inline_object, False, False)
            return (inline_object, "inline_object")
        elif TIME_RE.match(v):
            h, m, s, _, ms = TIME_RE.match(v).groups()
            time = datetime.time(int(h), int(m), int(s), int(ms) if ms else 0)
            return (time, "time")
        else:
            parsed_date = _load_date(v)
            return handle_remaining()
        ### --- BLOCK END 33
    
    
    
    def bounded_string(s):
        ### --- BLOCK BEGIN 34
        if len(s) == 0:
            return True
        if s[-1] != s[0]:
            return False
        i = -2
        backslash = False
        while len(s) + i > 0:
            if s[i] == "\\":
                backslash = not backslash
                i -= 1
            else:
                break
        return not backslash
        ### --- BLOCK END 34
    
    
    
    def _load_array_isstrarray(a):
        ### --- BLOCK BEGIN 35
        a = a[1:-1].strip()
        if a != '' and (a[0] == '"' or a[0] == "'"):
            return True
        return False
        ### --- BLOCK END 35
    
    
    
    def load_array(a):
        ### --- BLOCK BEGIN 36
        retval = []
        a = a.strip()
        if '[' not in a[1:-1] or "" != a[1:-1].split('[')[0].strip():
            strarray = class_var._load_array_isstrarray(a)
            if not a[1:-1].strip().startswith('{'):
                a = a[1:-1].split(',')
            else:
            # a is an inline object, we must find the matching parenthesis
            # to define groups
                new_a = []
                start_group_index = 1
                end_group_index = 2
                open_bracket_count = 1 if a[start_group_index] == '{' else 0
                in_str = False
                while end_group_index < len(a[1:]):
                    if a[end_group_index] == '"' or a[end_group_index] == "'":
                        if in_str:
                            backslash_index = end_group_index - 1
                            while (backslash_index > -1 and
                            a[backslash_index] == '\\'):
                                in_str = not in_str
                                backslash_index -= 1
                        in_str = not in_str
                    if not in_str and a[end_group_index] == '{':
                        open_bracket_count += 1
                    if in_str or a[end_group_index] != '}':
                        end_group_index += 1
                        continue
                    elif a[end_group_index] == '}' and open_bracket_count > 1:
                        open_bracket_count -= 1
                        end_group_index += 1
                        continue
                    # Increase end_group_index by 1 to get the closing bracket
                    end_group_index += 1
                    new_a.append(a[start_group_index:end_group_index])
                    # The next start index is at least after the closing
                    # bracket, a closing bracket can be followed by a comma
                    # since we are in an array.
                    start_group_index = end_group_index + 1
                    while (start_group_index < len(a[1:]) and
                    a[start_group_index] != '{'):
                        start_group_index += 1
                    end_group_index = start_group_index + 1
                a = new_a
            b = 0
            if strarray:
                while b < len(a) - 1:
                    ab = a[b].strip()
                    while (not class_var.bounded_string(ab) or
                    (len(ab) > 2 and
                    ab[0] == ab[1] == ab[2] and
                    ab[-2] != ab[0] and
                    ab[-3] != ab[0])):
                        a[b] = a[b] + ',' + a[b + 1]
                        ab = a[b].strip()
                        if b < len(a) - 2:
                            a = a[:b + 1] + a[b + 2:]
                        else:
                            a = a[:b + 1]
                    b += 1
        else:
            al = list(a[1:-1])
            a = []
            openarr = 0
            j = 0
            for i in range(len(al)):
                if al[i] == '[':
                    openarr += 1
                elif al[i] == ']':
                    openarr -= 1
                elif al[i] == ',' and not openarr:
                    a.append(''.join(al[j:i]))
                    j = i + 1
            a.append(''.join(al[j:]))
        for i in range(len(a)):
            a[i] = a[i].strip()
            if a[i] != '':
                nval, ntype = class_var.load_value(a[i], True)
                retval.append(nval)
        return retval
        ### --- BLOCK END 36
    
    
    
    def preserve_comment(line_no, key, comment, beginline):
        ### --- BLOCK BEGIN 37
        pass
        ### --- BLOCK END 37
    
    
    
    def embed_comments(idx, currentlevel):
        ### --- BLOCK BEGIN 38
        pass
        ### --- BLOCK END 38
    
    
    
    class_var = SkelClass('TomlDecoder')
    class_var.__init__ = __init__
    class_var.get_empty_table = get_empty_table
    class_var.get_empty_inline_table = get_empty_inline_table
    class_var.load_inline_object = load_inline_object
    class_var._get_split_on_quotes = _get_split_on_quotes
    class_var.load_line = load_line
    class_var._load_line_multiline_str = _load_line_multiline_str
    class_var.load_value = load_value
    class_var.bounded_string = bounded_string
    class_var._load_array_isstrarray = _load_array_isstrarray
    class_var.load_array = load_array
    class_var.preserve_comment = preserve_comment
    class_var.embed_comments = embed_comments
    __init__(param_0)
    return class_var


def TomlPreserveCommentDecoder(param_0):
    def __init__(_dict):
        ### --- BLOCK BEGIN 39
        class_var.saved_comments = {}
        
        ### --- BLOCK END 39
    
    
    
    def preserve_comment(line_no, key, comment, beginline):
        ### --- BLOCK BEGIN 40
        class_var.saved_comments[line_no] = (key, comment, beginline)
        ### --- BLOCK END 40
    
    
    
    def embed_comments(idx, currentlevel):
        ### --- BLOCK BEGIN 41
        if idx not in class_var.saved_comments:
            return
        key, comment, beginline = class_var.saved_comments[idx]
        currentlevel[key] = CommentValue(currentlevel[key], comment, beginline, class_var._dict)
        ### --- BLOCK END 41
    
    
    
    class_var = TomlDecoder(param_0)
    class_var._class_name = 'TomlPreserveCommentDecoder;' + class_var._class_name
    class_var.__init__ = __init__
    class_var.preserve_comment = preserve_comment
    class_var.embed_comments = embed_comments
    __init__(param_0)
    return class_var


def dump(o, f, encoder):
    """Writes out dict as toml to a file

    Args:
        o: Object to dump into toml
        f: File descriptor where the toml should be stored
        encoder: The ``TomlEncoder`` to use for constructing the output string

    Returns:
        String containing the toml corresponding to dictionary

    Raises:
        TypeError: When anything other than file descriptor is passed
    """
    ### --- BLOCK BEGIN 42
    if not f.write:
        raise TypeError("You can only dump an object to a file descriptor")
    d = dumps(o, encoder=encoder)
    f.write(d)
    return d
    ### --- BLOCK END 42



def dumps(o, encoder):
    """Stringifies input dict as toml

    Args:
        o: Object to dump into toml
        encoder: The ``TomlEncoder`` to use for constructing the output string

    Returns:
        String containing the toml corresponding to dict

    Examples:
        ```python
        >>> import toml
        >>> output = {
        ... 'a': "I'm a string",
        ... 'b': ["I'm", "a", "list"],
        ... 'c': 2400
        ... }
        >>> toml.dumps(output)
        'a = "I\'m a string"\nb = [ "I\'m", "a", "list",]\nc = 2400\n'
        ```
    """
    ### --- BLOCK BEGIN 43
    retval = ""
    if encoder is None:
        encoder = TomlEncoder(o.__class__, False)
    addtoretval, sections = encoder.dump_sections(o, "")
    retval += addtoretval
    while sections:
        newsections = encoder.get_empty_table()
        for section in sections:
            addtoretval, addtosections = encoder.dump_sections(sections[section], section)
            if addtoretval or (not addtoretval and not addtosections):
                if retval and retval[-2:] != "\n\n":
                    retval += "\n"
                retval += "[" + section + "]\n"
                if addtoretval:
                    retval += addtoretval
            for s in addtosections:
                newsections[section + "." + s] = addtosections[s]
        sections = newsections
    return retval
    ### --- BLOCK END 43



def _dump_str(v):
    ### --- BLOCK BEGIN 44
    v = "%r" % v
    if v[0] == 'u':
        v = v[1:]
    singlequote = v.startswith("'")
    if singlequote or v.startswith('"'):
        v = v[1:-1]
    if singlequote:
        v = v.replace("\\'", "'")
        v = v.replace('"', '\\"')
    v = v.split("\\x")
    while len(v) > 1:
        i = -1
        if not v[0]:
            v = v[1:]
        v[0] = v[0].replace("\\\\", "\\")
        # No, I don't know why != works and == breaks
        joinx = v[0][i] != "\\"
        while v[0][:i] and v[0][i] == "\\":
            joinx = not joinx
            i -= 1
        if joinx:
            joiner = "x"
        else:
            joiner = "u00"
        v = [v[0] + joiner + v[1]] + v[2:]
    return str('"' + v[0] + '"')
    ### --- BLOCK END 44



def _dump_float(v):
    ### --- BLOCK BEGIN 45
    return "{}".format(v).replace("e+0", "e+").replace("e-0", "e-")
    ### --- BLOCK END 45



def _dump_time(v):
    ### --- BLOCK BEGIN 46
    utcoffset = v.utcoffset()
    if utcoffset is None:
        return v.isoformat()
    # The TOML norm specifies that it's local time thus we drop the offset
    return v.isoformat()[:-6]
    ### --- BLOCK END 46



def _dump_bool(v):
    ### --- BLOCK BEGIN 47
    return str(v).lower()
    ### --- BLOCK END 47



def _dump_int(v):
    ### --- BLOCK BEGIN 48
    return v
    ### --- BLOCK END 48



def _dump_datetime(v):
    ### --- BLOCK BEGIN 49
    return v.isoformat().replace('+00:00', 'Z')
    ### --- BLOCK END 49



def _dump_date(v):
    ### --- BLOCK BEGIN 50
    return v.isoformat()
    ### --- BLOCK END 50



def TomlEncoder(param_0, param_1):
    def __init__(_dict, preserve):
        ### --- BLOCK BEGIN 51
        class_var._dict = _dict
        class_var.preserve = preserve
        class_var.dump_funcs = {
        "str": _dump_str,
        "list": class_var.dump_list,
        "bool": _dump_bool,
        "int": _dump_int,
        "float": _dump_float,
        }
        ### --- BLOCK END 51
    
    
    
    def get_empty_table():
        ### --- BLOCK BEGIN 52
        return class_var._dict()
        ### --- BLOCK END 52
    
    
    
    def dump_list(v):
        ### --- BLOCK BEGIN 53
        retval = "["
        for u in v:
            retval += " " + str(class_var.dump_value(u)) + ","
        retval += "]"
        return retval
        ### --- BLOCK END 53
    
    
    
    def dump_inline_table(section):
        """Preserve inline table in its compact syntax instead of expanding
            into subsection.
    
            https://github.com/toml-lang/toml#user-content-inline-table
            """
        ### --- BLOCK BEGIN 54
        retval = ""
        if isinstance(section, dict):
            val_list = []
            for k, v in section.items():
                val = class_var.dump_inline_table(v)
                val_list.append(k + " = " + val)
            retval += "{ " + ", ".join(val_list) + " }\n"
            return retval
        else:
            return str(class_var.dump_value(section))
        ### --- BLOCK END 54
    
    
    
    def dump_value(v):
        ### --- BLOCK BEGIN 55
        dump_fn = None
        for t in class_var.dump_funcs:
            if (t == "str" and isinstance(v, str) or
            t == "list" and isinstance(v, list) or
            t == "bool" and isinstance(v, bool) or
            t == "int" and isinstance(v, int) or
            t == "float" and isinstance(v, float) or
            t == "CommentValue" and user_check_type(v, CommentValue)):
                dump_fn = class_var.dump_funcs[t]
                break
        if dump_fn is None and hasattr(v, '__iter__'):
            dump_fn = class_var.dump_funcs["list"]
        # Evaluate function (if it exists) else return v
        return dump_fn(v) if dump_fn is not None else class_var.dump_funcs["str"](v)
        ### --- BLOCK END 55
    
    
    
    def dump_sections(o, sup):
        ### --- BLOCK BEGIN 56
        retstr = ""
        if sup != "" and sup[-1] != ".":
            sup += '.'
        retdict = class_var._dict()
        arraystr = ""
        for section in o:
            section = str(section)
            qsection = section
            if not re.match(r'^[A-Za-z0-9_-]+$', section):
                qsection = _dump_str(section)
            if not isinstance(o[section], dict):
                arrayoftables = False
                if isinstance(o[section], list):
                    for a in o[section]:
                        if isinstance(a, dict):
                            arrayoftables = True
                if arrayoftables:
                    for a in o[section]:
                        arraytabstr = "\n"
                        arraystr += "[[" + sup + qsection + "]]\n"
                        s, d = class_var.dump_sections(a, sup + qsection)
                        if s:
                            if s[0] == "[":
                                arraytabstr += s
                            else:
                                arraystr += s
                        while d:
                            newd = class_var._dict()
                            for dsec in d:
                                s1, d1 = class_var.dump_sections(d[dsec], sup +
                                qsection + "." +
                                dsec)
                                if s1:
                                    arraytabstr += ("[" + sup + qsection +
                                    "." + dsec + "]\n")
                                    arraytabstr += s1
                                for s1 in d1:
                                    newd[dsec + "." + s1] = d1[s1]
                            d = newd
                        arraystr += arraytabstr
                else:
                    if o[section] is not None:
                        retstr += (qsection + " = " +
                        str(class_var.dump_value(o[section])) + '\n')
            elif class_var.preserve and user_check_type(o[section], InlineTableDict):
                retstr += (qsection + " = " +
                class_var.dump_inline_table(o[section]))
            else:
                retdict[qsection] = o[section]
        retstr += arraystr
        return (retstr, retdict)
        ### --- BLOCK END 56
    
    
    
    class_var = SkelClass('TomlEncoder')
    class_var.__init__ = __init__
    class_var.get_empty_table = get_empty_table
    class_var.dump_list = dump_list
    class_var.dump_inline_table = dump_inline_table
    class_var.dump_value = dump_value
    class_var.dump_sections = dump_sections
    __init__(param_0, param_1)
    return class_var


def TomlPreserveInlineDictEncoder(param_0):
    def __init__(_dict):
        ### --- BLOCK BEGIN 57
        
        pass
        ### --- BLOCK END 57
    
    
    
    class_var = TomlEncoder(param_0, True)
    class_var._class_name = 'TomlPreserveInlineDictEncoder;' + class_var._class_name
    class_var.__init__ = __init__
    __init__(param_0)
    return class_var


def TomlArraySeparatorEncoder(param_0, param_1, param_2):
    def __init__(_dict, preserve, separator):
        ### --- BLOCK BEGIN 58
        
        if separator.strip() == "":
            separator = "," + separator
        elif separator.strip(' \t\n\r,'):
            raise ValueError("Invalid separator for arrays")
        class_var.separator = separator
        ### --- BLOCK END 58
    
    
    
    def dump_list(v):
        ### --- BLOCK BEGIN 59
        t = []
        retval = "["
        for u in v:
            t.append(class_var.dump_value(u))
        while t != []:
            s = []
            for u in t:
                if isinstance(u, list):
                    for r in u:
                        s.append(r)
                else:
                    retval += " " + str(u) + class_var.separator
            t = s
        retval += "]"
        return retval
        ### --- BLOCK END 59
    
    
    
    class_var = TomlEncoder(param_0, param_1)
    class_var._class_name = 'TomlArraySeparatorEncoder;' + class_var._class_name
    class_var.__init__ = __init__
    class_var.dump_list = dump_list
    __init__(param_0, param_1, param_2)
    return class_var


def TomlNumpyEncoder(param_0, param_1):
    def __init__(_dict, preserve):
        ### --- BLOCK BEGIN 60
        import numpy as np
        
        class_var.dump_funcs[np.float16] = _dump_float
        class_var.dump_funcs[np.float32] = _dump_float
        class_var.dump_funcs[np.float64] = _dump_float
        class_var.dump_funcs[np.int16] = class_var._dump_int
        class_var.dump_funcs[np.int32] = class_var._dump_int
        class_var.dump_funcs[np.int64] = class_var._dump_int
        ### --- BLOCK END 60
    
    
    
    def _dump_int(v):
        ### --- BLOCK BEGIN 61
        return "{}".format(int(v))
        ### --- BLOCK END 61
    
    
    
    class_var = TomlEncoder(param_0, param_1)
    class_var._class_name = 'TomlNumpyEncoder;' + class_var._class_name
    class_var.__init__ = __init__
    class_var._dump_int = _dump_int
    __init__(param_0, param_1)
    return class_var


def TomlPreserveCommentEncoder(param_0, param_1):
    def __init__(_dict, preserve):
        ### --- BLOCK BEGIN 62
        
        class_var.dump_funcs["CommentValue"] = lambda v: v.dump(class_var.dump_value)
        ### --- BLOCK END 62
    
    
    
    class_var = TomlEncoder(param_0, param_1)
    class_var._class_name = 'TomlPreserveCommentEncoder;' + class_var._class_name
    class_var.__init__ = __init__
    __init__(param_0, param_1)
    return class_var


def TomlPathlibEncoder(*args):
    def _dump_pathlib_path(v):
        ### --- BLOCK BEGIN 63
        return _dump_str(str(v))
        ### --- BLOCK END 63
    
    
    
    def dump_value(v):
        ### --- BLOCK BEGIN 64
        if (3, 4) <= sys.version_info:
            import pathlib
            if user_check_type(v, pathlib.PurePath):
                v = str(v)
        return TomlEncoder(TomlPathlibEncoder, class_var).dump_value(v)
        ### --- BLOCK END 64
    
    
    
    class_var = TomlEncoder(*args)
    class_var._class_name = 'TomlPathlibEncoder;' + class_var._class_name
    class_var._dump_pathlib_path = _dump_pathlib_path
    class_var.dump_value = dump_value
    return class_var


def TomlOrderedDecoder():
    def __init__():
        ### --- BLOCK BEGIN 65
        
        pass
        ### --- BLOCK END 65
    
    
    
    class_var = TomlDecoder('Error: Type not support')
    class_var._class_name = 'TomlOrderedDecoder;' + class_var._class_name
    class_var.__init__ = __init__
    __init__()
    return class_var


def TomlOrderedEncoder():
    def __init__():
        ### --- BLOCK BEGIN 66
        
        pass
        ### --- BLOCK END 66
    
    
    
    class_var = TomlEncoder('Error: Type not support')
    class_var._class_name = 'TomlOrderedEncoder;' + class_var._class_name
    class_var.__init__ = __init__
    __init__()
    return class_var


def TomlTz(param_0):
    def __init__(toml_offset):
        ### --- BLOCK BEGIN 67
        if toml_offset == "Z":
            class_var._raw_offset = "+00:00"
        else:
            class_var._raw_offset = toml_offset
        class_var._sign = -1 if class_var._raw_offset[0] == '-' else 1
        class_var._hours = int(class_var._raw_offset[1:3])
        class_var._minutes = int(class_var._raw_offset[4:6])
        ### --- BLOCK END 67
    
    
    
    def __getinitargs__():
        ### --- BLOCK BEGIN 68
        return (class_var._raw_offset,)
        ### --- BLOCK END 68
    
    
    
    def __deepcopy__(memo):
        ### --- BLOCK BEGIN 69
        return class_var.__class__(class_var._raw_offset)
        ### --- BLOCK END 69
    
    
    
    def tzname(dt):
        ### --- BLOCK BEGIN 70
        return "UTC" + class_var._raw_offset
        ### --- BLOCK END 70
    
    
    
    def utcoffset(dt):
        ### --- BLOCK BEGIN 71
        return class_var._sign * timedelta(hours=class_var._hours, minutes=class_var._minutes)
        ### --- BLOCK END 71
    
    
    
    def dst(dt):
        ### --- BLOCK BEGIN 72
        return timedelta(0)
        ### --- BLOCK END 72
    
    
    
    class_var = SkelClass(class_name='tzinfo', super_class=tzinfo)
    class_var.__init__ = __init__
    class_var.__getinitargs__ = __getinitargs__
    class_var.__deepcopy__ = __deepcopy__
    class_var.tzname = tzname
    class_var.utcoffset = utcoffset
    class_var.dst = dst
    __init__(param_0)
    return class_var


def convert(v):
    ### --- BLOCK BEGIN 73
    if isinstance(v, list):
        return [convert(vv) for vv in v]
    elif v.get('type', None) is None or v.get('value', None) is None:
        return {k: convert(vv) for (k, vv) in v.items()}
    elif v['type'] == 'string':
        return v['value']
    elif v['type'] == 'integer':
        return int(v['value'])
    elif v['type'] == 'float':
        return float(v['value'])
    elif v['type'] == 'bool':
        return True if v['value'] == 'true' else False
    elif v['type'] in ['datetime', 'datetime-local', 'date-local', 'time-local']:
        return loads('a=' + v['value'])['a']
    else:
        raise Exception(f'unknown type: {v}')
    ### --- BLOCK END 73



def tag(value):
    ### --- BLOCK BEGIN 74
    if isinstance(value, dict):
        return {k: tag(v) for (k, v) in value.items()}
    elif isinstance(value, list):
        return [tag(v) for v in value]
    elif isinstance(value, str):
        return {'type': 'string', 'value': value}
    elif isinstance(value, bool):
        return {'type': 'bool', 'value': str(value).lower()}
    elif isinstance(value, int):
        return {'type': 'integer', 'value': str(value)}
    elif isinstance(value, float):
        return {'type': 'float', 'value': repr(value)}
    elif user_check_type(value, datetime.datetime):
        return {
        'type': 'datetime-local' if value.tzinfo is None else 'datetime',
        'value': value.isoformat().replace('+00:00', 'Z'),
        }
    elif user_check_type(value, datetime.date):
        return {'type': 'date-local', 'value': value.isoformat()}
    elif user_check_type(value, datetime.time):
        return {'type': 'time-local', 'value': value.strftime('%H:%M:%S.%f')}
    assert False, 'Unknown type: %s' % user_get_type(value)
    ### --- BLOCK END 74



def tester(name):
    ### --- BLOCK BEGIN 75
    decode_input = tool_functions.get_input(name)
    decode_result = loads(decode_input, func_dict, None)
    decode_result = tag(decode_result)
    # print(decode_result)
    encode_input = {k: convert(v) for (k, v) in decode_result.items()}
    encode_result = dumps(encode_input, None)
    # print(encode_result)
    ### --- BLOCK END 75



def test_bug_148():
    ### --- BLOCK BEGIN 76
    assert 'a = "\\u0064"\n' == dumps({'a': '\\x64'}, None)
    assert 'a = "\\\\x64"\n' == dumps({'a': '\\\\x64'}, None)
    assert 'a = "\\\\\\u0064"\n' == dumps({'a': '\\\\\\x64'}, None)
    ### --- BLOCK END 76



def test__dict():
    ### --- BLOCK BEGIN 77
    assert isinstance(loads(TEST_STR, func_dict, None), dict)
    ### --- BLOCK END 77



def test_dict_decoder():
    ### --- BLOCK BEGIN 78
    _test_dict_decoder = TomlDecoder(func_dict)
    assert isinstance(loads(TEST_STR, func_dict, _test_dict_decoder), dict)
    ### --- BLOCK END 78



def test_array_sep():
    ### --- BLOCK BEGIN 79
    encoder = TomlArraySeparatorEncoder(func_dict, False, ",\t")
    d = {"a": [1, 2, 3]}
    tmp = dumps(d, encoder)
    o = loads(tmp, func_dict, None)
    tmp2 = dumps(o, encoder)
    assert o == loads(tmp2, func_dict, None)
    ### --- BLOCK END 79



def test_tuple():
    ### --- BLOCK BEGIN 80
    d = {"a": (3, 4)}
    encoder = TomlEncoder(func_dict, False)
    tmp = dumps(d, encoder)
    o = loads(tmp, func_dict, None)
    tmp2 = dumps(o, encoder)
    assert o == loads(tmp2, func_dict, None)
    ### --- BLOCK END 80



def test_commutativity():
    ### --- BLOCK BEGIN 81
    encoder = TomlEncoder(func_dict, False)
    tmp = dumps(TEST_DICT, encoder)
    o = loads(tmp, func_dict, None)
    tmp2 = dumps(o, encoder)
    assert o == loads(tmp2, func_dict, None)
    ### --- BLOCK END 81



def test_comment_preserve_decoder_encoder():
    ### --- BLOCK BEGIN 82
    tmp = loads(tool_functions.test_str, func_dict, TomlPreserveCommentDecoder(func_dict))
    s = dumps(tmp, TomlPreserveCommentEncoder(func_dict, False))
    assert len(s) == len(tool_functions.test_str) and sorted(tool_functions.test_str) == sorted(s)
    ### --- BLOCK END 82



def test():
    ### --- BLOCK BEGIN 83
    tester("Comment")
    tester("Boolean")
    tester("Integer")
    tester("Float")
    tester("Table")
    tester("Inline Table")
    tester("String")
    tester("Array")
    tester("Array of Tables")
    test_bug_148()
    test__dict()
    test_dict_decoder()
    test_array_sep()
    test_tuple()
    test_commutativity()
    test_comment_preserve_decoder_encoder()
    additional_test()
    additional_test2()
    additional_test3()
    additional_test4()
    additional_test5()
    ### --- BLOCK END 83



def unichr(s):
    ### --- BLOCK BEGIN 84
    return chr(s)
    ### --- BLOCK END 84



def additional_test():
    ### --- BLOCK BEGIN 85
    decoder = TomlDecoder(func_dict)
    cur = {}
    multikey = False
    multibackslash = False
    decoder.load_line("'a.x'=2=3", cur, multikey, multibackslash)
    assert(cur == {'a.x': {'=2': 3}})
    ### --- BLOCK END 85



def additional_test2():
    ### --- BLOCK BEGIN 86
    decoder = TomlDecoder(func_dict)
    input_str = "[{'x' = 1}]"
    res = decoder.load_array(input_str)
    assert(res == [{'x': 1}])
    input_str = "[{'x' = 1}, {'y' = 2}]"
    res = decoder.load_array(input_str)
    assert(res == [{'x': 1}, {'y': 2}])
    ### --- BLOCK END 86



def additional_test3():
    ### --- BLOCK BEGIN 87
    v = "abc\\"
    hexbytes = ['0064']
    prefix = 'u'
    res = _load_unicode_escapes(v, hexbytes, prefix)
    assert(res == 'abc\\u0064')
    ### --- BLOCK END 87



def additional_test4():
    ### --- BLOCK BEGIN 88
    v = "\\\\"
    res = _unescape(v)
    assert(res == '\\')
    v = "\\u"
    res = _unescape(v)
    assert(res == '\\u')
    ### --- BLOCK END 88



def additional_test5():
    ### --- BLOCK BEGIN 89
    s = """['"test"']"""
    t = loads(s, func_dict, None)
    assert(t == {'"test"': {}})
    s = """["abc"]"""
    t = loads(s, func_dict, None)
    assert(t == {'abc': {}})
    ### --- BLOCK END 89



### Preprocessing:

# We remove several tests due to the following reasons:

# 1. The lack of corresponding libraries in JS: "pathlib", "tzinfo"

# 2. Currently unsupported datatypes: "ordereddict", "decimal", "numpy"

# 3. Nondeterministic code or reflective code: `id()`, `time.now()`

# 4. Dynamic hierachy, or hierachy from built-in class

# These tests are not related to the main functionality of the program.

### Global Begin

### --- BLOCK BEGIN 0
TIME_RE = re.compile(r"([0-9]{2}):([0-9]{2}):([0-9]{2})(\.([0-9]{3,6}))?")

# Matches a TOML number, which allows underscores for readability

_number_with_underscores = re.compile('([0-9])(_([0-9]))*')

_groupname_re = re.compile(r'^[A-Za-z0-9_-]+$')

# Unescape TOML string values.

# content after the \

_escapes = ['0', 'b', 'f', 'n', 'r', 't', '"']

# What it should be replaced by

_escapedchars = ['\0', '\b', '\f', '\n', '\r', '\t', '\"']

# Used for substitution

_escape_to_escapedchars = dict(zip(_escapes, _escapedchars))

TEST_STR = """
[a]\r
b = 1\r
c = 2
"""

TEST_DICT = {"a": {"b": 1, "c": 2}}

test()

### --- BLOCK END 0
