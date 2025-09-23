all: debug

debug:
	cd bld;                                   \
	make -f makefile-debug;                   \
	cd ..

release:
	cd bld;                                   \
	make -f makefile-release;                 \
	cd ..;

unittest:
	cd bld;                                   \
	make -f makefile-unittest;                \
	cd ..

clean-debug:
	cd bld;                                   \
	make -f makefile-debug clean;             \
	cd ..

clean-release:
	cd bld;                                   \
	make -f makefile-release clean;           \
	cd ..

clean-unittest:
	cd bld;                                   \
	make -f makefile-unittest clean;          \
	cd ..

clean: clean-debug clean-release clean-unittest

.PHONY: all debug release unittest clean-debug clean-release clean-unittest clean

