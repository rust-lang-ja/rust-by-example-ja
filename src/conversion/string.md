<!--
# To and from Strings
-->
# Stringとの型変換

<!--
## Converting to String
-->
## Stringへの型変換

<!--
To convert any type to a `String` is as simple as implementing the [`ToString`]
trait for the type. Rather than doing so directly, you should implement the
[`fmt::Display`][Display] trait which automagically provides [`ToString`] and
also allows printing the type as discussed in the section on [`print!`][print].
-->
任意の型を`String`に変換するのは簡単で、その型に[`ToString`]トレイトを実装するだけです。これを直接実装するよりも、[`fmt::Display`][Display]トレイトを実装するのがよいでしょう。そうすることで自動的に[`ToString`]が提供されるだけでなく、[`print!`][print]の章で説明したように、その型を表示できるようにもなります。

```rust,editable
use std::fmt;

struct Circle {
    radius: i32
}

impl fmt::Display for Circle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Circle of radius {}", self.radius)
    }
}

fn main() {
    let circle = Circle { radius: 6 };
    println!("{}", circle.to_string());
}
```

<!--
## Parsing a String
-->
## Stringの解析

<!--
One of the more common types to convert a string into a number. The idiomatic
approach to this is to use the [`parse`] function and either to arrange for
type inference or to specify the type to parse using the 'turbofish' syntax.
Both alternatives are shown in the following example.
-->
文字列からの型変換において、数値への型変換はよく行われるものの一つです。これを行うイディオムは[`parse`]関数を使用することですが、このときに型を推論できるようにするか、もしくは turbofish構文を使用して型を指定するかのいずれかを行います。以下の例では、どちらの方法も紹介しています。

<!--
This will convert the string into the type specified as long as the [`FromStr`]
trait is implemented for that type. This is implemented for numerous types
within the standard library. To obtain this functionality on a user defined type
simply implement the [`FromStr`] trait for that type.
-->
`parse`関数は、指定された型に`FromStr`トレイトが実装されていれば、文字列をその型に変換します。このトレイトは標準ライブラリの多くの型に対して実装されています。ユーザー定義の型でこの機能を利用するには、その型に対して`FromStr`トレイトを実装するだけです。

```rust,editable
fn main() {
    let parsed: i32 = "5".parse().unwrap();
    let turbo_parsed = "10".parse::<i32>().unwrap();

    let sum = parsed + turbo_parsed;
    println!("Sum: {:?}", sum);
}
```

[`ToString`]: https://doc.rust-lang.org/std/string/trait.ToString.html
[Display]: https://doc.rust-lang.org/std/fmt/trait.Display.html
[print]: ../hello/print.md
[`parse`]: https://doc.rust-lang.org/std/primitive.str.html#method.parse
[`FromStr`]: https://doc.rust-lang.org/std/str/trait.FromStr.html
