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

``` rust,editable
// `'static`ライフタイムを持つ定数を作成
static NUM: i32 = 18;

// `NUM`への参照を返す。ライフタイムは`'static`から引数の
// ライフタイムへと圧縮されている。
fn coerce_static<'a>(_: &'a i32) -> &'a i32 {
    &NUM
}

fn main() {
    {
        // 文字列リテラルを用いて変数を作成し、プリントする
        let static_string = "I'm in read-only memory";
        println!("static_string: {}", static_string);

        // `static_string`がスコープから抜けると、参照は使用することが
        // できなくなるが、データはバイナリ中に残る。
    }

    {
        // `coerce_static`関数を呼び出すために、整数を作成
        let lifetime_num = 9;

        // `NUM`を`lifetime_num`のライフタイムへと圧縮
        let coerced_static = coerce_static(&lifetime_num);

        println!("coerced_static: {}", coerced_static);
    }

    println!("NUM: {} stays accessible!", NUM);
}

```

### See also:

[`'static` 定数s][static_const]

[static_const]: /custom_types/constants.html
