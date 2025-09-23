<p align="center"><img width="250px" src="res/logo.png"></p>
<h1 align="center">emlang</h1>
<p align="center">A stack-based emoticon esolang</p>

<p align="center">
	<a href="./LICENSE">
		<img alt="License" src="https://img.shields.io/badge/license-GPL v3-26c374?style=for-the-badge">
	</a>
	<a href="https://github.com/LordOfTrident/emlang/issues">
		<img alt="Issues" src="https://img.shields.io/github/issues/LordOfTrident/emlang?style=for-the-badge&color=4f79e4">
	</a>
	<a href="https://github.com/LordOfTrident/emlang/pulls">
		<img alt="GitHub pull requests" src="https://img.shields.io/github/issues-pr/LordOfTrident/emlang?style=for-the-badge&color=4f79e4">
	</a>
	<br><br><br>
</p>

A silly [emoticon](https://en.wikipedia.org/wiki/List_of_emoticons) [stack-based](https://en.wikipedia.org/wiki/Stack-oriented_programming) [esoteric programming language](https://en.wikipedia.org/wiki/Esoteric_programming_language).

## Table of contents
* [Hello world](#hello-world)
* [Quickstart](#quickstart)
* [Syntax](#syntax)
  * [Integers](#integers)
  * [Strings](#strings)
  * [Comments](#comments)
  * [Keywords](#keywords)
* [Bugs](#bugs)

## Hello world
```
:O Hello, world! :)
```

## Quickstart
```sh
$ make
$ ./emlang -h
```

Try the examples from the [examples folder](./examples).

## Syntax
The syntax is composed of tokens separated by whitespaces. The tokens can be integers,
strings or keywords.

### Integers
Integers are composed of decimal digits only. Optionally, they can have a dash (`-`) at the start
to make them negative.
```
1024
-52
```

### Strings
Strings can be quoted or unquoted. Unquoted strings can contain any characters except whitespaces.
```
Hello!
I'm-an-unquoted-string.
```

Quoted strings start and end with double quotes (`"`) and can contain whitespaces.
```
"Hello, world!"
```

They also have escape sequences, see the table below.

| Sequence | Name            |
| -------- | --------------- |
| `\n`     | New line        |
| `\r`     | Return carriage |
| `\t`     | Tab             |
| `\f`     | Form-feed       |
| `\v`     | Vertical tab    |
| `\b`     | Backspace       |
| `\a`     | Alert           |
| `\"`     | Double quotes   |
| `\e`     | Escape          |
| `\\`     | Backslash       |

Example
```
"I can use quotes inside a quoted string:\n\t\"Look!\""
```

### Comments
Comments start with `:x` and last until the end of the line.
```
:x I am a comment
```

## Bugs
If you find any bugs, please create an issue and report them.
