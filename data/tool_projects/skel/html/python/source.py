
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

import re

import os

import sys

import json

import tracer_skip as tool_functions

def escape(s, quote):
    """
    Replace special characters "&", "<" and ">" to HTML-safe sequences.
    If the optional flag quote is true (the default), the quotation mark
    characters, both double quote (") and single quote (') characters are also
    translated.
    """
    ### --- BLOCK BEGIN 1
    s = s.replace("&", "&amp;")
    # Must be done first!
    s = s.replace("<", "&lt;")
    s = s.replace(">", "&gt;")
    if quote:
        s = s.replace('"', "&quot;")
        s = s.replace('\'', "&#x27;")
    return s
    ### --- BLOCK END 1



def _replace_charref(s):
    ### --- BLOCK BEGIN 2
    if s[0] == '#':
    # numeric charref
        if s[1] in 'xX':
            num = int(s[2:].rstrip(';'), 16)
        else:
            num = int(s[1:].rstrip(';'))
        if ((hasattr(tool_functions._invalid_charrefs, '__contains__') and tool_functions._invalid_charrefs.__contains__(num)) or (not hasattr(tool_functions._invalid_charrefs, '__contains__') and num in tool_functions._invalid_charrefs)):
            return tool_functions._invalid_charrefs[num]
        if 0xD800 <= num <= 0xDFFF or num > 0x10FFFF:
            return '\uFFFD'
        if ((hasattr(tool_functions._invalid_codepoints, '__contains__') and tool_functions._invalid_codepoints.__contains__(num)) or (not hasattr(tool_functions._invalid_codepoints, '__contains__') and num in tool_functions._invalid_codepoints)):
            return ''
        return chr(num)
    else:
    # named charref
        if ((hasattr(tool_functions._html5, '__contains__') and tool_functions._html5.__contains__(s)) or (not hasattr(tool_functions._html5, '__contains__') and s in tool_functions._html5)):
            return tool_functions._html5[s]
        # find the longest matching name (as defined by the standard)
        for x in range(len(s)-1, 1, -1):
            if ((hasattr(tool_functions._html5, '__contains__') and tool_functions._html5.__contains__(s[:x])) or (not hasattr(tool_functions._html5, '__contains__') and s[:x] in tool_functions._html5)):
                return tool_functions._html5[s[:x]] + s[x:]
        else:
            return '&' + s
    ### --- BLOCK END 2



def unescape(s):
    """
    Convert all named and numeric character references (e.g. &gt;, &#62;,
    &x3e;) in the string s to the corresponding unicode characters.
    This function uses the rules defined by the HTML 5 standard
    for both valid and invalid character references, and the list of
    HTML 5 named character references defined in html.entities.html5.
    """
    ### --- BLOCK BEGIN 3
    if '&' not in s:
        return s
    start = 0
    while True:
        match = _charref_regular_exp.search(s, start)
        # Search for the pattern
        if not match:  # If no more matches, break the loop
            break
        # Replace the matched text
        replacement = _replace_charref(match.group(1))
        s = s[:match.start()] + replacement + s[match.end():]
        # Update the start index to avoid infinite loop
        start = match.start() + len(replacement)
    return s
    ### --- BLOCK END 3



