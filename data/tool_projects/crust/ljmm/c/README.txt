1. Introduction
===============

    On Linux/x86-64 box, Luajit is not able to allocate more than 1Gb memory[1],
  which is very restrictive to us. This work tries to provide a workaround such
  that Luajit can allocate about 2G memory.

    Due to the fact that Luajit uses a very compact representation of its
  internal data structure (TValue to be exact), it is difficult to support
  64-bit or 47-bit pointer or references. Currently, it can only support
  31-bit (not 32-bit) pointer/reference on x86-64 platform. 32-bit pointer
  will be unexpectly sign-extended to 64-bit quantity causing
  erroneous result[2].

    This workaround is only for Linux/x86-64. We use this workaround in the
  environment of Nginx with Openresty and Luajit.

    It does not see any negative performance impact at all. This workaround
  has been running in our online system for a quite a while as of I write
  this README (9/9/2015). We implemented a similar workaround before
  (https://github.com/cloudflare/luajit-mm), which is considered bit
  complicated, and never ever get chance to run in our system.


2. How to use this workaround in Nginx+Openresty+Luajit
=======================================================

    1. First, Make sure luajit library (i.e. the libluajit-x.y.so.z) is statically
  linked against Nginx (see Section xxx for the rationale). It can be done via
  one of following approaches:

    -. just delete libluajit-*.so from your system.

    -. if you build luajit from source code, add BUILDMODE=static to the
       make-utility command. The BUILDMODE=static instructs luajit to build
       static archive only.

    -. if you opt to change link-command-line, you can add "-Wl,-static" and
       "-Wl,-Bdynamic" around "-lluajit-x.y". If flag is to instruct gcc
       to pick up static archive only for luajit library.

         For instance, if the Nginx is linked with lua-nginx-module which is
       the only module use Luajit, you can substitute the occurrences of
       "-lluajit-5.1" in its config file into:
            "-Wl,-static  -lluajit-5.1 -Wl,-Bdynamic".

          If libluajit is statically linked, the command
        "Ldd /the/final/nginx | grep luajit" should produce nothing.

    2. Then, build this workaround by "cd /the/path/to/ljmm; make" which will
  build ljmm.o object file.

    3. Finally, tell Nginx to link ljmm.o and replace mmap64() system call with
  our wrapper function. This step can be done by adding following flag
  to Nginx's configure command:
    --with-ld-opt='... -Wl,--wrap=mmap64 /the/path/to/the/ljmm.o ...'

References:

[1] http://lua-users.org/lists/lua-l/2010-11/msg00241.html
[2] http://lua-users.org/lists/lua-l/2010-10/msg00688.html
