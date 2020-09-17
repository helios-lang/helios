```
-- PRELUDE ---------------------------------------------------------------------

LITERAL ::=
  | literal-character
  | literal-float     -- TODO
  | literal-integer   -- TODO
  | literal-string
  ;

escape-sequence ::=
  | `\\` ( `\\` | `0` | `n` | `r` | `t` | `"` | ` ` | escape-sequence-composed )
  ;

escape-sequence-composed ::=
  | escape-sequence-hex
  | escape-sequence-unicode
  ;

escape-sequence-hex ::=
  | `x` HEXADECIMAL-DIGIT{1,2}
  ;

escape-sequence-unicode ::=
  | `u` `{` HEXADECIMAL-DIGIT{1,6} `}`
  ;

quoted-text ::=
  | ( escaped-character | ^`"` )
  ;

literal-character ::=
  | `'` ( escape-sequence | ^`'` ) `'`
  ;

literal-string ::=
  | literal-string-static
  | literal-string-interpolated
  | literal-string-raw
  ;

literal-string-static ::=
  | `"` quoted-text* `"`
  ;

literal-string-interpolated ::=
  | `f"` ( interpolated-expression | quoted-text )* `"`
  ;

-- TODO: Don't match `{{ ... }}` (escaped braces)
interpolated-expression ::=
  | `{`{1,} expression `}`
  ;

literal-string-raw ::=
  | `r"` ( ^`"` )* `"`
  ;

-- PROGRAM ---------------------------------------------------------------------

program ::=
  | top-level? EOF
  ;

top-level ::=
  | declaration
  | expression
  ;

-- COMMON ----------------------------------------------------------------------

generic-parameter-list ::=
  | `(` `of` generic-parameter-list-element
    ( `,` generic-parameter-list-element )* `,`? `)`
  ;

generic-parameter-list-element ::=
  | IDENTIFIER ( `:` generic-parameter-list-constraint-list )?
  ;

generic-parameter-list-constraint-list ::=
  | type ( `+` type )*
  ;

parameter-list ::=
  | parameter ( `,` parameter-list? )?
  ;

parameter ::=
  | IDENTIFIER type-annotation?
  ;

type-annotation ::=
  | `:` type
  ;

type ::=
  | IDENTIFER generic-parameter-list?
  | array-type
  | function-type
  | tuple-type
  ;

visibility ::=
  | `pub`
  | `internal` -- Perhaps find a shorter keyword for this?
  ;

visibility-parameterized ::=
  | visibility ( `(` ( `get` `;` `set` ) | `get` | `set` `)` )?
  ;

-- DECLARATIONS ----------------------------------------------------------------

declaration ::=
  | declaration-extend    -- TODO
  | declaration-function
  | declaration-module
  | declaration-trait     -- TODO
  | declaration-type
  | declaration-using     -- TODO
  ;

-- TODO: No `where` clause
declaration-function ::=
  | visibility? `fun` IDENTIFIER generic-parameter-list? `(` parameter-list `)`
    type-annotation declaration-function-where-clause?
    ( `=` expression | expression-block )
  ;

-- REFACTOR: This allows: `fun foo(of A, B)(a: A): B where A, B ::= ???`; i.e.
-- constraints are not enforced in where-clauses.
declaration-function-where-clause ::=
  | `where` generic-parameter-list-element
    ( `,` generic-parameter-list-element )* `,`?
  ;

declaration-module ::=
  | visibility? `module` IDENTIFIER `{` top-level* `}`
  ;

-- TODO: This doesn't take generic parameters into consideration
declaration-type ::=
  | visibility? `type` IDENTIFIER `=` ( type | declaration-type-new )
  ;

declaration-type-new ::=
  | enum
  | struct
  ;

-- COMPUTED-PROPERTIES ---------------------------------------------------------

computed-property-declaration ::=
  | visibility-parameterized? `var` IDENTIFIER type-annotation? `with`
    computed-property-get-clause computed-property-set-clause?
  ;

computed-property-get-clause ::=
  | `get` ( `=>` expression | expression-block )
  ;

computed-property-set-clause ::=
  | `set` ( `=>` expression | expression-block )
  ;

-- ENUMS -----------------------------------------------------------------------

enum ::=
  | `enum` enum-body
  ;

enum-body ::=
  | `{` enum-body-element* `}`
  ;

-- TODO: Should we allow type declarations in enum bodies?
enum-body-element ::=
  | computed-property
  | declaration-function
  | enum-body-element-case
  ;

enum-body-element-case ::=
  | `case` IDENTIFIER ( `(` enum-body-element-case-parameters? `)` )?
  ;

enum-body-element-case-parameters ::=
  | type ( `,` enum-body-element-case-parameters? )?
  ;

-- STRUCTS ---------------------------------------------------------------------

struct ::=
  | `struct` struct-body
  ;

struct-body ::=
  | `{` struct-body-element* `}`
  ;

-- TODO: Should we allow type declarations in struct bodies?
struct-body-element ::=
  | computed-property
  | declaration-function
  | struct-body-element-field
  ;

struct-body-element-field ::=
  | visibility? `let` IDENTIFIER
    ( type-annotation | type-annotation? `=` expression )
  ;

-- EXPRESSIONS -----------------------------------------------------------------

expression ::=
  | expression-binding
  | expression-block
  | expression-if
  | expression-loop
  | expression-match
  | LITERAL
  ;

expression-binding ::=
  | ( `let` | `var` ) BINDING-PATTERN
    ( type-annotation | type-annotation? `=` expression )
  ;

expression-block ::=
  | `{` expression-block-body `}`
  ;

-- TODO: Should the semicolon act a operator?
expression-block-body ::=
  | expression ( ( NL | `;` ) expression-block-body )?
  ;

-- TODO: if-let patterns
-- The else-clause should be mandatory except for cases when the true branch
-- (and consequent else-if branches) returns the unit type `()`.
expression-if ::=
  | `if` expression expression-block expression-if-else-clause?
  ;

expression-if-else-clause ::=
  | `else` ( expression-if | expression-block )
  ;

expression-loop ::=
  | for-loop
  | while-loop
  ;

expression-loop-for ::=
  | `for` BINDING-PATTERN `in` expression expression-block
  ;

expression-loop-while ::=
  | `while` expression expression-block
  ;

expression-match ::=
  | `match` expression expression-match-body
  ;

expression-match-body ::=
  | `{` expression-match-body-element+ `}`
  ;

expression-match-body-element ::=
  | `case` MATCH-PATTERN `=>` expression
  ;
```
