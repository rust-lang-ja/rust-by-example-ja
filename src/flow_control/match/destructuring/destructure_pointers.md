<!--
# pointers/ref
-->
# ポインタとref

<!--
For pointers, a distinction needs to be made between destructuring
and dereferencing as they are different concepts which are used
differently from a language like `C`.
-->
Rustのポインタは、`C`のポインタとは異なる概念なので、デストラクトとデリファレンスを同じようなやり方で扱うことはできない

<!--
 * Dereferencing uses `*`
 * Destructuring uses `&`, `ref`, and `ref mut`
-->
* デリファレンスには`*`を用いる。
* デストラクトには`&`, `ref`, `ref mut`を用いる。

```rust,editable
fn main() {
    // Assign a reference of type `i32`. The `&` signifies there
    // is a reference being assigned.
    // `i32`型へのリファレンスをアサインする。
    // `&`によってリファレンスであることを明示している。
    let reference = &4;

    match reference {
        // If `reference` is pattern matched against `&val`, it results
        // in a comparison like:
        // 上で定義した`reference`という変数が`&val`とのパターンマッチ
        // に用いられた場合、以下の2つの値が比較されていることになる。
        // `&i32`
        // `&val`
        // ^ We see that if the matching `&`s are dropped, then the `i32`
        // should be assigned to `val`.
        // ^ よって`&`を落とせば、`i32`が`val`にアサインされることがわかる。
        &val => println!("Got a value via destructuring: {:?}", val),
    }

    // To avoid the `&`, you dereference before matching.
    // `&`を使用したくない場合は、マッチングの前にデリファレンスする。
    match *reference {
        val => println!("Got a value via dereferencing: {:?}", val),
    }

    // What if you don't start with a reference? `reference` was a `&`
    // because the right side was already a reference. This is not
    // a reference because the right side is not one.
    // いきなりリファレンスを変数に代入するのではない場合はどうでしょう。
    // 先ほどは右辺値が`&`で始まっていたのでリファレンスでしたが、
    // これは違います。
    let _not_a_reference = 3;

    // Rust provides `ref` for exactly this purpose. It modifies the
    // assignment so that a reference is created for the element; this
    // reference is assigned.
    // このような場合、Rustでは変数束縛時に`ref`を宣言します。
    // 要素のリファレンスが作られて、それが束縛対象になります。
    let ref _is_a_reference = 3;

    // Accordingly, by defining 2 values without references, references
    // can be retrieved via `ref` and `ref mut`.
    // 同様にミュータブルな値の場合`ref mut`を使用することでリファレンスを
    // 取得できます。イミュータブルの場合と合わせてみていきましょう。
    let value = 5;
    let mut mut_value = 6;

    // Use `ref` keyword to create a reference.
    // `ref`を使用してリファレンスを作成。
    match value {
        ref r => println!("Got a reference to a value: {:?}", r),
    }

    // Use `ref mut` similarly.
    // 同様に`ref mut`を使用。
    match mut_value {
        ref mut m => {
            // Got a reference. Gotta dereference it before we can
            // add anything to it.
            // リファレンスを取得、値を変更するためにはデリファレンスする必要がある。
            *m += 10;
            println!("We added 10. `mut_value`: {:?}", m);
        },
    }
}
```

### See also:

[The ref pattern](../../../scope/borrow/ref.md)
