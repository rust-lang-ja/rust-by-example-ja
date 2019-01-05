<!--- We've seen that formatting is specified via a *format string*: --->
これまで、文字列がどのようにフォーマットされるかは*フォーマット文字列*によって決まるということを見てきました 。

* `format!("{}", foo)` -> `"3735928559"`
* `format!("0x{:X}", foo)` ->
  [`"0xDEADBEEF"`][deadbeef]
* `format!("0o{:o}", foo)` -> `"0o33653337357"`

<!--- The same variable (`foo`) can be formatted differently depending on which --->
<!--- *argument type* is used: `X` vs `o` vs *unspecified*. --->
ここでは （`foo`）という単一の変数が`X`、`o`、*指定なし*、という様々な*引数タイプ*(argument type)に応じてフォーマットされています。


<!--- This formatting functionality is implemented via traits, and there is one trait --->
<!--- for each argument type. The most common formatting trait is `Display`, which --->
<!--- handles cases where the argument type is left unspecified: `{}` for instance. --->
フォーマットの機能はそれぞれの引数タイプごとに個別のトレイトを用いて実装されています。
最も一般的なトレイトは`Display`で、これは引数タイプが未指定（たとえば`{}`）の時に呼び出されます。

``` rust,editable
use std::fmt::{self, Formatter, Display};

struct City {
    name: &'static str,
    // 緯度
    lat: f32,
    // 経度
    lon: f32,
}

impl Display for City {
    // `f` はバッファです。このメソッドは
    // ここにフォーマットされた文字列を書き込みます。
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let lat_c = if self.lat >= 0.0 { 'N' } else { 'S' };
        let lon_c = if self.lon >= 0.0 { 'E' } else { 'W' };

        // `write!`は`format!`に似ていますが、フォーマットされた文字列を
        // バッファ(第一引数)に書き込みます。
        write!(f, "{}: {:.3}°{} {:.3}°{}",
               self.name, self.lat.abs(), lat_c, self.lon.abs(), lon_c)
    }
}

#[derive(Debug)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

fn main() {
    for city in [
        City { name: "Dublin", lat: 53.347778, lon: -6.259722 },
        City { name: "Oslo", lat: 59.95, lon: 10.75 },
        City { name: "Vancouver", lat: 49.25, lon: -123.1 },
    ].iter() {
        println!("{}", *city);
    }
    for color in [
        Color { red: 128, green: 255, blue: 90 },
        Color { red: 0, green: 3, blue: 254 },
        Color { red: 0, green: 0, blue: 0 },
    ].iter() {
        // fmt::Displayに実装を追加したら、 {} を使用するように変更してください。
        println!("{:?}", *color)
    }
}

```

<!--- You can view a [full list of formatting traits][fmt_traits] and their argument --->
<!--- types in the [`std::fmt`][fmt] documentation. --->
フォーマット用トレイトの全リスト、及び引数の型は[こちら][fmt_traits]から、引数の型については[`std::fmt`のドキュメンテーション][fmt]から参照できます。

<!--- ### Activity --->
### 演習

<!--- Add an implementation of the `fmt::Display` trait for the `Color` struct above --->
<!--- so that the output displays as: --->
上にあるソースコード中の`Color`という構造体のための`fmt::Display`トレイトの実装を追加しましょう。アウトプットは以下のように表示されるはずです。

``` text
RGB (128, 255, 90) 0x80FF5A
RGB (0, 3, 254) 0x0003FE
RGB (0, 0, 0) 0x000000
```
<!--- Two hints if you get stuck: --->
詰まったら以下の2つがヒントになります。
<!---  * You [may need to list each color more than once][argument_types], --->
<!---  * You can [pad with zeros to a width of 2][fmt_width] with `:02`. --->
* [それぞれの色を2回以上記述する必要があるかもしれません。][argument_typs]
* `:02`で、[幅を2に指定し、空白を0で埋める事ができます。][fmt_width]


### See also
[`std::fmt`][fmt]

[argument_types]: http://doc.rust-lang.org/std/fmt/#argument-types
[deadbeef]: https://en.wikipedia.org/wiki/Deadbeef#Magic_debug_values
[fmt]: http://doc.rust-lang.org/std/fmt/
[fmt_traits]: http://doc.rust-lang.org/std/fmt/#formatting-traits
[fmt_width]: http://doc.rust-lang.org/std/fmt/#width
