<!--
# Debug
-->
# デバッグ

<!--
All types which want to use `std::fmt` formatting `traits` require an
implementation to be printable. Automatic implementations are only provided
for types such as in the `std` library. All others *must* be manually
implemented somehow.
-->
`std::fmt`のフォーマット用`トレイト`を使用したい型は、プリント可能である用に実装されている必要があります。`std`ライブラリの型のように自動でプリント可能なものもありますが、他はすべて *手動で実装する必要があります。*

<!--
The `fmt::Debug` `trait` makes this very straightforward. *All* types can
`derive` (automatically create) the `fmt::Debug` implementation. This is
not true for `fmt::Display` which must be manually implemented.
-->
`fmt::Debug`という`トレイト`はこれを簡略化します。 *すべての* 型は`fmt::Debug`の実装を`derive`、（すなわち自動で作成）することができるためです。
`fmt::Display`の場合はやはり手動で実装しなくてはなりません。

```rust
// This structure cannot be printed either with `fmt::Display` or
// with `fmt::Debug`.
// この構造体は`fmt::Display`、`fmt::Debug`のいずれによっても
// プリントすることができません。
struct UnPrintable(i32);

// The `derive` attribute automatically creates the implementation
// required to make this `struct` printable with `fmt::Debug`.
// `derive`アトリビュートは、
// この構造体を`fmt::Debug`でプリントするための実装を自動で提供します。
#[derive(Debug)]
struct DebugPrintable(i32);
```

<!--
All `std` library types automatically are printable with `{:?}` too:
-->
`std`ライブラリの型の場合は、自動的に`{:?}`によりプリント可能になっています。

```rust,editable
// Derive the `fmt::Debug` implementation for `Structure`. `Structure`
// is a structure which contains a single `i32`.
// `Structure`という構造体のための`fmt::Debug`をderiveしています。
// `Structure`は単一の`i32`をメンバに持っています。
#[derive(Debug)]
struct Structure(i32);

// Put a `Structure` inside of the structure `Deep`. Make it printable
// also.
// `Deep`という構造体の中に`Structure`を入れます。
// また、これをプリント可能にしています。
#[derive(Debug)]
struct Deep(Structure);

fn main() {
    // Printing with `{:?}` is similar to with `{}`.
    // `{:?}`によるプリントは `{}`に似ています。
    println!("{:?} months in a year.", 12);
    println!("{1:?} {0:?} is the {actor:?} name.",
             "Slater",
             "Christian",
             actor="actor's");

    // `Structure` is printable!
    // `Structure`はプリント可能です！
    println!("Now {:?} will print!", Structure(3));
    
    // The problem with `derive` is there is no control over how
    // the results look. What if I want this to just show a `7`?
    // `derive`を用いることの問題は、結果がどのように見えるか
    // コントロールする方法がないことです。
    // 出力を`7`だけにするためにはどうしたらよいでしょう？
    println!("Now {:?} will print!", Deep(Structure(7)));
}
```

<!--
So `fmt::Debug` definitely makes this printable but sacrifices some
elegance. Rust also provides "pretty printing" with `{:#?}`.
-->
`fmt::Debug`は確実にプリント可能にしてくれるのですが、一方である種の美しさを犠牲にしています。
Rustは`{:#?}`による「見栄えの良いプリント」も提供します。

```rust,editable
#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8
}

fn main() {
    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };

    // Pretty print
    println!("{:#?}", peter);
}
```

<!--
One can manually implement `fmt::Display` to control the display.
-->
手動で`fmt::Display`を実装することでプリント結果を思い通りにできます。

<!--
### See also:
-->
### 参照

<!--
[attributes][attributes], [`derive`][derive], [`std::fmt`][fmt],
and [`struct`][structs]
-->
[アトリビュート][attributes], [`derive`][derive], [`std::fmt`][fmt],
[構造体][structs]

[attributes]: https://doc.rust-lang.org/reference/attributes.html
[derive]: ../../trait/derive.md
[fmt]: https://doc.rust-lang.org/std/fmt/
[structs]: ../../custom_types/structs.md

