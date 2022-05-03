<!--
# Display
-->
# ディスプレイ

<!--
`fmt::Debug` hardly looks compact and clean, so it is often advantageous to
customize the output appearance. This is done by manually implementing
[`fmt::Display`][fmt], which uses the `{}` print marker. Implementing it
looks like this:
-->
`fmt::Debug`はコンパクトでクリーンであるようには見えませんね。大抵の場合は、アウトプットの見た目をカスタマイズしたほうが好ましいでしょう。これは`{}`を使用する[`fmt::Display`][fmt]を手動で実装することで可能です。

```rust
// Import (via `use`) the `fmt` module to make it available.
// （`use`を使用し、）`fmt`モジュールをインポートします。
use std::fmt;

// Define a structure for which `fmt::Display` will be implemented. This is
// a tuple struct named `Structure` that contains an `i32`.
// `fmt::Display`を実装するための構造体を定義します。
// これは`Structure`という名前に紐付けられた、`i32`を含むタプルです。
struct Structure(i32);

// To use the `{}` marker, the trait `fmt::Display` must be implemented
// manually for the type.
// `{}` というマーカーを使用するためには、
// この型専用の`fmt::Display`というトレイトが実装されていなくてはなりません。
impl fmt::Display for Structure {
    // This trait requires `fmt` with this exact signature.
    // このトレイトは`fmt`が想定通りのシグネチャであることを要求します。
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        // 厳密に最初の要素を、与えられた出力ストリーム `f` に書き込みます。
        // `fmt::Result`を返します。これはオペレーションが成功したか否か
        // を表します。
        // `write!`は`println!`に非常によく似た文法を使用していることに注目。
        write!(f, "{}", self.0)
    }
}
```

<!--
`fmt::Display` may be cleaner than `fmt::Debug` but this presents
a problem for the `std` library. How should ambiguous types be displayed?
For example, if the `std` library implemented a single style for all
`Vec<T>`, what style should it be? Would it be either of these two?
-->
`fmt::Display`は`fmt::Debug`より綺麗かもしれませんが、`std`ライブラリの場合は問題が生じます。曖昧な(ambiguous)タイプはどのように表示すれば良いでしょう？
例えば、`std`ライブラリがあらゆる`Vec<T>`に対して単一のスタイルを提供していた場合、どのようなスタイルに整形すればよいでしょう？以下の２つのどちらかを選ぶべきでしょうか？

<!--
* `Vec<path>`: `/:/etc:/home/username:/bin` (split on `:`)
* `Vec<number>`: `1,2,3` (split on `,`)
-->
* `Vec<path>`: `/:/etc:/home/username:/bin` （`:`で分割）
* `Vec<number>`: `1,2,3` （`,`で分割）

<!--
No, because there is no ideal style for all types and the `std` library
doesn't presume to dictate one. `fmt::Display` is not implemented for `Vec<T>`
or for any other generic containers. `fmt::Debug` must then be used for these
generic cases.
-->
答えはNOです。あらゆる型に対して理想的なスタイルなどというものはありませんし、`std`ライブラリによってそれが提供されているわけでもありません。`fmt::Display`は`Vec<T>`のようなジェネリックなコンテナ用に定義されているわけではありませんので、このような場合は`fmt::Debug`を使用するべきです。

<!--
This is not a problem though because for any new *container* type which is
*not* generic,`fmt::Display` can be implemented.
-->
ジェネリック *でない* コンテナ型の場合は、このような問題は生じませんので問題なく`fmt::Display`を実装することができます。

```rust,editable
use std::fmt; // Import `fmt`

// A structure holding two numbers. `Debug` will be derived so the results can
// be contrasted with `Display`.
// 2つの数字を扱うための構造体です。出力を`Display`と比較するため`Debug`
// をDeriveしています
#[derive(Debug)]
struct MinMax(i64, i64);

// Implement `Display` for `MinMax`.
// `MinMax`用の`Display`を実装しています。
impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Use `self.number` to refer to each positional data point.
        write!(f, "({}, {})", self.0, self.1)
    }
}

// Define a structure where the fields are nameable for comparison.
// 比較のため、フィールドに名前をつけれる様な構造体を定義しましょう
#[derive(Debug)]
struct Point2D {
    x: f64,
    y: f64,
}

// Similarly, implement `Display` for `Point2D`
// 先程と同様にして、Point2D用の`Display`を実装しています。
impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Customize so only `x` and `y` are denoted.
        // `x`と`y`のみが明示的になるようにカスタマイズ
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}

fn main() {
    let minmax = MinMax(0, 14);

    println!("Compare structures:");
    println!("Display: {}", minmax);
    println!("Debug: {:?}", minmax);

    let big_range =   MinMax(-300, 300);
    let small_range = MinMax(-3, 3);

    println!("The big range is {big} and the small is {small}",
             small = small_range,
             big = big_range);

    let point = Point2D { x: 3.3, y: 7.2 };

    println!("Compare points:");
    println!("Display: {}", point);
    println!("Debug: {:?}", point);

    // Error. Both `Debug` and `Display` were implemented, but `{:b}`
    // requires `fmt::Binary` to be implemented. This will not work.
    // `Debug`と`Display`は実装されていますが、`fmt::Binary`はされていないため
    // `{:b}`使用している以下の例はエラーになります、
    // println!("What does Point2D look like in binary: {:b}?", point);
}
```

<!--
So, `fmt::Display` has been implemented but `fmt::Binary` has not, and
therefore cannot be used. `std::fmt` has many such [`traits`][traits] and
each requires its own implementation. This is detailed further in
[`std::fmt`][fmt].
-->
`fmt::Display`は実装されていますが、`fmt::Binary`はされていないので使用できません。
`std::fmt`はそのような[トレイト][traits]が数多くあり、それぞれに独自の実装が必要です。詳しくは[`std::fmt`][fmt]を参照してください。

<!--
### Activity
-->
### 演習

<!--
After checking the output of the above example, use the `Point2D` struct as a
guide to add a `Complex` struct to the example. When printed in the same
way, the output should be:
-->
上記の例のアウトプットを確認し、`Point2D`構造体を参考として、複素数を格納するための`Complex`構造体を定義しましょう。うまく行けば以下のように出力されるはずです。

```txt
Display: 3.3 + 7.2i
Debug: Complex { real: 3.3, imag: 7.2 }
```

<!--
### See also:
-->
### 参照

<!--
[`derive`][derive], [`std::fmt`][fmt], [`macros`][macros], [`struct`][structs],
[`trait`][traits], and [`use`][use]
-->
[`derive`][derive], [`std::fmt`][fmt], [マクロ][macros], [`struct`][structs],
[`trait`][traits], [use][use]

[derive]: ../../trait/derive.md
[fmt]: https://doc.rust-lang.org/std/fmt/
[macros]: ../../macros.md
[structs]: ../../custom_types/structs.md
[traits]: https://doc.rust-lang.org/std/fmt/#formatting-traits
[use]: ../../mod/use.md