def ParserBase():
    """Parser base class which provides some common support methods used
    by the SGML/HTML and XHTML parsers."""
    def __init__():
        ### --- BLOCK BEGIN 4
        pass
        # if self.__class__ is ParserBase:
        #     raise RuntimeError(
        #         "ParserBase must be subclassed")
        ### --- BLOCK END 4
    
    
    
    def reset():
        ### --- BLOCK BEGIN 5
        class_var.lineno = 1
        class_var.offset = 0
        ### --- BLOCK END 5
    
    
    
    def getpos():
        """Return current line number and offset."""
        ### --- BLOCK BEGIN 6
        return class_var.lineno, class_var.offset
        ### --- BLOCK END 6
    
    
    
    # Internal -- update line number and offset.  This should be
    # called for each piece of data exactly once, in order -- in other
    # words the concatenation of all the input strings to this
    # function should be exactly the entire input.
    def updatepos(i, j):
        ### --- BLOCK BEGIN 7
        if i >= j:
            return j
        rawdata = class_var.rawdata
        nlines = rawdata.count("\n", i, j)
        if nlines:
            class_var.lineno = class_var.lineno + nlines
            pos = rawdata.rindex("\n", i, j)
            # Should not fail
            class_var.offset = j-(pos+1)
        else:
            class_var.offset = class_var.offset + j-i
        return j
        ### --- BLOCK END 7
    
    
    
    # TRANSLATION NOTE: this function is inside a class `ParserBase.`
    # Internal -- parse declaration (for use by subclasses).
    def parse_declaration(i):
        ### --- BLOCK BEGIN 8
        rawdata = class_var.rawdata
        j = i + 2
        assert rawdata[i:j] == "<!", "unexpected call to parse_declaration"
        if rawdata[j:j+1] == ">":
        # the empty comment <!>
            return j + 1
        if rawdata[j:j+1] in ("-", ""):
        # Start of comment followed by buffer boundary,
        # or just a buffer boundary.
            return -1
        # A simple, practical version could look like: ((name|stringlit) S*) + '>'
        n = len(rawdata)
        if rawdata[j:j+2] == '--': #comment
        # Locate --.*-- as the body of the comment
            return class_var.parse_comment(i, 1)
        elif rawdata[j] == '[': #marked section
        # Locate [statusWord [...arbitrary SGML...]] as the body of the marked section
        # Where statusWord is one of TEMP, CDATA, IGNORE, INCLUDE, RCDATA
        # Note that this is extended by Microsoft Office "Save as Web" function
        # to include [if...] and [endif].
            return class_var.parse_marked_section(i, 1)
        else: #all other declaration elements
            decltype, j = class_var._scan_name(j, i)
        if j < 0:
            return j
        if decltype == "doctype":
            class_var._decl_otherchars = ''
        while j < n:
            c = rawdata[j]
            if c == ">":
            # end of declaration syntax
                data = rawdata[i+2:j]
                if decltype == "doctype":
                    class_var.handle_decl(data)
                else:
                # According to the HTML5 specs sections "8.2.4.44 Bogus
                # comment state" and "8.2.4.45 Markup declaration open
                # state", a comment token should be emitted.
                # Calling unknown_decl provides more flexibility though.
                    class_var.unknown_decl(data)
                return j + 1
            if c in "\"'":
                m = _declstringlit.match(rawdata, j)
                if not m:
                    return -1
                    # incomplete
                j = m.end()
            elif c in "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ":
                name, j = class_var._scan_name(j, i)
            elif c in class_var._decl_otherchars:
                j = j + 1
            elif c == "[":
            # this could be handled in a separate doctype parser
                if decltype == "doctype":
                    j = class_var._parse_doctype_subset(j + 1, i)
                elif decltype in {"attlist", "linktype", "link", "element"}:
                # must tolerate []'d groups in a content model in an element declaration
                # also in data attribute specifications of attlist declaration
                # also link type declaration subsets in linktype declarations
                # also link attribute specification lists in link declarations
                    raise AssertionError("unsupported '[' char in %s declaration" % decltype)
                else:
                    raise AssertionError("unexpected '[' char in declaration")
            else:
                raise AssertionError("unexpected %r char in declaration" % rawdata[j])
            if j < 0:
                return j
        return -1
        # incomplete
        ### --- BLOCK END 8
    
    
    
    # TRANSLATION NOTE: these functions are inside a class `ParserBase.`
    # Internal -- parse a marked section
    # Override this to handle MS-word extension syntax <![if word]>content<![endif]>
    def parse_marked_section(i, report):
        ### --- BLOCK BEGIN 9
        rawdata= class_var.rawdata
        assert rawdata[i:i+3] == '<![', "unexpected call to parse_marked_section()"
        sectName, j = class_var._scan_name(i+3, i)
        if j < 0:
            return j
        if ((hasattr({"temp", "cdata", "ignore", "include", "rcdata"}, '__contains__') and {"temp", "cdata", "ignore", "include", "rcdata"}.__contains__(sectName)) or (not hasattr({"temp", "cdata", "ignore", "include", "rcdata"}, '__contains__') and sectName in {"temp", "cdata", "ignore", "include", "rcdata"})):
        # look for standard ]]> ending
            match= _markedsectionclose.search(rawdata, i+3)
        elif ((hasattr({"if", "else", "endif"}, '__contains__') and {"if", "else", "endif"}.__contains__(sectName)) or (not hasattr({"if", "else", "endif"}, '__contains__') and sectName in {"if", "else", "endif"})):
        # look for MS Office ]> ending
            match= _msmarkedsectionclose.search(rawdata, i+3)
        else:
            raise AssertionError(
            'unknown status keyword %r in marked section' % rawdata[i+3:j]
            )
        if not match:
            return -1
        if report:
            j = match.start(0)
            class_var.unknown_decl(rawdata[i+3: j])
        return match.end(0)
        ### --- BLOCK END 9
    
    
    
    # Internal -- parse comment, return length or -1 if not terminated
    def parse_comment(i, report):
        ### --- BLOCK BEGIN 10
        rawdata = class_var.rawdata
        if rawdata[i:i+4] != '<!--':
            raise AssertionError('unexpected call to parse_comment()')
        match = _commentclose.search(rawdata, i+4)
        if not match:
            return -1
        if report:
            j = match.start(0)
            class_var.handle_comment(rawdata[i+4: j])
        return match.end(0)
        ### --- BLOCK END 10
    
    
    
    # TRANSLATION NOTE: this function is inside a class `ParserBase.`
    # Internal -- scan past the internal subset in a <!DOCTYPE declaration,
    # returning the index just past any whitespace following the trailing ']'.
    def _parse_doctype_subset(i, declstartpos):
        ### --- BLOCK BEGIN 11
        rawdata = class_var.rawdata
        n = len(rawdata)
        j = i
        while j < n:
            c = rawdata[j]
            if c == "<":
                s = rawdata[j:j+2]
                if s == "<":
                # end of buffer; incomplete
                    return -1
                if s != "<!":
                    class_var.updatepos(declstartpos, j + 1)
                    raise AssertionError(
                    "unexpected char in internal subset (in %r)" % s
                    )
                if (j + 2) == n:
                # end of buffer; incomplete
                    return -1
                if (j + 4) > n:
                # end of buffer; incomplete
                    return -1
                if rawdata[j:j+4] == "<!--":
                    j = class_var.parse_comment(j, 0)
                    if j < 0:
                        return j
                    continue
                name, j = class_var._scan_name(j + 2, declstartpos)
                if j == -1:
                    return -1
                if name not in {"attlist", "element", "entity", "notation"}:
                    class_var.updatepos(declstartpos, j + 2)
                    raise AssertionError(
                    "unknown declaration %r in internal subset" % name
                    )
                # handle the individual names
                meth = getattr(class_var, "_parse_doctype_" + name)
                j = meth(j, declstartpos)
                if j < 0:
                    return j
            elif c == "%":
            # parameter entity reference
                if (j + 1) == n:
                # end of buffer; incomplete
                    return -1
                s, j = class_var._scan_name(j + 1, declstartpos)
                if j < 0:
                    return j
                if rawdata[j] == ";":
                    j = j + 1
            elif c == "]":
                j = j + 1
                while j < n and rawdata[j].isspace():
                    j = j + 1
                if j < n:
                    if rawdata[j] == ">":
                        return j
                    class_var.updatepos(declstartpos, j)
                    raise AssertionError("unexpected char after internal subset")
                else:
                    return -1
            elif c.isspace():
                j = j + 1
            else:
                class_var.updatepos(declstartpos, j)
                raise AssertionError("unexpected char %r in internal subset" % c)
        # end of buffer reached
        return -1
        ### --- BLOCK END 11
    
    
    
    # TRANSLATION NOTE: this function is inside a class `ParserBase.`
    # Internal -- scan past <!ELEMENT declarations
    def _parse_doctype_element(i, declstartpos):
        ### --- BLOCK BEGIN 12
        name, j = class_var._scan_name(i, declstartpos)
        if j == -1:
            return -1
        # style content model; just skip until '>'
        rawdata = class_var.rawdata
        if '>' in rawdata[j:]:
            return rawdata.find(">", j) + 1
        return -1
        ### --- BLOCK END 12
    
    
    
    # Internal -- scan past <!ATTLIST declarations
    def _parse_doctype_attlist(i, declstartpos):
        ### --- BLOCK BEGIN 13
        pass
        # Not reachable
        ### --- BLOCK END 13
    
    
    
    # TRANSLATION NOTE: these functions are inside a class `ParserBase.`
    # Internal -- scan past <!NOTATION declarations
    def _parse_doctype_notation(i, declstartpos):
        ### --- BLOCK BEGIN 14
        name, j = class_var._scan_name(i, declstartpos)
        if j < 0:
            return j
        rawdata = class_var.rawdata
        while 1:
            c = rawdata[j:j+1]
            if not c:
            # end of buffer; incomplete
                return -1
            if c == '>':
                return j + 1
            if c in "'\"":
                m = _declstringlit.match(rawdata, j)
                if not m:
                    return -1
                j = m.end()
            else:
                name, j = class_var._scan_name(j, declstartpos)
                if j < 0:
                    return j
        ### --- BLOCK END 14
    
    
    
    # TRANSLATION NOTE: these functions are inside a class `ParserBase.`
    # Internal -- scan past <!ENTITY declarations
    def _parse_doctype_entity(i, declstartpos):
        ### --- BLOCK BEGIN 15
        rawdata = class_var.rawdata
        if rawdata[i:i+1] == "%":
            j = i + 1
            while 1:
                c = rawdata[j:j+1]
                if not c:
                    return -1
                if c.isspace():
                    j = j + 1
                else:
                    break
        else:
            j = i
        name, j = class_var._scan_name(j, declstartpos)
        if j < 0:
            return j
        while 1:
            c = class_var.rawdata[j:j+1]
            if not c:
                return -1
            if c in "'\"":
                m = _declstringlit.match(rawdata, j)
                if m:
                    j = m.end()
                else:
                    return -1
                    # incomplete
            elif c == ">":
                return j + 1
            else:
                name, j = class_var._scan_name(j, declstartpos)
                if j < 0:
                    return j
        ### --- BLOCK END 15
    
    
    
    # TRANSLATION NOTE: these functions are inside a class `ParserBase.`
    # Internal -- scan a name token and the new position and the token, or
    # return -1 if we've reached the end of the buffer.
    def _scan_name(i, declstartpos):
        ### --- BLOCK BEGIN 16
        rawdata = class_var.rawdata
        n = len(rawdata)
        if i == n:
            return SCAN_NAME_DEFAULT
        m = _declname.match(rawdata, i)
        if m:
            s = m.group()
            name = s.strip()
            if (i + len(s)) == n:
                return SCAN_NAME_DEFAULT
                # end of buffer
            return name.lower(), m.end()
        else:
            class_var.updatepos(declstartpos, i)
            raise AssertionError(
            "expected name token at %r" % rawdata[declstartpos:declstartpos+20]
            )
        ### --- BLOCK END 16
    
    
    
    # To be overridden -- handlers for unknown objects
    def unknown_decl(data):
        ### --- BLOCK BEGIN 17
        pass
        ### --- BLOCK END 17
    
    
    
    class_var = SkelClass('ParserBase')
    class_var.__init__ = __init__
    class_var.reset = reset
    class_var.getpos = getpos
    class_var.updatepos = updatepos
    class_var.parse_declaration = parse_declaration
    class_var.parse_marked_section = parse_marked_section
    class_var.parse_comment = parse_comment
    class_var._parse_doctype_subset = _parse_doctype_subset
    class_var._parse_doctype_element = _parse_doctype_element
    class_var._parse_doctype_attlist = _parse_doctype_attlist
    class_var._parse_doctype_notation = _parse_doctype_notation
    class_var._parse_doctype_entity = _parse_doctype_entity
    class_var._scan_name = _scan_name
    class_var.unknown_decl = unknown_decl
    __init__()
    return class_var


