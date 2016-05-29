<!-- Modules can be mapped to a file/directory hierarchy. Let's break down the
[visibility example][visibility] in files: -->
モジュールはファイル・ディレクトリ間の階層構造と対応関係にあります。モジュールに[お互いがどのように見えているか][visibility]、以下の様なファイルを例に詳しく見ていきましょう。

```
$ tree .
.
|-- my
|   |-- inaccessible.rs
|   |-- mod.rs
|   `-- nested.rs
`-- split.rs
```

{split.rs}

{my/mod.rs}

{my/nested.rs}

{my/inaccessible.rs}

<!-- Let's check that things still work as before: -->
では、以前と同じように実行できるか確認しましょう。

```
$ rustc split.rs && ./split
called `my::function()`
called `function()`
called `my::indirect_access()`, that
> called `my::private_function()`
called `my::nested::function()`
```

[visibility]: ../mod/visibility.html
