<!-- There are two types of strings in Rust: `String` and `&str`. -->
Rustには文字列を扱う型が2つあります。`String`と`&str`です。

<!-- A `String` is stored as a vector of bytes (`Vec<u8>`), but guaranteed to
always be a valid UTF-8 sequence. `String` is heap allocated, growable and not
null terminated. -->
`String`は有効なUTF-8の配列であることを保証されたバイトのベクタ(`Vec<u8>`)として保持されます。ヒープ上に保持され、伸長可能で、末端にnull文字を含みません。

<!-- `&str` is a slice (`&[u8]`) that always points to a valid UTF-8 sequence, and
can be used to view into a `String`, just like `&[T]` is a view into `Vec<T>`. -->
`&str`は有効なUTF-8の配列のスライス(`&[u8]`)で、いつでも`String`に変換することができます。`&[T]`がいつでも`Vec<T>`に変換できるのと同様です。

{str.play}

<!-- More `str`/`String` methods can be found under the
[std::str][str] and
[std::string][string]
modules -->
`str`/`String`のメソッドをもっと見たい場合は[std::str][str]、[std::string][string]モジュールを参照してください。

[str]: http://doc.rust-lang.org/std/str/
[string]: http://doc.rust-lang.org/std/string/
