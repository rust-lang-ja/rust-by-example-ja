<!-- The `File` struct represents a file that has been opened (it wraps a file
descriptor), and gives read and/or write access to the underlying file.

Since many things can go wrong when doing file I/O, all the `File` methods
return the `io::Result<T>` type, which is an alias for `Result<T, io::Error>`. -->
`File`構造体は開かれたファイルを表し(実際にはファイルディスクリプタのラッパーです)、読み込み・書き込み権限のどちらか一方、あるいは両方を提供します。

<!-- This makes the failure of all I/O operations *explicit*. Thanks to this, the
programmer can see all the failure paths, and is encouraged to handle them in
a proactive manner. -->
これはI/Oに関するオペレーションの失敗をより明瞭にします。このおかげでプログラマは直面した失敗を全て見ることができ、より生産的な方法でそれらを扱うことが可能になります。
