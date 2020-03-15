<!--- Rust provides access to a wide variety of `primitives`. A sample includes: --->
Rustは様々な基本データ型(`primitives`)の使用をサポートしています。以下がその例です。

<!--- * signed integers: `i8`, `i16`, `i32`, `i64` and `isize` (pointer size) --->
* 符号付き整数: `i8`, `i16`, `i32`, `i64`, `isize`（ポインタのサイズ）
<!--- * unsigned integers: `u8`, `u16`, `u32`, `u64` and `usize` (pointer size) --->
* 符号無し整数: `u8`, `i16`, `u32`, `u64`, `usize`（ポインタのサイズ）
<!--- * floatoing point: `f32`, `f64` --->
* 浮動小数点: `f32`, `f64`
<!--- * `char` Unicode scalar values like `'a'`, `'α'` and `'∞'` (4 bytes each) --->
* `char`: `'a'`, `'α'`, `'∞'`などのUnicodeのスカラー
<!--- * `bool` either `true` or `false` --->
* `bool`: `true`または`false`
<!--- * and the unit type `()`, whose only value is also `()` --->
* unit型: `()`が唯一の値
<!--- * arrays like `[1, 2, 3]` --->
* 配列: e.g. `[1, 2, 3]`など
<!--- * tuples like `(1, true)` --->
* タプル: e.g. (1, true)

<!--- Variables can always be *type annotated*. Numbers may additionally be --->
<!--- annotated via a *suffix* or *by default*. Integers default to `i32` and --->
<!--- floats to `f64`. --->
変数は常に*型指定(`type annotate`)可能*です。数値型の場合はさらにサフィックスでの指定が可能です。指定しない場合デフォルトになります。例えば整数は`i32`が、浮動小数点は`f64`がデフォルトです。

{primitives.play}

### See also:

[`std` ライブラリ][std]

[std]: http://doc.rust-lang.org/std/
