<!--
# Literals and operators
-->
# リテラルとオペレータ

<!--
Integers `1`, floats `1.2`, characters `'a'`, strings `"abc"`, booleans `true`
and the unit type `()` can be expressed using literals.
-->
整数`1`、浮動小数点`1.2`、文字(`char`)`'a'`、文字列`"abc"`、ブーリアン`true`、ユニット`()`は、リテラルを使用することで明示することが可能です。

<!--
Integers can, alternatively, be expressed using hexadecimal, octal or binary
notation using these prefixes respectively: `0x`, `0o` or `0b`.
-->
また整数型の場合、リテラルの代わりにプレフィックスに`0x`、`0o`、`0b`を指定することでそれぞれ16進数、8進数、2進数を使うことができます。

<!--
Underscores can be inserted in numeric literals to improve readability, e.g.
`1_000` is the same as `1000`, and `0.000_001` is the same as `0.000001`.
-->
可読性のため、`_`（アンダースコア）を数値リテラルの間に挿入することができます。例えば`1_000`は`1000`と、`0.000_001`は`0.000001`とそれぞれ同一です。

<!--
Rust also supports scientific [E-notation][enote], e.g. `1e6`, `7.6e-4`. The
associated type is `f64`.
-->
また、Rustは`1e6`や`7.6e-4`などの科学的な[E表記][enote]をサポートしています。
関連型は`f64`です。

<!--
We need to tell the compiler the type of the literals we use. For now,
we'll use the `u32` suffix to indicate that the literal is an unsigned 32-bit
integer, and the `i32` suffix to indicate that it's a signed 32-bit integer.
-->
コンパイラに、どのリテラルを使用するのかを教えてあげなくてはなりません。現在の仕様では、リテラルが32ビット符号無し整数であることを伝える場合、`u32`サフィックスを、符号付き32ビット整数であれば`i32`を使用します。

<!--
The operators available and their precedence [in Rust][rust op-prec] are similar
to other [C-like languages][op-prec].
-->
Rustで使用可能な演算子と、[その実行順序][rust op-prec]は、[Cなどの言語のもの][op-prec]とほぼ同じです。

```rust,editable
fn main() {
    // Integer addition
    // 整数の足し算
    println!("1 + 2 = {}", 1u32 + 2);

    // Integer subtraction
    // 整数の引き算
    println!("1 - 2 = {}", 1i32 - 2);
    // TODO ^ Try changing `1i32` to `1u32` to see why the type is important
    // TODO ^ 型が重要であることを実感するため`1i32`を`1u32`に変更してみましょう。

    // Scientific notation
    // 科学的表記
    println!("1e4 is {}, -2.5e-3 is {}", 1e4, -2.5e-3);

    // Short-circuiting boolean logic
    // 単純な論理演算子
    println!("true AND false is {}", true && false);
    println!("true OR false is {}", true || false);
    println!("NOT true is {}", !true);

    // Bitwise operations
    // ビットワイズ演算
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);

    // Use underscores to improve readability!
    // 可読性のための`_`（アンダースコア）の使用
    println!("One million is written as {}", 1_000_000u32);
}
```

[enote]: https://en.wikipedia.org/wiki/Scientific_notation#E_notation
[rust op-prec]: https://doc.rust-lang.org/reference/expressions.html#expression-precedence
[op-prec]: https://en.wikipedia.org/wiki/Operator_precedence#Programming_languages
