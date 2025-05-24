TODO:

```
program -> expression*

expression -> assignment | if | call | infix | primary

assignment -> identifier "=" expression

if -> "if" expression "do" block ("else" block)? "end"

block -> expression*

call -> identifier expression*

infix -> expression infix_op expression

infix_op -> "==" | "!=" | "<=" | ">=" | "<" | ">" | "+" | "-" | "*" | "/" | "="

primary -> identifier | integer | float | sring | "(" expression ")"

identifier -> letter (letter | digit | "_")*

integer -> digit+

float -> digit+ "." digit+

string -> "\"" character* "\""

letter -> "A".."Z" | "a".."z"

digit -> "0".."9"
```
