#!/usr/bin/env python
"""
The expected output is below:

"""
from subprocess import Popen, PIPE, check_call
from os import remove

# N.B., this MUST be run from within the source directory!

# Read test
o = open("test/found", "w")
check_call(["./test/exampleRead", "test/foo.2bit"], stdout=o)
o.close()
check_call(["diff", "test/found", "test/expected"])

print("Passed!")
