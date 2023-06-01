<!--
# Scope and Shadowing
-->
# スコープとシャドーイング

<!--
Variable bindings have a scope, and are constrained to live in a *block*. A
block is a collection of statements enclosed by braces `{}`. 
-->
変数はスコープを持つため、 **ブロック** の中に閉じ込められています。ブロックとは`{}`で囲まれた領域のことです。

```rust,editable,ignore,mdbook-runnable
fn main() {
    // This binding lives in the main function
    // この変数はmain関数内が生息域です。
    let long_lived_binding = 1;

    // This is a block, and has a smaller scope than the main function
    // ここから下が`main`より小さいスコープを持つブロックとなります。
    {
        // This binding only exists in this block
        // この変数はこのブロック内のみに存在します。
        let short_lived_binding = 2;

        println!("inner short: {}", short_lived_binding);
    }
    // End of the block
    // ブロックの終わり

    // Error! `short_lived_binding` doesn't exist in this scope
    // `short_lived_binding`はこのスコープ内には存在しませんのでエラーとなります。
    println!("outer short: {}", short_lived_binding);
    // FIXME ^ Comment out this line
    // FIXME ^ コメントアウトしましょう

    println!("outer long: {}", long_lived_binding);
}
```

<!--
Also, [variable shadowing][variable-shadow] is allowed.
-->
同様に、[変数のシャドーイング][variable-shadow]も可能です。

```rust,editable,ignore,mdbook-runnable
fn main() {
    let shadowed_binding = 1;

    {
        println!("before being shadowed: {}", shadowed_binding);

        // This binding *shadows* the outer one
        // この変数はスコープ外の同名の変数を *シャドーイング* します
        let shadowed_binding = "abc";

        println!("shadowed in inner block: {}", shadowed_binding);
    }
    println!("outside inner block: {}", shadowed_binding);

    // This binding *shadows* the previous binding
    // この変数バインディングは以前に定義した変数を *シャドーイング* します
    let shadowed_binding = 2;
    println!("shadowed in outer block: {}", shadowed_binding);
}
```
[variable-shadow]: https://en.wikipedia.org/wiki/Variable_shadowing
