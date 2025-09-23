PROGS?=		totp test
PROGS.cross?=	totp32.exe test32.exe \
		totp64.exe test64.exe \
		totp.wasm

SRC.totp=	totp.c main.c
SRC.test=	totp.c test.c
SRC.wasm=	totp.c std.c

CC.win32?=	i686-w64-mingw32-gcc
CC.win64?=	x86_64-w64-mingw32-gcc
CC.wasm?=	clang -target wasm32

CFLAGS+=	-Wall -Wextra

CFLAGS.wasm+=	-Wall -Wextra
CFLAGS.wasm+=	-O2
CFLAGS.wasm+=	-nostdlib -fvisibility=hidden -DNO_STD

LDFLAGS.wasm+=	-s
LDFLAGS.wasm+=	-Wl,--no-entry
LDFLAGS.wasm+=	-Wl,--export-dynamic
LDFLAGS.wasm+=	-Wl,--import-memory

LINK.wasm=	${CC.wasm} ${CFLAGS.wasm} ${LDFLAGS.wasm}

-include config.mk

all:   ${PROGS}
cross: ${PROGS.cross}

clean:
	rm -f ${PROGS} ${PROGS.cross} *.o

check: all
	./test_1

cross-check: cross
	wine ./test32.exe
	wine ./test64.exe

totp: ${SRC.totp} *.h
	${LINK.c} -o $@ ${SRC.totp}

totp32.exe: ${SRC.totp} *.h
	${CC.win32} ${CFLAGS} -o $@ ${SRC.totp}

totp64.exe: ${SRC.totp} *.h
	${CC.win64} ${CFLAGS} -o $@ ${SRC.totp}

totp.wasm: ${SRC.wasm} *.h
	${LINK.wasm} -o $@ ${SRC.wasm}

test: ${SRC.test} *.h
	${LINK.c} -o test_1 ${SRC.test}
	

test32.exe: ${SRC.test} *.h
	${CC.win32} ${CFLAGS} -o $@ ${SRC.test}

test64.exe: ${SRC.test} *.h
	${CC.win64} ${CFLAGS} -o $@ ${SRC.test}

.PHONY: all cross clean check cross-check
