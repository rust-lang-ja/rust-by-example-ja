<!--
# C-like
-->
# C言語ライクな列挙型

<!--
`enum` can also be used as C-like enums.
-->
列挙型はC言語の列挙型のような使い方をする事もできます。

```rust,editable
// An attribute to hide warnings for unused code.
// 使用されていないコードによる警告を抑えるアトリビュート
#![allow(dead_code)]

// enum with implicit discriminator (starts at 0)
// 値を明示しない場合、0から整数が順に入る。
enum Number {
    Zero,
    One,
    Two,
}

// enum with explicit discriminator
// 値を明示する場合
enum Color {
    Red = 0xff0000,
    Green = 0x00ff00,
    Blue = 0x0000ff,
}

fn main() {
    // `enums` can be cast as integers.
    // 列挙型の中身を整数としてキャストする。
    println!("zero is {}", Number::Zero as i32);
    println!("one is {}", Number::One as i32);

    println!("roses are #{:06x}", Color::Red as i32);
    println!("violets are #{:06x}", Color::Blue as i32);
}
```

### See also:

<!--
[casting][cast]
-->
[キャスティング][cast]

[cast]: ../../types/cast.md
