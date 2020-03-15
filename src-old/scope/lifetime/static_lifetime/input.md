<!-- A `'static` lifetime is longest possible lifetime, and lasts for
the lifetime of the running program. A `'static` lifetime may also be
coerced to a shorter lifetimes. There are two ways to make a variable
with `'static` lifetime, and both are stored in the read-only memory
of the binary: -->
`'static`ライフタイムは全てのライフタイムの中で最長で、プログラムが動作している間、常に有効になります。`'static`であっても、より短いライフタイムに圧縮されることはあります。`'static`なライフタイムをもつ変数を作成する方法は2つあり、いずれも実行バイナリの一部としてROM上に保存されます。

<!-- * Make a constant with the `static` declaration.
* Make a `string` literal which has type: `&'static str`. -->
* `static`宣言とともに定数を作成する。
* 文字列リテラル で`&'static str`型を持つ変数を作成する。

<!-- See the following example for a display of each method: -->
では、それぞれの方法の例を見ていきましょう。

{static_lifetime.play}

### See also:

[`'static` 定数s][static_const]

[static_const]: /custom_types/constants.html