def HTMLParser(param_0):
    """Find tags and other markup and call handler functions.

    Usage:
        p = HTMLParser()
        p.feed(data)
        ...
        p.close()

    Start tags are handled by calling self.handle_starttag() or
    self.handle_startendtag(); end tags by self.handle_endtag().  The
    data between tags is passed from the parser to the derived class
    by calling self.handle_data() with the data as argument (the data
    may be split up in arbitrary chunks).  If convert_charrefs is
    True the character references are converted automatically to the
    corresponding Unicode character (and self.handle_data() is no
    longer split in chunks), otherwise they are passed by calling
    self.handle_entityref() or self.handle_charref() with the string
    containing respectively the named or numeric reference as the
    argument.
    """
    def __init__(convert_charrefs):
        """Initialize and reset this instance.
    
            If convert_charrefs is True (the default), all character references
            are automatically converted to the corresponding Unicode characters.
            """
        ### --- BLOCK BEGIN 18
        class_var.CDATA_CONTENT_ELEMENTS = CDATA_CONTENT_ELEMENTS
        class_var.convert_charrefs = convert_charrefs
        class_var.reset()
        ### --- BLOCK END 18
    
    
    
    def reset():
        """Reset this instance.  Loses all unprocessed data."""
        ### --- BLOCK BEGIN 19
        class_var.rawdata = ''
        class_var.lasttag = '???'
        class_var.interesting = interesting_normal
        class_var.cdata_elem = None
        # ParserBase.reset(self)
        class_var.lineno = 1
        class_var.offset = 0
        ### --- BLOCK END 19
    
    
    
    def feed(data):
        r"""Feed data to the parser.
    
            Call this as often as you want, with as little or as much text
            as you want (may include '\n').
            """
        ### --- BLOCK BEGIN 20
        class_var.rawdata = class_var.rawdata + data
        class_var.goahead(0)
        ### --- BLOCK END 20
    
    
    
    def close():
        """Handle any buffered data."""
        ### --- BLOCK BEGIN 21
        class_var.goahead(1)
        ### --- BLOCK END 21
    
    
    
    def get_starttag_text():
        """Return full source of start tag: '<...>'."""
        ### --- BLOCK BEGIN 22
        return class_var.__starttag_text
        ### --- BLOCK END 22
    
    
    
    def set_cdata_mode(elem):
        ### --- BLOCK BEGIN 23
        class_var.cdata_elem = elem.lower()
        class_var.interesting = re.compile(r'</\s*%s\s*>' % class_var.cdata_elem, re.I)
        ### --- BLOCK END 23
    
    
    
    def clear_cdata_mode():
        ### --- BLOCK BEGIN 24
        class_var.interesting = interesting_normal
        class_var.cdata_elem = None
        ### --- BLOCK END 24
    
    
    
    # TRANSLATION NOTE: the following function(s) is inside a class `HTMLParser`
    # Internal -- handle data as far as reasonable.  May leave state
    # and data to be processed by a subsequent call.  If 'end' is
    # true, force handling all data as if followed by EOF marker.
    def goahead(end):
        def handle_leftangle():
            nonlocal i
            ### --- BLOCK BEGIN 25
            if starttagopen.match(rawdata, i): # < + letter
                k = class_var.parse_starttag(i)
            elif rawdata.startswith("</", i):
                k = class_var.parse_endtag(i)
            elif rawdata.startswith("<!--", i):
                k = class_var.parse_comment(i, 1)
            elif rawdata.startswith("<?", i):
                k = class_var.parse_pi(i)
            elif rawdata.startswith("<!", i):
                k = class_var.parse_html_declaration(i)
            elif (i + 1) < n:
                class_var.handle_data("<")
                k = i + 1
            else:
                return "break"
            if k < 0:
                if not end:
                    return "break"
                k = rawdata.find('>', i + 1)
                if k < 0:
                    k = rawdata.find('<', i + 1)
                    if k < 0:
                        k = i + 1
                else:
                    k += 1
                if class_var.convert_charrefs and not class_var.cdata_elem:
                    class_var.handle_data(unescape(rawdata[i:k]))
                else:
                    class_var.handle_data(rawdata[i:k])
            i = class_var.updatepos(i, k)
            ### --- BLOCK END 25
        
        
        
        def handle_charref():
            nonlocal i
            ### --- BLOCK BEGIN 26
            match = charref.match(rawdata, i)
            if match:
                name = match.group()[2:-1]
                class_var.handle_charref(name)
                k = match.end()
                if not rawdata.startswith(';', k-1):
                    k = k - 1
                i = class_var.updatepos(i, k)
                return "continue"
            else:
                if ";" in rawdata[i:]:  # bail by consuming &#
                    class_var.handle_data(rawdata[i:i+2])
                    i = class_var.updatepos(i, i+2)
                return "break"
            ### --- BLOCK END 26
        
        
        
        def handle_entityref():
            nonlocal i
            ### --- BLOCK BEGIN 27
            match = entityref.match(rawdata, i)
            if match:
                name = match.group(1)
                class_var.handle_entityref(name)
                k = match.end()
                if not rawdata.startswith(';', k-1):
                    k = k - 1
                i = class_var.updatepos(i, k)
                return "continue"
            match = incomplete.match(rawdata, i)
            if match:
            # match.group() will contain at least 2 chars
                if end and match.group() == rawdata[i:]:
                    k = match.end()
                    if k <= i:
                        k = n
                    i = class_var.updatepos(i, i + 1)
                # incomplete
                return "break"
            elif (i + 1) < n:
            # not the end of the buffer, and can't be confused
            # with some other construct
                class_var.handle_data("&")
                i = class_var.updatepos(i, i + 1)
            else:
                return "break"
            ### --- BLOCK END 27
        
        
        
        ### --- BLOCK BEGIN 28
        rawdata = class_var.rawdata
        i = 0
        n = len(rawdata)
        while i < n:
            if class_var.convert_charrefs and not class_var.cdata_elem:
                j = rawdata.find('<', i)
                if j < 0:
                    amppos = rawdata.rfind('&', max(i, n-34))
                    if (amppos >= 0 and
                    not re.compile(r'[\s;]').search(rawdata, amppos)):
                        break
                        # wait till we get all the text
                    j = n
            else:
                match = class_var.interesting.search(rawdata, i)
                # < or &
                if match:
                    j = match.start()
                else:
                    if class_var.cdata_elem:
                        break
                    j = n
            if i < j:
                if class_var.convert_charrefs and not class_var.cdata_elem:
                    class_var.handle_data(unescape(rawdata[i:j]))
                else:
                    class_var.handle_data(rawdata[i:j])
            i = class_var.updatepos(i, j)
            if i == n:     break
            if rawdata.startswith('<', i):
                act = handle_leftangle()
                if act == "break":
                    break
                elif act == "continue":
                    continue
                else:
                    pass
            elif rawdata.startswith("&#", i):
                _act = handle_charref()
                if _act == "break":
                    break
                elif _act == "continue":
                    continue
                else:
                    pass
            elif rawdata.startswith('&', i):
                _act = handle_entityref()
                if _act == "break":
                    break
                elif _act == "continue":
                    continue
                else:
                    pass
            else:
                assert 0, "interesting.search() lied"
        # end while
        if end and i < n and not class_var.cdata_elem:
            if class_var.convert_charrefs and not class_var.cdata_elem:
                class_var.handle_data(unescape(rawdata[i:n]))
            else:
                class_var.handle_data(rawdata[i:n])
            i = class_var.updatepos(i, n)
        class_var.rawdata = rawdata[i:]
        ### --- BLOCK END 28
    
    
    
    # TRANSLATION NOTE: the following function(s) is inside a class `HTMLParser`
    # Internal -- parse html declarations, return length or -1 if not terminated
    # See w3.org/TR/html5/tokenization.html#markup-declaration-open-state
    # See also parse_declaration in _markupbase
    def parse_html_declaration(i):
        ### --- BLOCK BEGIN 29
        rawdata = class_var.rawdata
        assert rawdata[i:i+2] == '<!', ('unexpected call to '
        'parse_html_declaration()')
        if rawdata[i:i+4] == '<!--':
        # this case is actually already handled in goahead()
            return class_var.parse_comment(i, 1)
        elif rawdata[i:i+3] == '<![':
            return class_var.parse_marked_section(i, 1)
        elif rawdata[i:i+9].lower() == '<!doctype':
        # find the closing >
            gtpos = rawdata.find('>', i+9)
            if gtpos == -1:
                return -1
            class_var.handle_decl(rawdata[i+2:gtpos])
            return gtpos+1
        else:
            return class_var.parse_bogus_comment(i, 1)
        ### --- BLOCK END 29
    
    
    
    # Internal -- parse bogus comment, return length or -1 if not terminated
    # see http://www.w3.org/TR/html5/tokenization.html#bogus-comment-state
    def parse_bogus_comment(i, report):
        ### --- BLOCK BEGIN 30
        rawdata = class_var.rawdata
        assert rawdata[i:i+2] in ('<!', '</'), ('unexpected call to '
        'parse_comment()')
        pos = rawdata.find('>', i+2)
        if pos == -1:
            return -1
        if report:
            class_var.handle_comment(rawdata[i+2:pos])
        return pos + 1
        ### --- BLOCK END 30
    
    
    
    # Internal -- parse processing instr, return end or -1 if not terminated
    def parse_pi(i):
        ### --- BLOCK BEGIN 31
        rawdata = class_var.rawdata
        assert rawdata[i:i+2] == '<?', 'unexpected call to parse_pi()'
        match = tool_functions.piclose.search(rawdata, i+2)
        # >
        if not match:
            return -1
        j = match.start()
        class_var.handle_pi(rawdata[i+2: j])
        j = match.end()
        return j
        ### --- BLOCK END 31
    
    
    
    # TRANSLATION NOTE: the following function(s) is inside a class `HTMLParser`
    # Internal -- handle starttag, return end or -1 if not terminated
    def parse_starttag(i):
        ### --- BLOCK BEGIN 32
        class_var.__starttag_text = None
        endpos = class_var.check_for_whole_start_tag(i)
        if endpos < 0:
            return endpos
        rawdata = class_var.rawdata
        class_var.__starttag_text = rawdata[i:endpos]
        # Now parse the data between i+1 and j into a tag and attrs
        attrs = []
        match = tagfind_tolerant.match(rawdata, i+1)
        assert match, 'unexpected call to parse_starttag()'
        k = match.end()
        class_var.lasttag = tag = match.group(1).lower()
        while k < endpos:
            m = attrfind_tolerant.match(rawdata, k)
            if not m:
                break
            attrname, rest, attrvalue = m.group(1, 2, 3)
            if not rest:
                attrvalue = None
            elif attrvalue[:1] == '\'' == attrvalue[-1:] or \
                             attrvalue[:1] == '"' == attrvalue[-1:]:
                attrvalue = attrvalue[1:-1]
            if attrvalue:
                attrvalue = unescape(attrvalue)
            attrs.append((attrname.lower(), attrvalue))
            k = m.end()
        end = rawdata[k:endpos].strip()
        if end not in (">", "/>"):
            class_var.handle_data(rawdata[i:endpos])
            return endpos
        if end.endswith('/>'):
        # XHTML-style empty tag: <span attr="value" />
            class_var.handle_startendtag(tag, attrs)
        else:
            class_var.handle_starttag(tag, attrs)
            if tag in class_var.CDATA_CONTENT_ELEMENTS:
                class_var.set_cdata_mode(tag)
        return endpos
        ### --- BLOCK END 32
    
    
    
    # TRANSLATION NOTE: the following function(s) is inside a class `HTMLParser`
    # Internal -- check to see if we have a complete starttag; return end
    # or -1 if incomplete.
    def check_for_whole_start_tag(i):
        ### --- BLOCK BEGIN 33
        rawdata = class_var.rawdata
        m = locatestarttagend_tolerant.match(rawdata, i)
        if m:
            j = m.end()
            next = rawdata[j:j+1]
            if next == ">":
                return j + 1
            if next == "/":
                if rawdata.startswith("/>", j):
                    return j + 2
                if rawdata.startswith("/", j):
                # buffer boundary
                    return -1
                # else bogus input
                if j > i:
                    return j
                else:
                    return i + 1
            if next == "":
            # end of input
                return -1
            if next in ("abcdefghijklmnopqrstuvwxyz=/"
            "ABCDEFGHIJKLMNOPQRSTUVWXYZ"):
            # end of input in or before attribute value, or we have the
            # '/' from a '/>' ending
                return -1
            if j > i:
                return j
            else:
                return i + 1
        raise AssertionError("we should not get here!")
        ### --- BLOCK END 33
    
    
    
    # TRANSLATION NOTE: the following function(s) is inside a class `HTMLParser`
    # Internal -- parse endtag, return end or -1 if incomplete
    def parse_endtag(i):
        ### --- BLOCK BEGIN 34
        rawdata = class_var.rawdata
        assert rawdata[i:i+2] == "</", "unexpected call to parse_endtag"
        match = endendtag.search(rawdata, i+1)
        # >
        if not match:
            return -1
        gtpos = match.end()
        match = endtagfind.match(rawdata, i)
        # </ + tag + >
        if not match:
            if class_var.cdata_elem is not None:
                class_var.handle_data(rawdata[i:gtpos])
                return gtpos
            # find the name: w3.org/TR/html5/tokenization.html#tag-name-state
            namematch = tagfind_tolerant.match(rawdata, i+2)
            if not namematch:
            # w3.org/TR/html5/tokenization.html#end-tag-open-state
                if rawdata[i:i+3] == '</>':
                    return i+3
                else:
                    return class_var.parse_bogus_comment(i, 1)
            tagname = namematch.group(1).lower()
            # consume and ignore other stuff between the name and the >
            # Note: this is not 100% correct, since we might have things like
            # </tag attr=">">, but looking for > after the name should cover
            # most of the cases and is much simpler
            gtpos = rawdata.find('>', namematch.end())
            class_var.handle_endtag(tagname)
            return gtpos+1
        elem = match.group(1).lower()
        # script or style
        if class_var.cdata_elem is not None:
            if elem != class_var.cdata_elem:
                class_var.handle_data(rawdata[i:gtpos])
                return gtpos
        class_var.handle_endtag(elem)
        class_var.clear_cdata_mode()
        return gtpos
        ### --- BLOCK END 34
    
    
    
    # TRANSLATION NOTE: the following function(s) is inside a class `HTMLParser`
    # Overridable -- finish processing of start+end tag: <tag.../>
    def handle_startendtag(tag, attrs):
        ### --- BLOCK BEGIN 35
        class_var.handle_starttag(tag, attrs)
        class_var.handle_endtag(tag)
        ### --- BLOCK END 35
    
    
    
    # Overridable -- handle start tag
    def handle_starttag(tag, attrs):
        ### --- BLOCK BEGIN 36
        pass
        ### --- BLOCK END 36
    
    
    
    # Overridable -- handle end tag
    def handle_endtag(tag):
        ### --- BLOCK BEGIN 37
        pass
        ### --- BLOCK END 37
    
    
    
    # Overridable -- handle character reference
    def handle_charref(name):
        ### --- BLOCK BEGIN 38
        pass
        ### --- BLOCK END 38
    
    
    
    # Overridable -- handle entity reference
    def handle_entityref(name):
        ### --- BLOCK BEGIN 39
        pass
        ### --- BLOCK END 39
    
    
    
    # Overridable -- handle data
    def handle_data(data):
        ### --- BLOCK BEGIN 40
        pass
        ### --- BLOCK END 40
    
    
    
    # Overridable -- handle comment
    def handle_comment(data):
        ### --- BLOCK BEGIN 41
        pass
        ### --- BLOCK END 41
    
    
    
    # Overridable -- handle declaration
    def handle_decl(decl):
        ### --- BLOCK BEGIN 42
        pass
        ### --- BLOCK END 42
    
    
    
    # Overridable -- handle processing instruction
    def handle_pi(data):
        ### --- BLOCK BEGIN 43
        pass
        ### --- BLOCK END 43
    
    
    
    def unknown_decl(data):
        ### --- BLOCK BEGIN 44
        pass
        ### --- BLOCK END 44
    
    
    
    class_var = ParserBase()
    class_var._class_name = 'HTMLParser;' + class_var._class_name
    class_var.__init__ = __init__
    class_var.reset = reset
    class_var.feed = feed
    class_var.close = close
    class_var.get_starttag_text = get_starttag_text
    class_var.set_cdata_mode = set_cdata_mode
    class_var.clear_cdata_mode = clear_cdata_mode
    class_var.goahead = goahead
    class_var.parse_html_declaration = parse_html_declaration
    class_var.parse_bogus_comment = parse_bogus_comment
    class_var.parse_pi = parse_pi
    class_var.parse_starttag = parse_starttag
    class_var.check_for_whole_start_tag = check_for_whole_start_tag
    class_var.parse_endtag = parse_endtag
    class_var.handle_startendtag = handle_startendtag
    class_var.handle_starttag = handle_starttag
    class_var.handle_endtag = handle_endtag
    class_var.handle_charref = handle_charref
    class_var.handle_entityref = handle_entityref
    class_var.handle_data = handle_data
    class_var.handle_comment = handle_comment
    class_var.handle_decl = handle_decl
    class_var.handle_pi = handle_pi
    class_var.unknown_decl = unknown_decl
    __init__(param_0)
    return class_var


