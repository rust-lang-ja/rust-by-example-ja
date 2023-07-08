<!--
# arrays/slices
-->
# 配列とスライス

<!--
Like tuples, arrays and slices can be destructured this way:
-->
タプル同様、配列とスライスも以下のようにデストラクトできます：

```rust,editable
fn main() {
    // Try changing the values in the array, or make it a slice!
    // 配列中の値を変更してみましょう。または、スライスにしてみましょう。
    let array = [1, -2, 6];

    match array {
        // Binds the second and the third elements to the respective variables
        // 2番目と3番目の要素を変数にバインドする。
        [0, second, third] =>
            println!("array[0] = 0, array[1] = {}, array[2] = {}", second, third),

        // Single values can be ignored with _
        // _で値を無視できる。
        [1, _, third] => println!(
            "array[0] = 1, array[2] = {} and array[1] was ignored",
            third
        ),

        // You can also bind some and ignore the rest
        // いくつかの値をバインドして残りを無視できる。
        [-1, second, ..] => println!(
            "array[0] = -1, array[1] = {} and all the other ones were ignored",
            second
        ),
        // The code below would not compile
        // 以下のコードはコンパイルできない。
        // [-1, second] => ...

        // Or store them in another array/slice (the type depends on
        // that of the value that is being matched against)
        // 別の配列やスライスに値を持たせることもできます。
        // (配列かスライスかは、マッチする値の型により異なります)
        [3, second, tail @ ..] => println!(
            "array[0] = 3, array[1] = {} and the other elements were {:?}",
            second, tail
        ),

        // Combining these patterns, we can, for example, bind the first and
        // last values, and store the rest of them in a single array
        // 例えば、これらのパターンを組み合わせて、
        // 最初と最後の値をバインドし、残りの値を配列に持たせることもできます。
        [first, middle @ .., last] => println!(
            "array[0] = {}, middle = {:?}, array[2] = {}",
            first, middle, last
        ),
    }
}
```

<!--
### See also:
-->
### 参照

<!--
[Arrays and Slices](../../../primitives/array.md) and [Binding](../binding.md) for `@` sigil
-->
[配列とスライス](../../../primitives/array.md)、@マークについては[バインディング](../binding.md)
