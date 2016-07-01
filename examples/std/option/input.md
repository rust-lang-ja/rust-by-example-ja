<!-- Sometimes it's desirable to catch the failure of some parts of a program
instead of calling `panic!`; this can be accomplished using the `Option` enum. -->
プログラムの一部が失敗した際、`panic!`するよりも、エラーを補足する方が望ましい場合があります。これは`Option`という列挙型を用いることで可能になります。

<!-- The `Option<T>` enum has two variants: -->
列挙型`Option<T>`には2つの値があります。

<!-- * `None`, to indicate failure or lack of value, and
* `Some(value)`, a tuple struct that wraps a `value` with type `T`. -->
* `None`、これは実行の失敗か値の欠如を示します。
* `Some(value)`、型`T`の`value`をラップするタプルです。

{option.play}
