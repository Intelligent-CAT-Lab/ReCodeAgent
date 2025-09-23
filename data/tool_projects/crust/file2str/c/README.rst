
NAME
    file2strl, file2str

SYNOPSIS
    #include "file2str.h"

    char *file2strl(const char *path, unsigned long *len);

    char *file2str(const char *path);

DESCRIPTION
    file2str simply reads a file and returns the contents as a null terminated
    string.
    
INSTALLATION
    Use `clib <https://github.com/clibs/clib>`_ to install file2str into your project, eg:
    λ clib install willemt/file2str

LICENSE
    Public domain