def MyHTMLParserTester(*args):
    def handle_starttag(tag, attrs):
        ### --- BLOCK BEGIN 45
        print("Encountered a start tag:", tag, attrs)
        listener_event_list.append(("starttag", tag, attrs))
        ### --- BLOCK END 45
    
    
    
    def handle_endtag(tag):
        ### --- BLOCK BEGIN 46
        print("Encountered an end tag :", tag)
        listener_event_list.append(("endtag", tag))
        ### --- BLOCK END 46
    
    
    
    def handle_data(data):
        ### --- BLOCK BEGIN 47
        print("Encountered some data  :", data)
        listener_event_list.append(("data", data))
        ### --- BLOCK END 47
    
    
    
    def handle_comment(data):
        ### --- BLOCK BEGIN 48
        print("Encountered comment    :", data)
        listener_event_list.append(("comment", data))
        ### --- BLOCK END 48
    
    
    
    def handle_entityref(name):
        ### --- BLOCK BEGIN 49
        print("entityref:", name)
        listener_event_list.append(("entityref", name))
        ### --- BLOCK END 49
    
    
    
    def handle_charref(name):
        ### --- BLOCK BEGIN 50
        print("charref  name:", name)
        listener_event_list.append(("charref", name))
        ### --- BLOCK END 50
    
    
    
    def handle_decl(data):
        ### --- BLOCK BEGIN 51
        print("decl     data:", data)
        listener_event_list.append(("decl", data))
        ### --- BLOCK END 51
    
    
    
    def handle_pi(data):
        ### --- BLOCK BEGIN 52
        print("pi       data:", data)
        listener_event_list.append(("pi", data))
        ### --- BLOCK END 52
    
    
    
    def unknown_decl(data):
        ### --- BLOCK BEGIN 53
        print("unknown  data:", data)
        listener_event_list.append(("unknown", data))
        ### --- BLOCK END 53
    
    
    
    class_var = HTMLParser(*args)
    class_var._class_name = 'MyHTMLParserTester;' + class_var._class_name
    class_var.handle_starttag = handle_starttag
    class_var.handle_endtag = handle_endtag
    class_var.handle_data = handle_data
    class_var.handle_comment = handle_comment
    class_var.handle_entityref = handle_entityref
    class_var.handle_charref = handle_charref
    class_var.handle_decl = handle_decl
    class_var.handle_pi = handle_pi
    class_var.unknown_decl = unknown_decl
    return class_var


