# Koi Grammar

<pre>
<i id="program">program</i> ::=
  | <a href="#top-level-list">top-level-list</a>? <b>EOF</b> ;

<i id="top-level-list">top-level-list</i> ::=
  | <a href="#top-level">top-level</a> ( <b>NEWLINE</b> <a href="#top-level-list">top-level-list</a> )? ;

<i id="top-level">top-level</i> ::=
  | <a href="#declaration">declaration</a>
  | <a href="#expression">expression</a> ;

<i id="declaration">declaration</i> ::=
  | <a href="#function-declaration">function-declaration</a>
  | <a href="#module-declaration">module-declaration</a>
  | <a href="#type-declaration">type-declaration</a>
  | <a href="#using-declaration">using-declaration</a> ;

<i id="expression">expression</i> ::=
  | <a href="#equality">equality</a>
  | <a href="#if-expression">if-expression</a>
  | <a href="#let-expression">let-expression</a>
  | <a href="#match-expression">match-expression</a> ;

<i id="function-declaration">function-declaration</i> ::=
  | <b>def</b> <a href="#visibility-modifier">visibility-modifier</a>? <b>IDENTIFIER</b> <b>(</b> <a href="#parameter-list">parameter-list</a>? <b>)</b> <a href="#type-annotation">type-annotation</a>? <b>=</b> <a href="#expression-block">expression-block</a> ;

<i id="module-declaration">module-declaration</i> ::=
  | <b>module</b> <a href="#visibility-modifier">visibility-modifier</a>? <b>IDENTIFIER</b> <b>=</b> <a href="#module-declaration-block">module-declaration-block</a> ;

<i id="type-declaration">type-declaration</i> ::=
  | <b>type</b> <a href="#visibility-modifier">visibility-modifier</a>? <b>IDENTIFIER</b> <b>=</b> <a href="#type-declaration-block">type-declaration-block</a> ;

<i id="using-declaration">using-declaration</i> ::=
  | <b>using</b> <b>PATH</b> ( <a href="#using-declaration-alias">using-declaration-alias</a> | <a href="#using-declaration-member">using-declaration-member</a> )? ;

<i id="visibility-modifier">visibility-modifier</i> ::=
  | <b>public</b>
  | <b>internal</b> ;

<i id="parameter">parameter</i> ::=
  | <b>IDENTIFIER</b> <a href="#type-annotation">type-annotation</a>? ;

<i id="parameter-list">parameter-list</i> ::=
  | <a href="#parameter">parameter</a> ( <b>,</b> <a href="#parameter-list">parameter-list</a> )? <b>,</b>? ;

<i id="module-declaration-block">module-declaration-block</i> ::=
  | <b>BEGIN</b> <a href="#module-declaration-block-item">module-declaration-block-item</a> <b>END</b> ;

<i id="type-declaration-block">type-declaration-block</i> ::=
  | <a href="#enum-body">enum-body</a>
  | <a href="#record-body">record-body</a> ;

<i id="using-declaration-member">using-declaration-member</i> ::=
  | <b>.</b> <b>IDENTIFIER</b> <a href="#using-declaration-alias">using-declaration-alias</a>?
  | <b>.</b> <b>{</b> <a href="#using-declaration-member-list">using-declaration-member-list</a>? <b>}</b> ;

<i id="using-declaration-member-list">using-declaration-member-list</i> ::=
  | <b>IDENTIFIER</b> <a href="#using-declaration-alias">using-declaration-alias</a>? ( <b>,</b> <a href="#using-declaration-member-list">using-declaration-member-list</a>? )? <b>,</b>? ;

<i id="using-declaration-alias">using-declaration-alias</i> ::=
  | <b>as</b> <b>IDENTIFIER</b> ;

<i id="module-declaration-block-item">module-declaration-block-item</i> ::=
  | <a href="#function-declaration">function-declaration</a>
  | <a href="#module-declaration">module-declaration</a>
  | <a href="#type-declaration">type-declaration</a> ;

<i id="enum-body">enum-body</i> ::=
  | <b>|</b>? <b>IDENTIFIER</b> ( <b>|</b> <b>IDENTIFIER</b> )*
  | <b>BEGIN</b> <b>|</b>? <b>IDENTIFIER</b> ( <b>|</b> <b>IDENTIFIER</b> )* <b>END</b> ;

<i id="record-body">record-body</i> ::=
  | <b>{</b> <a href="#record-body-fields">record-body-fields</a> <b>}</b>
  | <b>BEGIN</b> <b>{</b> <a href="#record-body-fields">record-body-fields</a> <b>}</b> <b>END</b> ;

<i id="record-body-fields">record-body-fields</i> ::=
  | <b>IDENTIFIER</b> <a href="#type-annotation">type-annotation</a> ( <b>,</b> <b>IDENTIFIER</b> <a href="#type-annotation">type-annotation</a> )* <b>,</b>? ;

<i id="type">type</i> ::=
  | <a href="#array-type">array-type</a>
  | <a href="#function-type">function-type</a>
  | <a href="#tuple-type">tuple-type</a>
  | <b>IDENTIFIER</b> ;

<i id="array-type">array-type</i> ::=
  | <b>[</b> <a href="#type">type</a> <b>]</b> ;

<i id="function-type">function-type</i> ::=
  | <a href="#type">type</a> ( <b>-></b> <a href="#function-type">function-type</a> )+ ;

