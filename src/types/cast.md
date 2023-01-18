<!--
# Casting
-->
# 型キャスティング

<!--
Rust provides no implicit type conversion (coercion) between primitive types.
But, explicit type conversion (casting) can be performed using the `as` keyword.
-->
Rustはプリミティブ型における強制的な型変換を暗黙に行うことはありません。しかし明示的に行うこと(`casting`)は可能です。その場合`as`キーワードを使用します。

<!--
Rules for converting between integral types follow C conventions generally,
except in cases where C has undefined behavior. The behavior of all casts
between integral types is well defined in Rust.
-->
整数型から整数型へ型変換する場合、C言語で可能なケースの場合はC言語と同じです。
C言語で未定義の場合の挙動も、Rustでは完全に定義されています。

```rust,editable,ignore,mdbook-runnable
// Suppress all warnings from casts which overflow.
// オーバーフローを起こすようなキャスティングによる警告を無視する。
#![allow(overflowing_literals)]

fn main() {
    let decimal = 65.4321_f32;

    // Error! No implicit conversion
    // エラー！ 暗黙的な型変換はできない。
    let integer: u8 = decimal;
    // FIXME ^ Comment out this line
    // FIXME ^ この行をコメントアウトしましょう。

    // Explicit conversion
    // 明示的な型変換
    let integer = decimal as u8;
    let character = integer as char;

    // Error! There are limitations in conversion rules.
    // A float cannot be directly converted to a char.
    let character = decimal as char;
    // FIXME ^ Comment out this line

    println!("Casting: {} -> {} -> {}", decimal, integer, character);

    // when casting any value to an unsigned type, T,
    // T::MAX + 1 is added or subtracted until the value
    // fits into the new type
    // 何らかの値を符号なしの型（仮にTとする）へキャスティングすると
    // 値がTに収まるまで、T::MAX + 1 が加算あるいは減算される。

    // 1000 already fits in a u16
    // 1000 はすでにu16に収まっているため変化しない。
    println!("1000 as a u16 is: {}", 1000 as u16);

    // 1000 - 256 - 256 - 256 = 232
    // Under the hood, the first 8 least significant bits (LSB) are kept,
    // while the rest towards the most significant bit (MSB) get truncated.
    // 水面下では最下位ビットから8bitが使用され、残りの上位ビットが圧縮される形になる。
    println!("1000 as a u8 is : {}", 1000 as u8);
    // -1 + 256 = 255
    println!("  -1 as a u8 is : {}", (-1i8) as u8);

    // For positive numbers, this is the same as the modulus
    println!("1000 mod 256 is : {}", 1000 % 256);

    // When casting to a signed type, the (bitwise) result is the same as
    // first casting to the corresponding unsigned type. If the most significant
    // bit of that value is 1, then the value is negative.
    // 符号付きの型にキャストする場合、結果は以下の2つを行った場合に等しい
    // 1. 対応する符号なしの型にキャストする。
    // 2. 2の補数(two's complement)をとる

    // Unless it already fits, of course.
    println!(" 128 as a i16 is: {}", 128 as i16);

    // 128 as u8 -> 128, whose value in 8-bit two's complement representation is:
    // 128をu8にキャストすると128となる。128の8ビットにおける補数は -128
    println!(" 128 as a i8 is : {}", 128 as i8);

    // repeating the example above
    // 上で示した例から
    // 1000 as u8 -> 232
    println!("1000 as a u8 is : {}", 1000 as u8);
    // and the value of 232 in 8-bit two's complement representation is -24
    // が成り立つ。232の8ビットにおける補数は -24
    println!(" 232 as a i8 is : {}", 232 as i8);

    // Since Rust 1.45, the `as` keyword performs a *saturating cast*
    // when casting from float to int. If the floating point value exceeds
    // the upper bound or is less than the lower bound, the returned value
    // will be equal to the bound crossed.

    // 300.0 as u8 is 255
    println!(" 300.0 as u8 is : {}", 300.0_f32 as u8);
    // -100.0 as u8 is 0
    println!("-100.0 as u8 is : {}", -100.0_f32 as u8);
    // nan as u8 is 0
    println!("   nan as u8 is : {}", f32::NAN as u8);

    // This behavior incurs a small runtime cost and can be avoided
    // with unsafe methods, however the results might overflow and
    // return **unsound values**. Use these methods wisely:
    unsafe {
        // 300.0 as u8 is 44
        println!(" 300.0 as u8 is : {}", 300.0_f32.to_int_unchecked::<u8>());
        // -100.0 as u8 is 156
        println!("-100.0 as u8 is : {}", (-100.0_f32).to_int_unchecked::<u8>());
        // nan as u8 is 0
        println!("   nan as u8 is : {}", f32::NAN.to_int_unchecked::<u8>());
    }
}
```