def test():
    ### --- BLOCK BEGIN 54
    p = MyHTMLParserTester(True)
    p.feed(_example_html)
    # print("----- call functions -----")
    listener_event_list.append(("PRINT", p.getpos()))
    listener_event_list.append(("PRINT", p.get_starttag_text()))
    listener_event_list.append(("PRINT", p.parse_declaration(0)))
    p.close()
    ### --- BLOCK END 54



def additional_test():
    ### --- BLOCK BEGIN 55
    p = MyHTMLParserTester(True)
    p.rawdata = "<!DOCTYPE html>"
    parse_res = p.parse_declaration(0)
    assert(parse_res == 15)
    p.reset()
    p.rawdata = "<!DOCTYPE '2'>"
    parse_res = p.parse_declaration(0)
    assert(parse_res == 14)
    p.reset()
    p.rawdata = "<!DOCTYPE [<!-->]> "
    parse_res = p.parse_declaration(0)
    assert(parse_res == -1)
    p.reset()
    p.rawdata = "<!DOCTYPE [%hello]> "
    parse_res = p.parse_declaration(0)
    assert(parse_res == 19)
    p.reset()
    p.rawdata = "<!DOCTYPE [ ]> "
    parse_res = p.parse_declaration(0)
    assert(parse_res == 14)
    p.reset()
    p.close()
    ### --- BLOCK END 55



