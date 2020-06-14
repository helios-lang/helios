# Koi Grammar

<pre>
<i id="program">program</i> ::=
  | <a href="#top-level-list">top-level-list</a><sub>opt</sub> <b>EOF</b> ;

<i id="top-level-list">top-level-list</i> ::=
  | <a href="#top-level">top-level</a>
  | <a href="#top-level">top-level</a> <b>NEWLINE</b> <a href="#top-level-list">top-level-list</a> ;

<i id="top-level">top-level</i> ::=
  | <a href="#declaration">declaration</a>
  | <a href="#expression">expression</a> ;

<i id="declaration">declaration</i> ::=
  | <a href="#function-declaration">function-declaration</a>
  | <a href="#module-declaration">module-declaration</a>
  | <a href="#type-declaration">type-declaration</a>
  | <a href="#using-declaration">using-declaration</a> ;

<i id="expression">expression</i> ::=
  | <a href="#if-expression">if-expression</a>
  | <a href="#let-expression">let-expression</a>
  | <a href="#literal-expression">literal-expression</a>
  | <a href="#match-expression">match-expression</a> ;

<i id="function-declaration">function-declaration</i> ::=
  | <b>def</b> <a href="#visibility-modifier">visibility-modifier</a><sub>opt</sub> <b>IDENTIFIER</b> <b>(</b> <a href="#parameter-list">parameter-list</a><sub>opt</sub> <b>)</b> <a href="#type-annotation">type-annotation</a><sub>opt</sub> <b>=</b> <a href="#expression-block">expression-block</a> ;

<i id="module-declaration">module-declaration</i> ::=
  | <b>module</b> <a href="#visibility-modifier">visibility-modifier</a><sub>opt</sub> <b>IDENTIFIER</b> <b>=</b> <a href="#module-declaration-block">module-declaration-block</a> ;

<i id="type-declaration">type-declaration</i> ::=
  | <b>type</b> <a href="#visibility-modifier">visibility-modifier</a><sub>opt</sub> <b>IDENTIFIER</b> <b>=</b> <a href="#type-declaration-block">type-declaration-block</a> ;

<i id="using-declaration">using-declaration</i> ::=
  | <b>using</b> <b>PATH</b> <a href="#using-declaration-alias">using-declaration-alias</a><sub>opt</sub> ;

<i id="visibility-modifier">visibility-modifier</i> ::=
  | <b>public</b>
  | <b>internal</b>  ;

<i id="parameter">parameter</i> ::=
  | <b>IDENTIFIER</b> <a href="#type-annotation">type-annotation</a><sub>opt</sub> ;

<i id="parameter-list">parameter-list</i> ::=
  | <a href="#parameter">parameter</a>
  | <a href="#parameter">parameter</a> <b>,</b> <a href="#parameter-list">parameter-list</a> ;
</pre>
