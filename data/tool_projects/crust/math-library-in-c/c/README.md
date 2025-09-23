# CustomMath library

This is a custom implementation of the `math.h` library in C language. The `CustomMath` library provides basic mathematical functions, such as sine, cosine, logarithms, etc.

## Build

To build the `CustomMath` library, you need to have CMake installed on your system. 

To build the library, run the following command:
```
cmake -S . -B ./build
cmake --build ./build
```

To clean the build directory, run the following command:
```
make clean
```

## Usage

To use the `CustomMath` library in your project, you need to link it with your program. 

Add the following lines to your CMakeLists.txt file:
```cmake
add_subdirectory(path/to/CustomMath)
target_link_libraries(your_project CustomMath)
```

Include the header file in your C source code:
```c
#include "castom_math.h"
```

Then, you can use any function provided by the `CustomMath` library, such as `castom_sin()` or `castom_log()`.

### Overview of some "math.h" functions

| No. | Function | Description |
| --- | -------- | ----------- |
| 1 | `int castom_abs(int x)` | computes absolute value of an integer value |
| 2 | `long double castom_acos(double x)` | computes arc cosine |
| 3 | `long double castom_asin(double x)` | computes arc sine |
| 4 | `long double castom_atan(double x)` | computes arc tangent |
| 5 | `long double castom_ceil(double x)` | returns the nearest integer not less than the given value |
| 6 | `long double castom_cos(double x)` | computes cosine |
| 7 | `long double castom_exp(double x)` | returns e raised to the given power |
| 8 | `long double castom_fabs(double x)` | computes absolute value of a floating-point value |
| 9 | `long double castom_floor(double x)` | returns the nearest integer not greater than the given value |
| 10 | `long double castom_fmod(double x, double y)` | remainder of the floating-point division operation |
| 11 | `long double castom_log(double x)` | computes natural logarithm |
| 12 | `long double castom_pow(double base, double exp)` | raises a number to the given power |
| 13 | `long double castom_sin(double x)` | computes sine |
| 14 | `long double castom_sqrt(double x)` | computes square root |
| 15 | `long double castom_tan(double x)` | computes tangent |  



## Dependencies

The `CustomMath` library has no external dependencies.

## Development

To run the tests for the `CustomMath` library, you need to have the `check` library installed on your system. 

To build and run the tests, run the following command:
```
cmake -S ./test -B ./test/build
cmake --build ./test/build
cd ./test/build/
./TEST
```

To check the code for errors and warnings, run the following command:
```
make cppcheck
```

To format the code according to the Google C++ style guide, run the following command:
```
make clang-format
```

## License

The `CustomMath` library is released under the MIT License. See the [LICENSE](LICENSE) file for details.