def additional_test2():
    ### --- BLOCK BEGIN 56
    p = MyHTMLParserTester(True)
    p.convert_charrefs = False
    p.feed("&abc<")
    # parse_res = p.parse_declaration(0)
    # assert(parse_res == -1)
    p.reset()
    p.convert_charrefs = False
    p.feed("&#abc<")
    # parse_res = p.parse_declaration(0)
    # assert(parse_res == -1)
    p.reset()
    p.convert_charrefs = False
    p.feed("&<")
    # parse_res = p.parse_declaration(0)
    # assert(parse_res == -1)
    p.reset()
    p.convert_charrefs = False
    p.feed("&#<")
    # parse_res = p.parse_declaration(0)
    # assert(parse_res == -1)
    p.reset()
    p.close()
    ### --- BLOCK END 56



def additional_test3():
    ### --- BLOCK BEGIN 57
    p = MyHTMLParserTester(True)
    p.handle_startendtag("tag", [])
    p.reset()
    p.handle_charref("name")
    p.reset()
    p.handle_entityref("name")
    p.reset()
    p.handle_data("data")
    p.reset()
    p.handle_comment("data")
    p.reset()
    p.handle_decl("data")
    p.reset()
    p.handle_pi("data")
    p.reset()
    p.unknown_decl("data")
    p.reset()
    p = HTMLParser(True)
    p.handle_startendtag("tag", [])
    p.reset()
    p.handle_charref("name")
    p.reset()
    p.handle_entityref("name")
    p.reset()
    p.handle_data("data")
    p.reset()
    p.handle_comment("data")
    p.reset()
    p.handle_decl("data")
    p.reset()
    p.handle_pi("data")
    p.reset()
    p.unknown_decl("data")
    p.reset()
    p.close()
    ### --- BLOCK END 57



