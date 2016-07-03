<!-- The `Path` struct represents file paths in the underlying filesystem. There are
two flavors of `Path`: `posix::Path`, for UNIX-like systems, and
`windows::Path`, for Windows. The prelude exports the appropriate
platform-specific `Path` variant. -->
構造体`Path`は、ファイルシステム中のパスを表します。`Path`には2つの変種があります。UNIXライクなファイルシステムのための`posix::Path`と、Windows用の`windows::Path`です。それぞれプラットフォームに対応した`Path`をエクスポートします。

<!-- A `Path` can be created from almost any type that implements the
`BytesContainer` trait, like a string, and provides several methods to get
information from the file/directory the path points to. -->
`BytesContainer`トレイトを実装している型ならばほぼ全て、そこから`Path`を作成できます。例えば文字列がそうです。そうすればpathが指すファイル・ディレクトリの情報を取得するためのメソッドがいくつか使えるようになります。

<!-- Note that a `Path` is *not* internally represented as an UTF-8 string, but
instead is stored as a vector of bytes (`Vec<u8>`). Therefore, converting a
`Path` to a `&str` is *not* free and may fail (an `Option` is returned). -->
`Path`の実態はUTF-8の文字列**ではなく**、バイト型のベクタ(`Vec<u8>`)であることに注意しましょう。したがって、`Path`を`&str`に変換するのは無条件**ではなく**、失敗する可能性があります。それゆえ`Option`型が返されます。

{path.play}

<!-- Be sure to check at other `Path` methods (`posix::Path` or `windows::Path`) and
the `FileStat` struct. -->
他の`Path`メソッド(`posix::Path`と`windows::Path`)をチェックするのを忘れずに！それと`FileStat`構造体も見ておくことをオススメします。
