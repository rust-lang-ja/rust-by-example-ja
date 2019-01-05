<!-- `map()` was described as a chainable way to simplify `match` statements.
However, using `map()` on a function that returns an `Option<T>` results
in the nested `Option<Option<T>>`. Chaining multiple calls together can
then become confusing. -->
先ほどは`map()`を、チェイン構文を用いて`match`文を単純化する物として説明しました。しかし`Option<T>`を返す関数に対しての`map()`の使用はネストした`Option<Option<T>>`を生じさせます。ですので、複数の関数呼び出しをチェインさせることは混乱を招く場合があります。

<!-- That's where `and_then()` comes in. Known in some languages as flatmap, `and_then()` calls its function input with the wrapped value or returns `None` if the `Option` is `None`. -->
そんな時こそ`and_then()`の出番です。他の言語ではflatmapと呼ばれることもあります。`and_then()`は引数として与えられた関数にラップされた値を渡しますが、その値が`None`だった場合は`None`を返します。

<!-- In the following example, `cookable_v2()` results in an `Option<Food>`.
Using `map()` instead of `and_then()` would have given an `Option<Option<Food>>`,
which is an invalid type for `eat()`. -->
以下の例では`cookable_v2()`は`Option<Food>`を返すため、`and_then()`ではなく`map()`を使用すると最終的に`Option<Option<Food>>`になります。これは`eat()`には不適切な型です。

{and_then.play}

### See also:

[`Option`][option], [`Option::map()`][map]、 [`Option::and_then()`][and_then]

[option]: http://doc.rust-lang.org/std/option/enum.Option.html
[map]: http://doc.rust-lang.org/std/option/enum.Option.html#method.map
[and_then]: http://doc.rust-lang.org/std/option/enum.Option.html#method.and_then
