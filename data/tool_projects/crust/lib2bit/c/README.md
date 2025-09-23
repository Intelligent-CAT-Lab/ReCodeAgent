[![Build Status](https://travis-ci.org/dpryan79/lib2bit.svg?branch=master)](https://travis-ci.org/dpryan79/lib2bit)

A C library for accessing [2bit](https://genome.ucsc.edu/FAQ/FAQformat.html#format7) files. This is still a work in process. Note that functionality is mostly aimed at what will be needed in a python wrapper.

# Introduction

lib2bit is a C-based library for accessing [2bit files](https://genome.ucsc.edu/FAQ/FAQformat.html#format7). At the moment, only reading 2bit files is supported (there are no plans to change this, though if someone wants to submit a pull request...). Though it's unlikely to matter, 

The motivation for this project is due to needing fast access to 2bit files in [deepTools](https://github.com/fidelram/deepTools). Originally, we were using bx-python for this, which had the benefit of being easy to install and pretty quick. However, that wasn't compatible with python3, so we switched to [twobitreader](https://github.com/benjschiller/twobitreader). While doing everything we needed and working under both python2 and python3, it turns out that it has terrible performance (up to 1000x slow down in `computeGCBias`). Since we'd like to have our cake and eat it too, I began wrote a C library for convenient 2bit access and then [a python wrapper](https://github.com/dpryan79/py2bit) around it to work in python2 and 3.

# Installation

2bit files are very simple and there are no dependencies. Simply typing `make` should suffice for compilation. To install into a specific path (the default is `/usr/local`):

    make install prefix=/some/where/else

`lib2bit.so` and `lib2bit.a` will then be in `/some/where/else/lib` and `2bit.h` in `/some/where/else/include`.

# Example

See the `test/` directory for an example of using the library.
