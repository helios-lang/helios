# Koi Grammar

<pre>
<i id="program">program</i> ::=
  | <a href="#top-level-list">top-level-list</a>* <b>EOF</b> ;

<i id="top-level-list">top-level-list</i> ::=
  | <a href="#top-level">top-level</a> ( <b>NEWLINE</b> <a href="#top-level">top-level</a> )* ;

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
  | <b>using</b> <b>PATH</b> ( <a href="#using-declaration-alias">using-declaration-alias</a> | <a href="#using-declaration-import">using-declaration-import</a> )? ;

<i id="visibility-modifier">visibility-modifier</i> ::=
  | <b>public</b>
  | <b>internal</b> ;

<i id="parameter">parameter</i> ::=
  | <b>IDENTIFIER</b> <a href="#type-annotation">type-annotation</a>* ;

<i id="parameter-list">parameter-list</i> ::=
  | <a href="#parameter">parameter</a> ( <b>,</b> <a href="#parameter">parameter</a> )* <b>,</b>? ;

<i id="module-declaration-block">module-declaration-block</i> ::=
  | <b>INDENT</b> <a href="#module-declaration-block-item">module-declaration-block-item</a> <b>OUTDENT</b> ;

<i id="type-declaration-block">type-declaration-block</i> ::=
  | <a href="#enum-body">enum-body</a>
  | <a href="#record-body">record-body</a> ;

<i id="using-declaration-import">using-declaration-import</i> ::=
  | <b>.</b> <b>IDENTIFIER</b> <a href="#using-declaration-alias">using-declaration-alias</a>?
  | <b>.</b> <b>{</b> <a href="#using-declaration-import-list">using-declaration-import-list</a>? <b>}</b> ;

<i id="using-declaration-import-list">using-declaration-import-list</i> ::=
  | <b>IDENTIFIER</b> <a href="#using-declaration-alias">using-declaration-alias</a>? ( <b>,</b> <b>IDENTIFIER</b> <a href="#using-declaration-alias">using-declaration-alias</a>? )* <b>,</b>? ;

<i id="using-declaration-alias">using-declaration-alias</i> ::=
  | <b>as</b> <b>IDENTIFIER</b> ;

<i id="module-declaration-block-item">module-declaration-block-item</i> ::=
  | <a href="#function-declaration">function-declaration</a>
  | <a href="#module-declaration">module-declaration</a>
  | <a href="#type-declaration">type-declaration</a> ;

<i id="enum-body">enum-body</i> ::=
  | <b>|</b>? <b>IDENTIFIER</b> ( <b>|</b> <b>IDENTIFIER</b> )* ;

<i id="record-body">record-body</i> ::=
  | <b>{</b> <a href="#record-body-fields">record-body-fields</a> <b>}</b> ;

<i id="record-body-fields">record-body-fields</i> ::=
  | <b>IDENTIFIER</b> <a href="#type-annotation">type-annotation</a> ( <b>,</b> <b>IDENTIFIER</b> <a href="#type-annotation">type-annotation</a> ) <b>,</b>? ;

<i id="type">type</i> ::=
  | <a href="#array-type">array-type</a>
  | <a href="#function-type">function-type</a>
  | <a href="#tuple-type">tuple-type</a>
  | <b>IDENTIFIER</b> ;

<i id="array-type">array-type</i> ::=
  | <b>[</b> <a href="#type">type</a> <b>]</b> ;

<i id="function-type">function-type</i> ::=
  | <a href="#type">type</a> <b>-></b> <a href="#function-type-rest">function-type-rest</a> ;

<i id="function-type-rest">function-type-rest</i> ::=
  | <a href="#type">type</a>
  | <a href="#type">type</a> <b>-></b> <a href="#function-type-rest">function-type-rest</a> ;

<i id="tuple-type">tuple-type</i> ::=
  | <b>(</b> <b>)</b>
  | <a href="#tuple-type-list">tuple-type-list</a>
  | <b>(</b> <a href="#tuple-type-list">tuple-type-list</a> <b>,</b>? <b>)</b> ;

<i id="tuple-type-list">tuple-type-list</i> ::=
  | <a href="#type">type</a> ( <b>,</b> <a href="#type">type</a> )* ;

---

<i id="equality">equality</i> ::=
  | <a href="#comparison">comparison</a> ( ( <b>=</b> | <b>!=</b> ) <a href="#comparison">comparison</a> )* ;

<i id="comparison">comparison</i> ::=
  | <a href="#add-sub">add-sub</a> ( ( <b><</b> | <b><=</b> | <b>=></b> | <b>></b> ) <a href="#add-sub">add-sub</a> )* ;

<i id="add-sub">add-sub</i> ::=
  | <a href="#mul-div">mul-div</a> ( ( <b>+</b> | <b>-</b> ) <a href="#mul-div">mul-div</a> )* ;

<i id="mul-div">mul-div</i> ::=
  | <a href="#unary">unary</a> ( ( <b>*</b> | <b>/</b> ) <a href="#unary">unary</a> )* ;

<i id="unary">unary</i> ::=
  | ( <b>~</b> | <b>!</b> ) <a href="#unary">unary</a>
  | <a href="#primary">primary</a> ;

<i id="primary">primary</i> ::=
  | <a href="#literal">literal</a>
  | <a href="#parenthesised-expr">parenthesised-expr</a> ;

<i id="parenthesised-expr">parenthesised-expr</i> ::=
  | <b>(</b> <a href="#expression">expression</a> <b>)</b> ;

<i id="literal">literal</i> ::=
  | ... ;
</pre>
