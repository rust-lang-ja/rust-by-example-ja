# Path

<!--
The `Path` struct represents file paths in the underlying filesystem. There are
two flavors of `Path`: `posix::Path`, for UNIX-like systems, and
`windows::Path`, for Windows. The prelude exports the appropriate
platform-specific `Path` variant.
-->
構造体`Path`は、ファイルシステム中のパスを表します。`Path`には2つの変種があります。UNIXライクなファイルシステムのための`posix::Path`と、Windows用の`windows::Path`です。それぞれプラットフォームに対応した`Path`をエクスポートします。

<!--
A `Path` can be created from an `OsStr`, and provides several methods to get
information from the file/directory the path points to.
-->
`Path`は`OsStr`から作ることができます。そうすればそのパスが指すファイル・ディレクトリの情報を取得するためのメソッドがいくつか使えるようになります。

<!--
A `Path` is immutable. The owned version of `Path` is `PathBuf`. The relation 
between `Path` and `PathBuf` is similar to that of `str` and `String`: 
a `PathBuf` can be mutated in-place, and can be dereferenced to a `Path`.
-->
`Path`はイミュータブルです。`Path`の所有権ありのバージョンが`PathBuf`です。
`Path`と`PathBuf`の関係は、`str`と`String`の関係に似ています。
`PathBuf`はそのまま変更でき、`Path`にデリファレンスすることができます。

<!--
Note that a `Path` is *not* internally represented as an UTF-8 string, but
instead is stored as an `OsString`. Therefore, converting a `Path` to a `&str`
is *not* free and may fail (an `Option` is returned). However, a `Path` can be 
freely converted to an `OsString` or `&OsStr` using `into_os_string` and
`as_os_str`, respectively.
-->
`Path`の実態はUTF-8の文字列 **ではなく** 、`OsString`であることに注意しましょう。したがって、`Path`を`&str`に変換するのは無条件 **ではなく** 、失敗する可能性があります。それゆえ`Option`型が返されます。
しかし`Path`から`OsString`あるいは`&OsStr`への変換はそれぞれ`into_os_string`と`as_os_str`によって無条件でできます。

```rust,editable
use std::path::Path;

fn main() {
    // Create a `Path` from an `&'static str`
    // `&'static str`から`Path`を作成
    let path = Path::new(".");

    // The `display` method returns a `Display`able structure
    // `display`メソッドは`Display`可能な構造体を返す。
    let _display = path.display();

    // `join` merges a path with a byte container using the OS specific
    // separator, and returns a `PathBuf`
    // `join`はOS固有のセパレータによってバイトのコンテナ型であるパス
    // を結合し、`PathBuf`を返す。
    let mut new_path = path.join("a").join("b");

    // `push` extends the `PathBuf` with a `&Path`
    // `push`は`PathBuf`を`&Path`で拡張する。
    new_path.push("c");
    new_path.push("myfile.tar.gz");

    // `set_file_name` updates the file name of the `PathBuf`
    // `set_file_name`は`PathBuf`のファイル名を更新する。
    new_path.set_file_name("package.tgz");

    // Convert the `PathBuf` into a string slice
    // `PathBuf`を文字列のスライスに変換する。
    match new_path.to_str() {
        None => panic!("new path is not a valid UTF-8 sequence"),
        Some(s) => println!("new path is {}", s),
    }
}

```

<!--
Be sure to check at other `Path` methods (`posix::Path` or `windows::Path`) and
the `Metadata` struct.
-->
他の`Path`メソッド（`posix::Path`と`windows::Path`）をチェックするのを忘れずに！それと`Metadata`構造体も見ておくことをオススメします。

<!--
### See also:
-->
### 参照

<!--
[OsStr][1] and [Metadata][2].
-->
[OsStr][1] ならびに [Metadata][2]。

[1]: https://doc.rust-lang.org/std/ffi/struct.OsStr.html
[2]: https://doc.rust-lang.org/std/fs/struct.Metadata.html
