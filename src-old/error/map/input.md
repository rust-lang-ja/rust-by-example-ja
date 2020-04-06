<!-- `match` is a valid method for handling `Option`s. However, you may eventually
find heavy usage tedious; this is the case especially with operations that
are only valid with an input. -->
`match`は`Option`は扱うのに適したメソッドです。しかし、大量にこれを使用しているとじきに億劫になってくるでしょう。引数の値が有効である(訳注: この場合は`None`でない)必要がある関数を扱う際には特にそうです。

<!-- For situations where a simplistic mapping of `Some -> Some` and
`None -> None` is needed, `Option` has a built in method called `map()`. -->
`Some -> Some`あるいは`None -> None`の単純な操作を適用する必要がある場合には、`Option`は`map()`というビルトインのメソッドを提供していますので、これを使用しましょう。

<!-- Multiple `map()` calls can be chained together for even more flexibility.
In the following example, `process()` easily replaces all functions previous
to it while staying compact. -->
`map()`のフレキシビリティは、複数の`map()`をチェインしなければならない場合にさらに際立ちます。以下の例では、`process()`が直前の関数全てを用いた場合と同じ機能を、よりコンパクトに果たしているのがわかります。

{map.play}

### See also:

[closures][closures]

[closures]: ../fn/closures.html
