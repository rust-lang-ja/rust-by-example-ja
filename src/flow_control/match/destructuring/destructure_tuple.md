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
    let triple = (0, -2, 3);
    // TODO ^ Try different values for `triple`
    // TODO ^ `triple`に別の値を入れてみましょう。

    println!("Tell me about {:?}", triple);
    // Match can be used to destructure a tuple
    // `match`を用いてタプルをデストラクトしてみましょう。
    match triple {
        // Destructure the second and third elements
        // 2つ目と3つ目の値をデストラクト
        (0, y, z) => println!("First is `0`, `y` is {:?}, and `z` is {:?}", y, z),
        (1, ..)  => println!("First is `1` and the rest doesn't matter"),
        // `..` can be used to ignore the rest of the tuple
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
