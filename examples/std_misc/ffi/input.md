<!-- Rust provides a Foreign Function Interface (FFI) to C libraries. Foreign
functions must be declared inside an `extern` block annotated with a `#[link]`
attribute containing the name of the foreign library. -->
RustはCのライブラリを呼び出すために他言語関数インターフェイス(Foreign Function Interface, FFI)を持っています。他言語の関数を使用する際には、そのライブラリ名を`#[link]`アトリビュートに渡し、更にそれでアノテーションされた`extern`ブロック内で宣言する必要があります。

{ffi.rs}

{ffi.out}

<!-- Since calling foreign functions is considered unsafe, it's common to write safe
wrappers around them. -->
他言語関数呼び出しは安全でない(unsafe)ので、安全にするためのラッパーを書くことが一般的です。

{safe.rs}

{safe.out}
