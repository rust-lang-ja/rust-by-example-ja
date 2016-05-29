<!-- Let's create a library, and then see how to link it to another crate. -->
ではライブラリを作成し、それを別のクレイトにリンクする方法を見ていきましょう。

{rary.rs}

```
$ rustc --crate-type=lib rary.rs
$ ls lib*
library.rlib
```

<!-- Libraries get prefixed with "lib", and by default they get named after their
crate file, but this default name can be overridden using the [`crate_name`
attribute][crate-name]. -->
ライブラリは「lib」が頭につき、デフォルトでは、その後ろに元となったクレイトファイル名をつけます。(訳注: ここでは`lib` + `rary`)この振る舞いは[`crate_name`アトリビュート][crate-name]を用いてオーバーライドできます。

[crate-name]: ../attribute/crate.html
