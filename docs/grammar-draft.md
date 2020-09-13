```
$WHITESPACE = ` ` | `\t` | `\n` | `\r` ;

program =
  | top-level? EOF
  ;

top-level =
  | declaration
  | expression
  ;

declaration =
  | extend-declaration
  | function-declaration
  | module-declaration
  | trait-declaration
  | type-declaration
  | using-declaration
  ;

type-annotation =
  | `:` type
  ;

visibility =
  | public
  | internal
  ;

visibility-parameterized =
  | visibility ( `(` ( `get` `;` `set` ) | `get` | `set` `)` )?
  ;

-- TODO: No `where` clause
function-declaration =
  | visibility? `fun` IDENTIFIER generic-parameter-list? `(` parameter-list `)` type-annotation ( `=>` expression | expression-block )
  ;

-- TODO: No trait constraints
generic-parameter-list =
  | `(` `of` IDENTIFIER ( `,` IDENTIFER )* `,`? `)`
  ;

parameter-list =
  | parameter ( `,` parameter-list )? `,`?
  ;

parameter =
  | IDENTIFIER type-annotation?
  ;

module-declaration =
  | visibility? `module` IDENTIFIER `{` top-level* `}`
  ;

type-declaration =
  | visibility? `type` IDENTIFIER `=` ( type | new-type-declaration )
  ;

type =
  | IDENTIFER generic-parameter-list?
  | array-type
  | function-type
  | tuple-type
  ;

new-type-declaration =
  | enum-declaration
  | struct-declaration
  ;

computed-property-declaration =
  | visibility-parameterized? `var` IDENTIFIER type-annotation? `with` computed-property-get-clause computed-property-set-clause?
  ;

computed-property-get-clause =
  | `get` ( `=>` expression | expression-block )
  ;

computed-property-set-clause =
  | `set` ( `=>` expression | expression-block )
  ;

enum-declaration =
  | `enum` enum-declaration-body
  ;

enum-declaration-body =
  | `{` enum-declaration-body-element* `}`
  ;

-- TODO: Should we allow type declaration in type bodies?
enum-declaration-body-element =
  | computed-property-declaration
  | enum-case
  | function-declaration
  ;

enum-case =
  | `case` IDENTIFIER ( `(` enum-case-type-list `)` )?
  ;

enum-case-type-list =
  | type ( `,` enum-case-type-list )? `,`?
  ;

struct-declaration =
  | `struct` struct-declaration-body
  ;

struct-declaration-body =
  | `{` struct-declaration-body-element* `}`
  ;

-- TODO: Should we allow type declaration in type bodies?
struct-declaration-body-element =
  | computed-property-declaration
  | function-declaration
  | struct-field-declaration
  ;

struct-field-declaration =
  | visibility? `let` IDENTIFIER type-annotation
  | visibility? `let` IDENTIFIER type-annotation? `=` expression
  ;

struct-case-type-list =
  | type ( `,` struct-case-type-list )? `,`?
  ;

expression =
  | binding-expression
  | expression-block
  | if-expression
  | loop-expression
  | match-expression
  ;

binding-expression =
  | ( `let` | `var` ) BINDING-PATTERN type-annotation? `=` expression
  ;

expression-block =
  | `{` expression-block-list `}`
  ;

expression-block-list =
  | expression ( `;` | `\n` ) ( expression-block-list )? expression
  ;

-- TODO: if-let patterns
if-expression =
  | `if` expression expression-block else-clause?
  ;

else-clause =
  | `else` if-expression
  | `else` expression-block
  ;

loop-expression =
  | for-loop
  | while-loop
  ;

for-loop =
  | `for` BINDING-PATTERN `in` expression expression-block
  ;

while-loop =
  | `while` expression expression-block
  ;

match-expression =
  | `match` expression `{` match-expression-case-clause+ `}`
  ;

match-expression-case-clause =
  | `case` PATTERN `=>` expression
  ;
```
