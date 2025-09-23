# Simple Lang
A simple language running on virtual machine written in C.

```
statement ::= "let" identifier "=" expression ";" | identifier "=" expression ";" | "dis" identifier ";" | "dis" expression ";"
expression ::= integer | identifier | expression operator expression
operator ::= "+" | "-"
```

