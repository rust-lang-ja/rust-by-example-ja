<!--
# Crates
-->
# クレート

<!--
A crate is a compilation unit in Rust. Whenever `rustc some_file.rs` is called,
`some_file.rs` is treated as the *crate file*. If `some_file.rs` has `mod`
declarations in it, then the contents of the module files would be inserted in
places where `mod` declarations in the crate file are found, *before* running
the compiler over it. In other words, modules do *not* get compiled
individually, only crates get compiled.
-->
クレートはRustにおけるコンパイルの単位です。`rustc some_file.rs`が呼ばれると、`some_file.rs`は必ず *クレートファイル* として扱われます。もし`some_file.rs`が`mod`宣言を含んでいるのならば、コンパイルの *前に* 、そのモジュールファイルの中身が`mod`の位置に挿入されます。言い換えると、それぞれのモジュールが独立にコンパイルされるということはありませんが、それぞれのクレートは互いに独立にコンパイルされるということです。

<!--
A crate can be compiled into a binary or into a library. By default, `rustc`
will produce a binary from a crate. This behavior can be overridden by passing
the `--crate-type` flag to `lib`.
-->
クレートはバイナリあるいはライブラリ形式でコンパイルされることが可能です。デフォルトでは`rustc`はクレートからバイナリを作り出しますが、この振る舞いは`--crate-type`フラグに`lib`を渡すことでオーバーライドできます。
