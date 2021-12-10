<!--
# Guards
-->
# ガード

<!--
A `match` *guard* can be added to filter the arm.
-->
`match`内の条件文をフィルタリングするために、 *ガード(`guard`)* を使用することができます。

```rust,editable
fn main() {
    let pair = (2, -2);
    // TODO ^ Try different values for `pair`
    // TODO ^ `pair`の値を変更してみましょう。

    println!("Tell me about {:?}", pair);
    match pair {
        (x, y) if x == y => println!("These are twins"),
        // The ^ `if condition` part is a guard
        //     ^ `if`とそれに続く条件式がガードです。
        (x, y) if x + y == 0 => println!("Antimatter, kaboom!"),
        (x, _) if x % 2 == 1 => println!("The first one is odd"),
        _ => println!("No correlation..."),
    }
}
```

Note that the compiler does not check arbitrary expressions for whether all
possible conditions have been checked.  Therefore, you must use the `_` pattern
at the end.

```rust,editable
fn main() {
    let number: u8 = 4;

    match number {
        i if i == 0 => println!("Zero"),
        i if i > 0 => println!("Greater than zero"),
        _ => println!("Fell through"), // This should not be possible to reach
    }
}
```

<!--
### See also:
-->
### 参照

<!--
[Tuples](../../primitives/tuples.md)
-->
[タプル](../../primitives/tuples.md)
