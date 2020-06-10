# Koi Language Grammar

## Lexical Grammar

```
whitespace =
  | U+000A
```

## Syntax Grammar

```
program =
  | top-level * ;

top-level =
  | comment
  | declaration
  | expr ;

comment =
  | line-comment
  | block-comment ;

line-comment =
  | "//" line-comment-body ;

line-comment-body =
  | ~( \r | \n )* ;

block-comment =
  | "(*" block-comment-body * "*)" ;

block-comment-body =
  | block-comment
  | CHAR ;

[???]
doc-comment =
  | "///" line-comment-body NEW_LINE ( declaration )* ;

declaration =
  | function-declaration
  | module-declaration
  | type-declaration ;

expr =
  | fun-expr
  | if-expr
  | literal
  | let-expr ;

literal =
  | boolean-literal
  | CHAR
  | NUMBER
  | STRING ;

boolean-literal =
  | "true"
  | "false" ;
```
