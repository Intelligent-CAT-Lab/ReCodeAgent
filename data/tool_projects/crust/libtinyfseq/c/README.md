# libtinyfseq

A single-file library (~150 LOC) for decoding FSEQ (.fseq) v2.0+ sequence files developed and popularized by the [fpp](https://github.com/FalconChristmas/fpp) and [xLights](https://github.com/smeighan/xLights) programs. Additional documentation for the file format is available at [Cryptkeeper/fseq-file-format](https://github.com/Cryptkeeper/fseq-file-format).

## Installation

- Download and copy `tinyfseq.h` into your project locally, or to your toolchain's include paths
- `#include "tinyfseq.h"` as expected (you may need to modify the path).
- Define `TINYFSEQ_IMPLEMENTATION` in a single C/C++ source code file ([more info on using single-file libraries](https://github.com/nothings/stb#how-do-i-use-these-libraries))

A short example of including libtinyfseq and printing the library version is available in the form of a [`test/build.c`](unit test).

## Compatibility

- libtinyfseq uses `stdint.h` for fixed-size int types
- libtinyfseq only supports FSEQ versions v2.x versions, with the schema initially released in 2018. Older v1.x files
  can be upgraded using the [xLights](https://github.com/smeighan/xLights) program.

## Usage

libtinyfseq provides decoding functions for the following components of a FSEQ file. See [tinyfseq.h](tinyfseq.h) for comments describing their specific usage.

| Function                  | Schema                                                            | Type                 |
| ------------------------- | ----------------------------------------------------------------- | -------------------- |
| `TFHeader_read`           | https://github.com/Cryptkeeper/fseq-file-format#header            | `TFHeader`           |
| `TFVarHeader_read`        | https://github.com/Cryptkeeper/fseq-file-format#variable          | `TFVarHeader`        |
| `TFChannelRange_read`     | https://github.com/Cryptkeeper/fseq-file-format#sparse-range      | `TFChannelRange`     |
| `TFCompressionBlock_read` | https://github.com/Cryptkeeper/fseq-file-format#compression-block | `TFCompressionBlock` |

All decoding functions return a `TFError` value, with `TF_OK` indicating success. If an error occurs, the value will be non-zero. An error string can be retrieved via `TFError_string` (and should _not_ be freed by the caller).

## License

See [LICENSE.txt](LICENSE.txt)
