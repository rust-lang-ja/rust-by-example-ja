<!-- The `crate_type` attribute can be used to tell the compiler whether a crate is
a binary or a library (and even which type of library), and the `crate_name`
attribute can be used to set the name of the crate. -->
`crate_type`アトリビュートは、そのクレイトがライブラリ、バイナリのいずれにコンパイルされるべきかをコンパイラに伝えるために使用します。ライブラリの場合は、どのタイプのライブラリであるかも伝えることができます。`crate_name`はクレイトの名前を決定するのに使用します。

{lib.rs}

<!-- When the `crate_type` attribute is used, we no longer need to pass the
`--crate-type` flag to `rustc`. -->
`crate_type`アトリビュートが使用されているときは、`rustc`に`--crate-type`フラグを伝える必要はありません。

```
$ rustc lib.rs
$ ls lib*
library.rlib
```
