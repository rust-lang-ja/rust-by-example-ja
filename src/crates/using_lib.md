<!--
# Using a Library
-->
# ライブラリの利用

<!--
To link a crate to this new library you may use `rustc`'s `--extern` flag. All 
of its items will then be imported under a module named the same as the library.
This module generally behaves the same way as any other module.
-->
クレートをこの新しいライブラリにリンクするには、`rustc`の`--extern`フラグを利用します。
クレートの要素を全てライブラリと同じ名前のモジュールにインポートします。
一般に、このモジュールは他のモジュールと同じように振る舞います。

```rust,ignore
// extern crate rary; // May be required for Rust 2015 edition or earlier
                      // Rust 2015以前で必要

fn main() {
    rary::public_function();

    // Error! `private_function` is private
    // エラー!`private_function`はプライベート
    //rary::private_function();

    rary::indirect_access();
}
```

```txt
# Where library.rlib is the path to the compiled library, assumed that it's
# in the same directory here:
# library.rlibがコンパイルされたライブラリのパスで、
# 同じディレクトリにあるものとする:
$ rustc executable.rs --extern rary=library.rlib && ./executable 
called rary's `public_function()`
called rary's `indirect_access()`, that
> called rary's `private_function()`
```