def additional_test4():
    ### --- BLOCK BEGIN 58
    p = HTMLParser(True)
    p.rawdata = "<abc/"
    parse_res = p.check_for_whole_start_tag(0)
    assert(parse_res == -1)
    p.reset()
    p.rawdata = '<tagname attr="value'
    parse_res = p.check_for_whole_start_tag(0)
    assert(parse_res == -1)
    p.reset()
    p.rawdata = '<tagname attr'
    parse_res = p.check_for_whole_start_tag(0)
    assert(parse_res == -1)
    p.reset()
    p.rawdata = '<tagname /'
    parse_res = p.check_for_whole_start_tag(0)
    assert(parse_res == -1)
    p.reset()
    p.rawdata = '<tagname attr = "value" /'
    parse_res = p.check_for_whole_start_tag(0)
    assert(parse_res == -1)
    p.reset()
    p.rawdata = '<tagname "value" /'
    parse_res = p.check_for_whole_start_tag(0)
    assert(parse_res == -1)
    p.reset()
    p.close()
    ### --- BLOCK END 58



def additional_test5():
    ### --- BLOCK BEGIN 59
    res = escape("abc<>/'", True)
    assert(res == "abc&lt;&gt;/&#x27;")
    res = escape("<>", True)
    assert(res == "&lt;&gt;")
    res = escape("abc", True)
    assert(res == "abc")
    res = escape("abc&", True)
    assert(res == "abc&amp;")
    res = unescape("abc&lt;&gt;/&#x27;")
    assert(res == "abc<>/'")
    res = unescape("&lt;&gt;")
    assert(res == "<>")
    res = unescape("abc")
    assert(res == "abc")
    res = unescape("abc&amp;")
    assert(res == "abc&")
    ### --- BLOCK END 59



