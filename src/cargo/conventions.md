<!--
# Conventions
-->
# 慣例

<!--
In the previous chapter, we saw the following directory hierarchy:
-->
前の章ではこのようなディレクトリ階層がありました。

```txt
foo
├── Cargo.toml
└── src
    └── main.rs
```

<!--
Suppose that we wanted to have two binaries in the same project, though. What
then?
-->
しかし同じプロジェクトで2つのバイナリが欲しいとします。その場合は？

<!--
It turns out that `cargo` supports this. The default binary name is `main`, as
we saw before, but you can add additional binaries by placing them in a `bin/`
directory:
-->
`cargo`はこれもサポートしています。以前見た通りデフォルトのバイナリ名は`main`ですが、`bin/`ディレクトリに置くことで他のバイナリを追加できます。

```txt
foo
├── Cargo.toml
└── src
    ├── main.rs
    └── bin
        └── my_other_bin.rs
```

<!--
To tell `cargo` to compile or run this binary as opposed to the default or other
binaries, we just pass `cargo` the `--bin my_other_bin` flag, where `my_other_bin`
is the name of the binary we want to work with.
-->
デフォルトバイナリや他のバイナリではなく、このバイナリをコンパイルや実行するように`cargo`に伝えるには、`cargo`に`--bin my_other_bin`フラグを渡します。ここでは`my_other_bin`が対象のバイナリの名前です。

<!--
In addition to extra binaries, `cargo` supports [more features] such as
benchmarks, tests, and examples.
-->
バイナリの追加に加えて、`cargo`はベンチマークやテスト、サンプルなどの[その他の機能][more features]もサポートしています。

<!--
In the next chapter, we will look more closely at tests.
-->
次の章ではテストについてより詳しく見ていきます。

[more features]: https://doc.rust-lang.org/cargo/guide/project-layout.html
