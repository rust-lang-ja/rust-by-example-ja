<!--
# Guards
-->
# ガード

<!--
A `match` *guard* can be added to filter the arm.
-->
`match`内の条件文をフィルタリングするために、 *ガード(`guard`)* を使用することができます。

```rust,editable
#[allow(dead_code)]
enum Temperature {
    Celsius(i32),
    Fahrenheit(i32),
}

fn main() {
    let temperature = Temperature::Celsius(35);
    // ^ TODO try different values for `temperature`
    // ^ TODO `temperature`の値を変更してみましょう。

    match temperature {
        Temperature::Celsius(t) if t > 30 => println!("{}C is above 30 Celsius", t),
        // The `if condition` part ^ is a guard
        //                         ^ `if`とそれに続く条件式がガードです。
        Temperature::Celsius(t) => println!("{}C is below 30 Celsius", t),

        Temperature::Fahrenheit(t) if t > 86 => println!("{}F is above 86 Fahrenheit", t),
        Temperature::Fahrenheit(t) => println!("{}F is below 86 Fahrenheit", t),
    }
}
```

<!--
Note that the compiler won't take guard conditions into account when checking
if all patterns are covered by the match expression.
-->
コンパイラは、match式ですべてのパターンがカバーされているかどうかを調べるときに、
ガード条件を考慮しない点に注意してください。

```rust,editable,ignore,mdbook-runnable
fn main() {
    let number: u8 = 4;

    match number {
        i if i == 0 => println!("Zero"),
        i if i > 0 => println!("Greater than zero"),
        // _ => unreachable!("Should never happen."),
        // TODO ^ uncomment to fix compilation
        // TODO ^ アンコメントしてコンパイルを修正しよう
    }
}
```

<!--
### See also:
-->
### 参照

<!--
[Tuples](../../primitives/tuples.md)
[Enums](../../custom_types/enum.md)
-->
[タプル](../../primitives/tuples.md)
