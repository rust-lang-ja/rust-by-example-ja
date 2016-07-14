<!-- We determined a snake is an inappropriate gift for a princess. But what if
she expected a gift and didn't receive one? That would be just as bad, so
it needs to be handled! In the `std` library, an `enum` called `Option<T>`
is used when absence is a possibility. It manifests itself as one of
two "options": -->
先ほど、ヘビがお姫様への贈り物として適切でないことを確定させました。しかし、お姫様が贈り物を受け取ろうとした際に、何もなかったらどうなるのでしょう？その状況も同程度に望ましくないので、適切に対処する必要があります。値の欠如が可能性として考えられる場合、`std`ライブラリの`Option<T>`という列挙型が用いられます。これは、以下の2つの型を取りうるように定義されています。

<!-- * `Some(T)`: An element of type `T` was found
* `None`: No element was found -->
* `Some(T)`: 型`T`の値がある場合
* `None`: 値が存在しない場合。

<!-- These can either be explicitly handled via `match` or implicitly with
`unwrap`. Implicit handling either returns the inner element or `panic`s. -->
これらは`match`を用いて明示的に扱うこともできますし、`unwrap`で暗黙に処理することもできます。後者は`Some`の中の値を返すか`panic`するかのどちらかです。

<!-- Note that it's possible to manually customize `panic` with
[expect][expect], but `unwrap` otherwise leaves us with a less
meaningful output than explicit handling. In the following example,
explicit handling yields a more controlled result while retaining the
option to `panic` if desired. -->
[expect]メソッドを用いて、`panic`を手動でカスタマイズできることに触れておきましょう。これは(`unwrap`をそのまま用いた場合よりも)内容が理解しやすいエラーメッセージを出力するのに役立ちます。次の例では、結果をより明示的に、可能ならいつでも`panic`できるように扱っていきます。

{unwrap.play}

[expect]: http://doc.rust-lang.org/std/option/enum.Option.html#method.expect