def additional_test6():
    ### --- BLOCK BEGIN 60
    p = HTMLParser(True)
    p.rawdata = "element>"
    p._parse_doctype_element(0, 0)
    p.reset()
    p.rawdata = "attlist element"
    p._parse_doctype_attlist(0, 0)
    p.reset()
    p.rawdata = "notation element"
    p._parse_doctype_notation(0, 0)
    p.reset()
    p.rawdata = "notation'"
    p._parse_doctype_notation(0, 0)
    p.reset()
    p.rawdata = "%element element"
    p._parse_doctype_entity(0, 0)
    p.reset()
    p.close()
    ### --- BLOCK END 60



def additional_tests():
    ### --- BLOCK BEGIN 61
    additional_test()
    additional_test2()
    additional_test3()
    additional_test4()
    additional_test5()
    additional_test6()
    ### --- BLOCK END 61



### Global Begin

### --- BLOCK BEGIN 0
name2codepoint = {
    'AElig':    0x00c6, # latin capital letter AE = latin capital ligature AE, U+00C6 ISOlat1
    # rest omitted
}

codepoint2name = {}

entitydefs = {}

for (name, codepoint) in name2codepoint.items():
    codepoint2name[codepoint] = name
    entitydefs[name] = chr(codepoint)

del name, codepoint

_charref_regular_exp = tool_functions._charref_regular_exp

_declname = tool_functions._declname

_declstringlit = tool_functions._declstringlit

_commentclose = tool_functions._commentclose

_markedsectionclose = tool_functions._markedsectionclose

_msmarkedsectionclose = tool_functions._msmarkedsectionclose

interesting_normal = tool_functions.interesting_normal

incomplete = tool_functions.incomplete

entityref = tool_functions.entityref

charref = tool_functions.charref

starttagopen = tool_functions.starttagopen

piclose = tool_functions.piclose

commentclose = tool_functions.commentclose

tagfind_tolerant = tool_functions.tagfind_tolerant

attrfind_tolerant = tool_functions.attrfind_tolerant

locatestarttagend_tolerant = tool_functions.locatestarttagend_tolerant

endendtag = tool_functions.endendtag

endtagfind = tool_functions.endtagfind

_example_html = tool_functions._example_html

_charref_regular_exp_match = _charref_regular_exp.match

_declname_match = _declname.match

_declstringlit_match = _declstringlit.match

_commentclose_match = _commentclose.match

_declstringlit_match = _declstringlit.match

_declname_match = _declname.match

_declstringlit_match = _declstringlit.match

_declstringlit_match = _declstringlit.match

_markedsectionclose_match = _markedsectionclose.match

_msmarkedsectionclose_match = _msmarkedsectionclose.match

interesting_normal_match = interesting_normal.match

incomplete_match = incomplete.match

entityref_match = entityref.match

charref_match = charref.match

starttagopen_match = starttagopen.match

piclose_match = piclose.match

commentclose_match = commentclose.match

tagfind_tolerant_match = tagfind_tolerant.match

attrfind_tolerant_match = attrfind_tolerant.match

locatestarttagend_tolerant_match = locatestarttagend_tolerant.match

endendtag_match = endendtag.match

endtagfind_match = endtagfind.match

CDATA_CONTENT_ELEMENTS = ["script", "style"]

SCAN_NAME_DEFAULT = [None, -1]

listener_event_list = []

test()

additional_tests()

### --- BLOCK END 0
