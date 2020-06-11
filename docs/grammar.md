# Koi Language Grammar

## Lexical Grammar

```
whitespace =
  | U+000A
  | U+0009
```

## Syntax Grammar

```
program     = top_level* EOF ;

top_level   = decl
            | expr ;

decl        = fun_decl
            | module_decl
            | type_decl ;
expr        = if_expr
            | let_expr
            | literal_expr
            | match_expr ;

fun_decl    = "def" IDENTIFIER "(" parameters? ")" fun_decl_block ;
module_decl = "module" IDENTIFIER "=" module_decl_block ;
type_decl   = "type" IDENTIFIER type_parameter_list? "=" type_decl_block ;

if_expr     = "if" closed_expr "then" block if_expr_else_tail? ;
let_expr    = "let" let_expr_binding_pattern "=" block ;

block       = ( expr "\n" )+
```
