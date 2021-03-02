<!--
# tuples
-->
# タプル

<!--
Tuples can be destructured in a `match` as follows:
-->
以下のように、タプルは`match`を用いてデストラクトすることができます。

```rust,editable
fn main() {
    let pair = (0, -2);
    // TODO ^ Try different values for `pair`
    // TODO ^ `pair`に別の値を入れてみましょう。

    println!("Tell me about {:?}", pair);
    // Match can be used to destructure a tuple
    // `match`を用いてタプルをデストラクトしてみましょう。
    match pair {
        // Destructure the second
        // 2つ目の値をデストラクト
        (0, y) => println!("First is `0` and `y` is `{:?}`", y),
        (x, 0) => println!("`x` is `{:?}` and last is `0`", x),
        _      => println!("It doesn't matter what they are"),
        // `_` means don't bind the value to a variable
        // ここでは`_`は、値を変数に束縛しないことを意味します。
    }
}
```

<!--
### See also:
-->
### 参照

<!--
[Tuples](../../../primitives/tuples.md)
-->
[タプル](../../../primitives/tuples.md)
