# worsp

Worsp is a minimal LISP interpreter written in C.

## Development

### VSCode extension

You can install the VSCode extension for syntax highlighting.

Please check https://marketplace.visualstudio.com/items?itemName=sosukesuzuki.worsp-vscode .

### `make format`

Run clang-format for all `*.c` and `*.h` files.

### `make main` and `make run-main`

Build and run main program. It execute worsp program from a file.

```
make run-main WORSP_FILE="./tmp/fact.wsp"
```

### `make lldb-main`

Build and run main program via lldb.

```
make lldb-main WORSP_FILE="./tmp/fact.wsp"
```

### `make test` and `make run-test`

Build `test.c` and run it.

### `make repl` and `make run-repl`

Build `repl.c` and run it.

### `make check-snapshot` and `make update-snapshot`

Run `./snapshot/fixtreus/*.wsp` files and checks and updates whether its standard output is equal to the one in `./snapshot/snapshot.json`.
