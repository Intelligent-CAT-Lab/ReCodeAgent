# JCCC (JabaCat C Compiler)
A C compiler and preprocessor written from scratch in C that targets x86-64 assembly

![CMake GitHub Workflow](https://img.shields.io/github/actions/workflow/status/jabacat/jccc/cmake.yml?style=for-the-badge)

## Build Instructions

Make sure you have `cmake` installed on your machine, then run
`./scripts/build.sh`. Then, you can run JCCC with `./build/jccc`.

## Usage

Only lexing is supported at this stage. To lex, run the following:
`jccc --token-dump <filename>`

## Docs

## 1. Testing

### 1.1 Building tests

Use the normal `./scripts/build.sh` that will also build an executable called `test_jccc`.

### 1.2 Running the tests

Run the executable `./test_jccc` and you should see a list of the tests being run.

```
Running tests from "test_lexer" ...
Running "test_ttype_name"
Running "test_ttype_from_string"
Running "test_ttype_many_chars"
Running "test_ttype_one_char"
Concluded tests from "test_lexer"
Running tests from "test_x86" ...
Running "test_init_int_literal"
Concluded tests from "test_x86"
```

For errors, no news is good news because `tassert`s that fail will show the failure but `tassert`s that succeed will not display anything.

### 1.3 Writing tests

Here is an example usage of the testing. This is from "lexer/test_lexer.c". For each module of code, create a test_{module_name} file. This file should include a test_{module_name} function that includes the testing_setup and the testing_cleanup functions.

```c
#include "lex.h"
#include <testing/test_utils.h>

int test_lexer() {
    testing_module_setup();

    test_ttype_from_string();

    testing_module_cleanup();
    return 0;
}
```

After this, include a call to this function in the "testing/main.c" file like
how it's done for test_lexer.

```c
#include "lexer/test_lexer.h"

int main() {
        test_lexer();

        return 0;
}
```

Finally, here is what a test might look like. Make sure to include a call to testing_func_setup at the start.

```c
int test_ttype_from_string() {
	testing_func_setup();

    tassert(ttype_from_string("1") == TT_LITERAL);
    tassert(ttype_from_string("1.2") == TT_LITERAL);

	// ...

    tassert(ttype_from_string(";") == TT_SEMI);

    return 0;
}
```

