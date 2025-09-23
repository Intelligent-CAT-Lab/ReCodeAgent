# Lambda Calculus Toy Lang

This is a toy lambda calculus language written in C!

To test it out, you can simply write a lambda expression such as:
```
type Nat
(@x : Nat.(x) var : Nat)
```

In a file such as `expr.lambda`.

You can run the evaluator as cli with `./lc`, or create a config file like this one:

```
file=expr.lambda
step_by_step_reduction=true
reduction_order=applicative
```
Reduction order can be `applicative` or `normal`.

Another useful thing you can do is have *definitions*.
Definitions are simply more lambda expressions that you bind to a name.
Everytime you write the name bound to the definition, before the reduction, the evaluator will expand the definition into its original expanded form. For example, assume a file `expr.lambda`:

```
type Nat
def a = @x : Nat.x

def b = @y : Nat.y

(a b)
```

It will reduce further to:

`(@x.x @y.y)` => `(@y.y)`

It is possible to import files, but the files imported can ONLY have definitions (type definitions and def statements).

Lambda expressions and unbound variables HAVE to be typed.

The typecheck step is, for now, simple typed, which means we only check if the function (lambda abstraction) type is the same type as its argument.

You can check some examples under the `examples` directory.

You can also run tests through: `make test`

## Syntax Highlighting

It is boring to write lambda expressions in plain uncolored characters.

If you want to highlight your `.lambda` files in `vim`, you can:

- Copy `vim-lambda` dir to `~/.vim/bundle/` with `cp ./vim-lambda ~/.vim/bundle/ -r`
- Copy syntax and ftdetect individually to `.vim/syntax` and `.vim/ftdetect`

Or use your plugin manage of preference, the two files are really simple.
