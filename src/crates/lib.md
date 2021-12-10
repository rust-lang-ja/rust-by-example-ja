<!--
# Creating a Library
-->
# ライブラリ

<!--
Let's create a library, and then see how to link it to another crate.
-->
ではライブラリを作成し、それを別のクレートにリンクする方法を見ていきましょう。

```rust,ignore
pub fn public_function() {
    println!("called rary's `public_function()`");
}

fn private_function() {
    println!("called rary's `private_function()`");
}

pub fn indirect_access() {
    print!("called rary's `indirect_access()`, that\n> ");

    private_function();
}
```

```shell
$ rustc --crate-type=lib rary.rs
$ ls lib*
library.rlib
```

<!--
Libraries get prefixed with "lib", and by default they get named after their
crate file, but this default name can be overridden by passing
the `--crate-name` option to `rustc` or by using the [`crate_name`
attribute][crate-name].
-->
ライブラリは「lib」が頭につき、デフォルトでは、その後ろに元となったクレートファイル名をつけます。（訳注: ここでは`lib` + `rary`）この振る舞いは[`crate_name`アトリビュート][crate-name]を用いてオーバーライドできます。

[crate-name]: ../attribute/crate.md
