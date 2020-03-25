<!--
# Expressions
-->
# 式文

<!--
A Rust program is (mostly) made up of a series of statements:
-->
Rustのプログラムは（ほとんどの場合）文(`statement`)の連続でできています

```
fn main() {
    // statement
    // statement
    // statement
}
```

<!--
There are a few kinds of statements in Rust. The most common two are declaring
a variable binding, and using a `;` with an expression:
-->
宣言文にはいくつかの種類があります。最も一般的なのは変数の束縛(`variable binding`)と式文(`expression`)で、いずれも行末に`;`が付きます

```
fn main() {
    // variable binding
    // 変数束縛
    let x = 5;

    // expression;
    // 式文
    x;
    x + 1;
    15;
}
```

<!--
Blocks are expressions too, so they can be used as values in
assignments. The last expression in the block will be assigned to the
place expression such as a local variable. However, if the last expression of the block ends with a
semicolon, the return value will be `()`.
-->
コードブロックも式の一種です。よってブロックを丸ごと値として扱うことができます。その場合ブロック内の最後の式が場所を表す式（例えばローカル変数）に代入されます。ただし、ブロック内の最後の式が`;`で終わる場合は返り値は`()`になります。

```rust,editable
fn main() {
    let x = 5u32;

    let y = {
        let x_squared = x * x;
        let x_cube = x_squared * x;

        // This expression will be assigned to `y`
        // この式文は`y`に代入されます。
        x_cube + x_squared + x
    };

    let z = {
        // The semicolon suppresses this expression and `()` is assigned to `z`
        // セミコロンがあるので`z`には`()`が入ります。
        2 * x;
    };

    println!("x is {:?}", x);
    println!("y is {:?}", y);
    println!("z is {:?}", z);
}
```
