<!-- The [`Drop`][Drop] trait only has one method: `drop`, which is called automatically
when an object goes out of scope. The main use of the `Drop` trait is to free the
resources that the implementor instance owns. -->
[`Drop`][Drop]トレイトにはメソッドが一つだけしかありません。`drop`です。これは、オブジェクトがスコープから抜けた時に自動で呼ばれます。`Drop`トレイトの主な使用目的は、インスタンスが所有する資源を開放することです。


<!-- `Box`, `Vec`, `String`, `File`, and `Process` are some examples of types that
implement the `Drop` trait to free resources. The `Drop` trait can also be
manually implemented for any custom data type. -->
`Drop`トレイトを実装している型の例としては`Box`、`Vec`、`String`、`File`、`Process`等があげられます。`Drop`トレイトは任意の型に対して手動で実装することができます。

<!-- The following example adds a print to console to the `drop` function to announce
when it is called. -->
以下の例では`drop`メソッドにコンソールへの出力を追加することで、`drop`が呼ばれたタイミングが分かるようにしています。

{drop.play}

[Drop]: https://doc.rust-lang.org/std/ops/trait.Drop.html
