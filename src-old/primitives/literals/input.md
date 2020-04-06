<!--- Integers `1`, floats `1.2`, characters `'a'`, strings `"abc"`, booleans `true` --->
<!--- and the unit type `()` can be expressed using literals. --->
整数`1`、浮動小数点`1.2`、文字(`char`)`'a'`、文字列`"abc"`、ブーリアン`true`、ユニット`()`は、リテラルを使用することで明示することが可能です。

<!--- Integers can, alternatively, be expressed using hexadecimal, octal or binary --->
<!--- notation using either of these prefixes: `0x`, `0o` or `0b`. --->
また整数型の場合、リテラルの代わりにプレフィックスに`0x`、`0o`、`0b`を指定することでそれぞれ16進数、8進数、2進数を使うことができます。

<!--- Underscores can be inserted in numeric literals to improve readability, e.g. --->
<!--- `1_000` is the same as `1000`, and `0.000_001` is the same as `0.000001`. --->
可読性のため、`_`（アンダースコア）を数値リテラルの間に挿入することができます。例えば`1_000`は`1000`と、`0.000_001`は`0.000001`とそれぞれ同一です。

<!--- We need to tell the compiler the type of the literals we use. For now, --->
<!--- we'll use the `u32` suffix to indicate that the literal is an unsigned 32-bit --->
<!--- integer, and the `i32` suffix to indicate that it's a signed 32-bit integer. --->
コンパイラに、どのリテラルを使用するのかを教えてあげなくてはなりません。現在の仕様では、リテラルが32ビット符号無し整数であることを伝える場合、`u32`サフィックスを、符号付き32ビット整数であれば`i32`を使用します。

<!--- The operators available and their precedence [in Rust][rust op-prec] are similar to other --->
Rustで使用可能な演算子と、[その実行順序](http://doc.rust-lang.org/reference.html#operator-precedence)は、[Cなどの言語のもの][op-prec]とほぼ同じです。

{literals.play}

[rust op-prec]: http://doc.rust-lang.org/reference.html#operator-precedence
[op-prec]: https://en.wikipedia.org/wiki/Operator_precedence#Programming_languages
