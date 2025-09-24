const fs = require('fs');
function get_input(test_case_name){
    let decode_input = fs.readFileSync('./example.toml', 'utf8');

    decode_input = decode_input.split('################################################################################\n');
    
    for (let i of decode_input) {
        if (i.includes('## ' + test_case_name)) {
            return i;
        }
    }
}

function self_split(s, sep, count){
    var i = 0;
    var j = 0;
    var k = 0;
    var split = [];
    while (i < s.length && k < count) {
        if (s.slice(i, i+sep.length) === sep) {
            split.push(s.substring(j, i));
            j = i+sep.length;
            k += 1;
        }
        i += 1;
    }
    split.push(s.substring(j));
    var _return_value = split;
    return _return_value;
}

var test_str = `[[products]]
name = "Nail"
sku = 284758393
# This is a comment
color = "gray" # Hello World
# name = { first = 'Tom', last = 'Preston-Werner' }
# arr7 = [
#  1, 2, 3
# ]
# lines  = '''
# The first newline is
# trimmed in raw strings.
#   All other whitespace
#   is preserved.
# '''

[animals]
color = "gray" # col
fruits = "apple" # a = [1,2,3]
a = 3
b-comment = "a is 3"
`

module.exports = {get_input, test_str, self_split};