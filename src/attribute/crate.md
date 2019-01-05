<!-- The `crate_type` attribute can be used to tell the compiler whether a crate is
a binary or a library (and even which type of library), and the `crate_name`
attribute can be used to set the name of the crate. -->
`crate_type`アトリビュートは、そのクレイトがライブラリ、バイナリのいずれにコンパイルされるべきかをコンパイラに伝えるために使用します。ライブラリの場合は、どのタイプのライブラリであるかも伝えることができます。`crate_name`はクレイトの名前を決定するのに使用します。

``` rust
// このクレイトはライブラリである。
#![crate_type = "lib"]
// このライブラリの名前は「rary」である。
#![crate_name = "rary"]

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

<!-- When the `crate_type` attribute is used, we no longer need to pass the
`--crate-type` flag to `rustc`. -->
`crate_type`アトリビュートが使用されているときは、`rustc`に`--crate-type`フラグを伝える必要はありません。

``` bash
$ rustc lib.rs
$ ls lib*
library.rlib
```
