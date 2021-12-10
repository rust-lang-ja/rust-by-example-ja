# match

<!--
Rust provides pattern matching via the `match` keyword, which can be used like
a C `switch`. The first matching arm is evaluated and all possible values must be
covered.
-->
Rustは`match`を用いて、C言語における`switch`のようなパターンマッチングを行うことができます。
マッチする最初のアームが評価され、取りうるすべての値はカバーされていなければなりません。

```rust,editable
fn main() {
    let number = 13;
    // TODO ^ Try different values for `number`

    println!("Tell me about {}", number);
    match number {
        // Match a single value
        // 単一の値とのマッチをチェック
        1 => println!("One!"),
        // Match several values
        // いくつかの値とのマッチをチェック
        2 | 3 | 5 | 7 | 11 => println!("This is a prime"),
        // TODO ^ Try adding 13 to the list of prime values
        // TODO ^ 素数のリストに 13 を加えてみましょう
        // Match an inclusive range
        // 特定の範囲の値とのマッチをチェック
        13..=19 => println!("A teen"),
        // Handle the rest of cases
        // その他の場合の処理
        _ => println!("Ain't special"),
        // TODO ^ Try commenting out this catch-all arm
        // TODO ^ この全てをキャッチするアームをコメントアウトしてみましょう
    }

    let boolean = true;
    // Match is an expression too
    // マッチは式でもある。
    let binary = match boolean {
        // The arms of a match must cover all the possible values
        // マッチは全ての可能な値をカバーしなくてはならない
        false => 0,
        true => 1,
        // TODO ^ Try commenting out one of these arms
        // TODO ^ 試しに片方をコメントアウトしてみましょう。
    };

    println!("{} -> {}", boolean, binary);
}
```
