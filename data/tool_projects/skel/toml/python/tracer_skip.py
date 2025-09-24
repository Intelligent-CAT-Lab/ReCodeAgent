def get_input(test_case_name = None):
    decode_input = ""
    with open("./example.toml") as f:
        decode_input = f.read()
    
    decode_input = decode_input.split("################################################################################\n")
    for i in decode_input:
        if "## " + test_case_name in i:
            return i

test_str = """[[products]]
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
"""
