<!--
# structs
-->
# 構造体

<!--
Similarly, a `struct` can be destructured as shown:
-->
以下のようにして、構造体(`struct`)も同様にデストラクトすることができる。

```rust,editable
fn main() {
    struct Foo {
        x: (u32, u32),
        y: u32,
    }

    // Try changing the values in the struct to see what happens
    let foo = Foo { x: (1, 2), y: 3 };

    match foo {
        Foo { x: (1, b), y } => println!("First of x is 1, b = {},  y = {} ", b, y),

        // you can destructure structs and rename the variables,
        // the order is not important
        // 構造体をデストラクトして変数をリネーム
        // 順番は重要ではない。
        Foo { y: 2, x: i } => println!("y is 2, i = {:?}", i),

        // and you can also ignore some variables:
        // 一部の変数を無視することもできる。
        Foo { y, .. } => println!("y = {}, we don't care about x", y),
        // this will give an error: pattern does not mention field `x`
        // `x`に言及していないため、以下はエラーになる。
        //Foo { y } => println!("y = {}", y),
    }

    let faa = Foo { x: (1, 2), y: 3 };

    // You do not need a match block to destructure structs:
    let Foo { x : x0, y: y0 } = faa;
    println!("Outside: x0 = {x0:?}, y0 = {y0}");
}
```

<!--
### See also:
-->
### 参照

<!--
[Structs](../../../custom_types/structs.md)
-->
[構造体](../../../custom_types/structs.md)
