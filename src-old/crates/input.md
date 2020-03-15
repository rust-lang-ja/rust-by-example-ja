<!-- A crate is a compilation unit in Rust. Whenever `rustc some_file.rs` is called,
`some_file.rs` is treated as the *crate file*. If `some_file.rs` has `mod`
declarations in it, then the contents of the module files will get merged with
the crate file *before* running the compiler over it. In other words, modules
do *not* get compiled individually, only crates get compiled. -->
クレイトはRustにおけるコンパイルの単位です。`rustc some_file.rs`が呼ばれると、`some_file.rs`は必ず*クレイトファイル*として扱われます。もし`some_file.rs`が`mod`宣言を含んでいるのならば、コンパイルの*前に*モジュールファイルの中身は、クレイトファイルと結合されます。言い換えると、それぞれのモジュールが独立にコンパイルされるということはありませんが、それぞれのクレートは互いに独立にコンパイルされるということです。

<!-- A crate can be compiled into a binary or into a library. By default, `rustc`
will produce a binary from a crate. This behavior can be overridden by passing
the `--crate-type` flag to `rustc`. -->
クレイトはバイナリあるいはライブラリ形式でコンパイルされることが可能です。デフォルトでは`rustc`はクレイトからバイナリを作り出しますが、この振る舞いは`--crate-type`フラグを`rustc`に渡すことでオーバーライドできます。