<i id="tuple-type">tuple-type</i> ::=
  | <b>(</b> <b>)</b>
  | <b>(</b> <a href="#tuple-type-list">tuple-type-list</a> <b>,</b>? <b>)</b>
  | <a href="#tuple-type-list">tuple-type-list</a> ;

<i id="tuple-type-list">tuple-type-list</i> ::=
  | <a href="#type">type</a> ( <b>,</b> <a href="#tuple-type-list">tuple-type-list</a> )? ;

<i id="type-annotation">type-annotation</i> ::=
  | <b>:</b> <a href="#type">type</a> ;

<i id="if-expression">if-expression</i> ::=
  | <b>if</b> <a href="#pattern">pattern</a> <b>then</b> <a href="#expression-block">expression-block</a> <a href="#else-clause">else-clause</a>? ;

<i id="else-clause">else-clause</i> ::=
  | <b>else</b> <a href="#expression-block">expression-block</a>
  | <b>else</b> <a href="#if-expression">if-expression</a> ;

<i id="let-expression">let-expression</i> ::=
  | <b>let</b> <a href="#pattern">pattern</a> <b>=</b> <a href="#expression-block">expression-block</a> ;

<i id="match-expression">match-expression</i> ::=
  | <b>match</b> <a href="#pattern">pattern</a> <b>with</b> <a href="#match-expression-clause">match-expression-clause</a> ;

<i id="match-expression-clause">match-expression-clause</i> ::=
  | <b>|</b>? <a href="#pattern">pattern</a> <b>-></b> <a href="#expression-block">expression-block</a> ( <b>|</b> <a href="#pattern">pattern</a> <b>-></b> <a href="#expression-block">expression-block</a> )* ;

<i id="expression-block">expression-block</i> ::=
  | <a href="#expression">expression</a>
  | <b>BEGIN</b> <a href="#expression-block-list">expression-block-list</a> <b>END</b> ;

<i id="expression-block-list">expression-block-list</i> ::=
  | <a href="#expression">expression</a> ( ( <b>;</b> | <b>NEWLINE</b> ) <a href="#expression-block-list">expression-block-list</a> )* ;

<i id="equality">equality</i> ::=
  | <a href="#comparison-expression">comparison-expression</a> ( ( <b>=</b> | <b>!=</b> ) <a href="#comparison-expression">comparison-expression</a> )* ;

<i id="comparison-expression">comparison-expression</i> ::=
  | <a href="#additive-expression">additive-expression</a> ( ( <b><</b> | <b><=</b> | <b>=></b> | <b>></b> ) <a href="#additive-expression">additive-expression</a> )* ;

<i id="additive-expression">additive-expression</i> ::=
  | <a href="#multiplicative-expression">multiplicative-expression</a> ( ( <b>+</b> | <b>-</b> ) <a href="#multiplicative-expression">multiplicative-expression</a> )* ;

<i id="multiplicative-expression">multiplicative-expression</i> ::=
  | <a href="#unary-expression">unary-expression</a> ( ( <b>*</b> | <b>/</b> ) <a href="#unary-expression">unary-expression</a> )* ;

<i id="unary-expression">unary-expression</i> ::=
  | ( <b>-</b> | <b>!</b> ) <a href="#unary-expression">unary-expression</a>
  | <a href="#primary">primary</a> ;

<i id="primary">primary</i> ::=
  | <b>IDENTIFIER</b>
  | <a href="#literal-boolean">literal-boolean</a>
  | <a href="#literal-character">literal-character</a>
  | <a href="#literal-number">literal-number</a>
  | <a href="#literal-string">literal-string</a>
  | <b>(</b> <a href="#expression">expression</a> <b>)</b> ;

<i id="literal-boolean">literal-boolean</i> ::=
  | <b>true</b>
  | <b>false</b> ;

<i id="literal-character">literal-character</i> ::=
  | <b>CHARACTER</b> ;

<i id="literal-number">literal-number</i> ::=
  | <b>FLOAT</b>
  | <b>INTEGER</b> ;

<i id="literal-string">literal-string</i> ::=
  | <a href="#interpolated-string-literal">interpolated-string-literal</a>
  | <a href="#raw-string-literal">raw-string-literal</a>
  | <a href="#static-string-literal">static-string-literal</a> ;

<i id="interpolated-string-literal">interpolated-string-literal</i> ::=
  | <b>f</b> <b>"</b> <a href="#quoted-text">quoted-text</a>* <b>"</b> ;

<i id="raw-string-literal">raw-string-literal</i> ::=
  | <b>r</b> <b>"</b> <a href="#quoted-text-item">quoted-text-item</a>* <b>"</b> ;

<i id="static-string-literal">static-string-literal</i> ::=
  | <b>"</b> <a href="#quoted-text">quoted-text</a>* <b>"</b> ;

<i id="quoted-text">quoted-text</i> ::=
  | <a href="#escaped-character">escaped-character</a>
  | <a href="#quoted-text-item">quoted-text-item</a> ;

<i id="quoted-text-item">quoted-text-item</i> ::=
  | <b>~</b>( <b>"</b> | <b>U+000A</b> | <b>U+000D</b> ) ;

<i id="escaped-character">escaped-character</i> ::=
  | <b>\</b> ( <b>\</b> | <b>0</b> | <b>n</b> | <b>t</b> | <b>'</b> | <b>"</b> )
  | <b>\</b> <b>x</b> <b>ASCII-HEXADECIMAL-DIGITS</b>
  | <b>\</b> <b>u</b> <b>{</b> <b>UNICODE-SCALAR-DIGITS</b> <b>}</b> ;
</pre>
