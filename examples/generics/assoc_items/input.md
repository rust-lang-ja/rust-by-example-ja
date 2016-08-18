<!-- "Associated Items" refers to a set of rules pertaining to [`item`][items]s
of various types. It is an extension to `trait` generics, and allows
`trait`s to internally define new items. -->
関連要素(Associated Items)とは複数の型の[要素(items)][items]に関係のある規則の総称です。トレイトの拡張機能であり、トレイトの中で新しい要素を定義することを可能にします。

<!-- One such item is called an *associated type*, providing simpler usage
patterns when the `trait` is generic over its container type. -->
そのように定義する要素の一つに**関連型**があります。これにより、ジェネリックなコンテナ型に対するトレイトを使用する際に、よりシンプルな書き方ができるようになります。

### See also:

[RFC][RFC]

[items]: http://doc.rust-lang.org/reference.html#items
[RFC]: https://github.com/rust-lang/rfcs/blob/master/text/0195-associated-items.md
