<!-- Rust provides no implicit type conversion (coercion) between primitive types.
But, explicit type conversion (casting) can be performed using the `as` keyword. -->
Rustはプリミティブ型における強制的な型変換を暗黙に行うことはありません。しかし明示的に行うこと（`casting`）は可能です。その場合`as`キーワードを使用します。

<!-- Rules for converting between integral types follow C conventions generally,
except in cases where C has undefined behavior. The behavior of all casts
between integral types is well defined in Rust. -->
整数型から整数型へ型変換する場合、C言語で可能なケースの場合はC言語と同じです。
C言語で未定義の場合の挙動も、Rustでは完全に定義されています。


``` rust,editable,ignore,mdbook-runnable
// オーバーフローを起こすようなキャスティングによる警告を無視する。
#![allow(overflowing_literals)]

fn main() {
    let decimal = 65.4321_f32;

    // エラー！ 暗黙的な型変換はできない。
    let integer: u8 = decimal;
    // FIXME ^ この行をコメントアウトしましょう。

    // 明示的な型変換
    let integer = decimal as u8;
    let character = integer as char;

    println!("Casting: {} -> {} -> {}", decimal, integer, character);

    // 何らかの値を符号なしの型（仮にTとする）へキャスティングすると
    // 値がTに収まるまで、std::T::MAX + 1 が加算あるいは減算される。

    // 1000 はすでにu16に収まっているため変化しない。
    println!("1000 as a u16 is: {}", 1000 as u16);

    // 1000 - 256 - 256 - 256 = 232
    // 水面下では最下位ビットから8bitが使用され、残りの上位ビットが圧縮される形になる。
    println!("1000 as a u8 is : {}", 1000 as u8);
    // -1 + 256 = 255
    println!("  -1 as a u8 is : {}", (-1i8) as u8);

    println!("1000 mod 256 is : {}", 1000 % 256);

    // 符号付きの型にキャストする場合、結果は以下の2つを行った場合に等しい
    // 1. 対応する符号なしの型にキャストする。
    // 2. 2の補数（two's complement）をとる

    // 128をu8にキャストすると128となる。128の8ビットにおける補数は -128
    println!(" 128 as a i8 is : {}", 128 as i8);

    // 上で示した例から
    // 1000 as u8 -> 232
    // が成り立つ。232の8ビットにおける補数は -24
    println!("1000 as a i8 is : {}", 1000 as i8);
    println!(" 232 as a i8 is : {}", 232 as i8);


}

```
