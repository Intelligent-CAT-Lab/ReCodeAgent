 
# ImpCheck - Immediate Massively Parallel Propositional Proof Checking

This suite of three executables (parser, checker, confirmer) can be used together with a parallel and/or distributed clause-sharing solver to check its reasoning on-the-fly during solving. The solver code does not need to be trusted; clauses are shared with cryptographically strong signatures which effectively transfer the guarantee of their soundness across checker instances. Individual solver threads are required to output LRAT proof information.

The software is released together with a 2024 SAT conference submission ("Scalable Trusted SAT Solving with on-the-fly LRAT Checking") – see https://github.com/domschrei/mallob-impcheck-data/ for more details.

![Three imps looking through magnifying glasses at a pile of tangled threads (via MS Designer Image Creator + GIMP)](https://dominikschreiber.de/img/impcheck.png)

## Compilation

```bash
mkdir build
cd build
cmake .. -DCMAKE_BUILD_TYPE=RELEASE -DIMPCHECK_WRITE_DIRECTIVES=0 -DIMPCHECK_FLUSH_ALWAYS=1
make
cd ..
```

### Flags

* `-DIMPCHECK_WRITE_DIRECTIVES=0`: Normal mode of operation
* `-DIMPCHECK_WRITE_DIRECTIVES=1`: Write each incoming directive into a separate binary file
* `-DIMPCHECK_WRITE_DIRECTIVES=2`: Write each incoming directive into a separate human-readable ASCII file

* `-DIMPCHECK_FLUSH_ALWAYS=0`: Flush checker's feedback pipe only for selected directives. Can be used (and is the most efficient) if the reading of feedback is done in a different thread than the writing of directives, or if reads are done in a non-blocking manner. CAN HANG otherwise, e.g., if a single thread forwards a clause derivation with a blocking write and then attempts a blocking read of the result.
* `-DIMPCHECK_FLUSH_ALWAYS=1`: Flush checker's feedback pipe after every single directive. Required if a single thread alternates between blocking reads and writes to the respective pipes. Safe, but may be slower.

### Secret Key

Our clause signing approach relies on a confidential 128-bit key $K$ shared among all trusted parties. In the current version of ImpCheck, $K$ is simply baked into the trusted processes at compilation time and expected to be inaccessible otherwise.

If you prefer to replace the default value of $K$ with a custom value, execute `python3 keygen.py x` (for some seed `x` that can be any string), which updates the source file `src/trusted/secret.c` with the new key.
Then recompile and replace *all* binaries – all parser, checker, and confirmer processes involved in a solving attempt must operate on the same $K$.

## Execution

### Isolated Execution

```
build/impcheck_parse -formula-input=<path/to/cnf> -fifo-parsed-formula=<path/to/output>
build/impcheck_check -fifo-directives=<path/to/input> -fifo-feedback=<path/to/output> [-check-model] [-lenient]
build/impcheck_confirm -formula-input=<path/to/cnf> -result=<10|20> -result-sig=<signature>
```
The intended mode of operation is that all paths specified via `-fifo-*` options are in fact named UNIX pipes precreated via `mkfifo`.
However, you can also specify actual, complete files to "replay" a sequence of written directives and to write the results persistently.

For `impcheck_check`, specify the optional argument `-check-model` if you also intend to get found models a.k.a. satisfying assignments checked (used together with Mallob's `-otfcm=1`). This can incur some memory overhead since deletion statements concerning original problem clauses will need to be ignored. Mallob mitigates this overhead to a degree by having each SAT process run only a single `impcheck_check` with `-check-model` enabled.

The optional argument `-lenient` lets the checker accept repeated clause imports (not derivations!) of _the same clause with the same ID_. In all other cases, `impcheck_check` aborts with an error when encountering a clause derivation or import with an existing ID.

### End-to-end Execution

Please examine the file `test/test_full.c` which is built into the executable `build/test_full`.
This code is (also) meant as an introduction and quick start on how these modules can be integrated in a parallel solver framework.

For an example within an actual parallel solver framework, please examine the following files of [MallobSat](https://github.com/domschrei/mallob/tree/proof23/):
* [`trusted_parser_process_adapter.hpp`](https://github.com/domschrei/mallob/blob/b9c7d1ec87d1511c541074562573af709536a8d2/src/app/sat/proof/trusted_parser_process_adapter.hpp)
  * Runs an instance of `impcheck_parse` to fetch the formula data and signature.
* [`cadical.cpp`](https://github.com/domschrei/mallob/blob/b9c7d1ec87d1511c541074562573af709536a8d2/src/app/sat/solvers/cadical.cpp)
  * Creates an `LratConnector` instance which is used to run and communicate with a checker instance. Initializes callbacks within CaDiCaL which connect to the `LratConnector`. Also consider the reactions for a solver finding a result.
* [`lrat_connector.hpp`](https://github.com/domschrei/mallob/blob/b9c7d1ec87d1511c541074562573af709536a8d2/src/app/sat/proof/lrat_connector.hpp)
  * Holds a `TrustedCheckerProcessAdapter` and lets solvers put LRAT directives (`LratOp` objects) into a SPSC ringbuffer. Spawns two separate threads: one reads directives from the ringbuffer and forwards them to the process adapter, and one receives results from the process adapter and reacts accordingly.
* [`trusted_checker_process_adapter.hpp`](https://github.com/domschrei/mallob/blob/b9c7d1ec87d1511c541074562573af709536a8d2/src/app/sat/proof/trusted_checker_process_adapter.hpp)
  * Runs an instance of `impcheck_check`, writing and reading directives/results as needed.

### Checker Input/Output Format

Each directive to a checker begins with a single character specifying the type of the directive, followed by a sequence of objects whose length (individually and in total) is given by the directive type and (in some cases) by certain "size" fields. A checker's output is similarly well-defined based on the shape of the directive. Please consult the definitions provided in `src/trusted/checker_interface.h` for the exact specification.
