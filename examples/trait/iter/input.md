<!-- The `Iterator` trait is used to implement iterators over collections such as arrays. -->
`Iterator`トレイトは、例えば配列のような、要素の集合に対してイテレータを実装するためのトレイトです。

<!-- The trait requires only a method to be defined for the `next` element,
which may be manually defined in an `impl` block or automatically
defined (as in arrays and ranges). -->
このトレイトは`next`の要素に相当するものを決定するためのメソッドのみを要求します。このメソッドは`impl`ブロック内で手動で実装するか、あるいは(配列やrangeのように)自動で定義されます。

<!-- As a point of convenience for common situations, the `for` construct
turns some collections into iterators using the [`.into_iterator()`][intoiter] method. -->
サッとイテレータを使いたい時は、`for`文で集合からイテレータを作成することが良くあります。これは[`.into_iterator()`][intoiter]メソッドを呼び出しています。

<!-- Methods that can be accessed using the `Iterator` trait in addition
to those shown in the example below can be found [here][iter]. -->
`Iterator`トレイトからアクセスできるメソッドの一覧は[ここ][iter]にあります。以下の例ではその一部を使用しています。

{iter.play}

[intoiter]: https://doc.rust-lang.org/std/iter/trait.IntoIterator.html
[iter]: http://doc.rust-lang.org/core/iter/trait.Iterator.html
