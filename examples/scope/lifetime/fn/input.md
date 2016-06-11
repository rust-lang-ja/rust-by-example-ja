<!-- Ignoring [elision], function signatures with lifetimes have a few constraints:  -->
[省略][elision]をしない場合、ライフタイムのシグネチャ(e.g. `<'a>`)を持つ関数にはいくつかの制限があります。

<!-- * any reference *must* have an annotated lifetime.
* any reference being returned *must* have the same lifetime as an input or
be `static`. -->
* 全ての変数においてライフタイムを明示しなくてはならない。
* 返り値となる参照はすべて引数と同じライフタイムか、`static`ライフタイムを持たなくてはならない

<!-- Additionally, note that returning references without input is banned if it
would result in returning references to invalid data. The following example shows
off some valid forms of functions with lifetimes: -->
加えて、引数のない関数から参照を返す場合、それが結果的に無効なデータへの参照になるならば、禁止されている

{fn.play}

### See also:

[functions][fn]

[elision]: /scope/lifetime/elision.html
[fn]: /fn.html
