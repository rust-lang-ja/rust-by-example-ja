<!--
# Primitives
-->
# 基本データ型

<!--
Rust provides access to a wide variety of `primitives`. A sample includes:
-->
Rustは様々な基本データ型(`primitives`)の使用をサポートしています。以下がその例です。


<!--
### Scalar Types
-->
### スカラー型

<!--
* signed integers: `i8`, `i16`, `i32`, `i64`, `i128` and `isize` (pointer size)
* unsigned integers: `u8`, `u16`, `u32`, `u64`, `u128` and `usize` (pointer
  size)
* floating point: `f32`, `f64`
* `char` Unicode scalar values like `'a'`, `'α'` and `'∞'` (4 bytes each)
* `bool` either `true` or `false`
* and the unit type `()`, whose only possible value is an empty tuple: `()`
-->
* 符号付き整数: `i8`, `i16`, `i32`, `i64`, `i128`, `isize`（ポインタのサイズ）
* 符号無し整数: `u8`, `u16`, `u32`, `u64`, `u128`, `usize`（ポインタのサイズ）
* 浮動小数点数: `f32`, `f64`
* `char`: `'a'`, `'α'`, `'∞'`などのUnicodeのスカラー値
* `bool`: `true`または`false`
* ユニット型: `()`が唯一の値

<!--
Despite the value of a unit type being a tuple, it is not considered a
compound type because it does not contain multiple values. 
-->
ユニット型はその値がタプルですが、複合型とはみなされません。内部に複数の値を含んでいるわけではないからです。

<!--
### Compound Types
-->
### 複合型

<!--
* arrays like `[1, 2, 3]`
* tuples like `(1, true)`
-->
* 配列: e.g. `[1, 2, 3]`など
* タプル: e.g. (1, true)

<!--
Variables can always be *type annotated*. Numbers may additionally be
annotated via a *suffix* or *by default*. Integers default to `i32` and
floats to `f64`. Note that Rust can also infer types from context.
-->
変数は常に *型指定(`type annotate`)可能* です。数値型の場合はさらにサフィックスでの指定が可能です。指定しない場合デフォルトになります。例えば整数は`i32`が、浮動小数点は`f64`がデフォルトです。また、Rustは文脈から型を推定することもできます。

```rust,editable,ignore,mdbook-runnable
fn main() {
    // Variables can be type annotated.
    // 変数に型を指定
    let logical: bool = true;

    let a_float: f64 = 1.0;  // Regular annotation
                             // 通常の型指定
    let an_integer   = 5i32; // Suffix annotation
                             // サフィックスによる型指定

    // Or a default will be used.
    // サフィックスを指定しない場合、デフォルトを選択
    let default_float   = 3.0; // `f64`
    let default_integer = 7;   // `i32`
    
    // A type can also be inferred from context 
    // 型を文脈から推定することも可能
    let mut inferred_type = 12; // Type i64 is inferred from another line
                                // 型 i64 は次行の内容に基づいて推定
    inferred_type = 4294967296i64;
    
    // A mutable variable's value can be changed.
    // ミュータブルな変数は値を変更できる
    let mut mutable = 12; // Mutable `i32`
                          // ミュータブルな `i32`.
    mutable = 21;
    
    // Error! The type of a variable can't be changed.
    // エラー！ ミュータブルな変数でも型は不変
    mutable = true;
    
    // Variables can be overwritten with shadowing.
    // 変数はシャドーイングによって上書きできる
    let mutable = true;
}
```

<!--
### See also:
-->
### 参照

<!--
[the `std` library][std], [`mut`][mut], [inference], and [shadowing]
-->
[`std` ライブラリ][std], [`mut`][mut], [inference], [shadowing]

[std]: https://doc.rust-lang.org/std/
[mut]: variable_bindings/mut.md
[inference]: types/inference.md
[shadowing]: variable_bindings/scope.md
