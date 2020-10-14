<!--
# Domain Specific Languages (DSLs)
-->
# Domain Specific Languages (ドメイン特化言語、DSLs)

<!--
A DSL is a mini "language" embedded in a Rust macro. It is completely valid
Rust because the macro system expands into normal Rust constructs, but it looks
like a small language. This allows you to define concise or intuitive syntax for
some special functionality (within bounds).
-->
DSLとはRustマクロに埋め込まれた小さな「言語」のことです。
マクロ機能は通常のRustのプログラムへと展開されるので、これは完全に正当なRustなのですが、まるで小さな言語であるかのように見えます。
これにより、（一定の条件のもとで）なんらかの特定の機能のための簡潔・直感的な構文を定義することができるようになります。

<!--
Suppose that I want to define a little calculator API. I would like to supply
an expression and have the output printed to console.
-->
ちょっとした計算機APIを定義したいとしましょう。
式を与えると、出力がコンソールに書き出されるようにしたいです。

```rust,editable
macro_rules! calculate {
    (eval $e:expr) => {{
        {
            let val: usize = $e; // Force types to be integers
                                 // 型を整数に制約
            println!("{} = {}", stringify!{$e}, val);
        }
    }};
}

fn main() {
    calculate! {
        eval 1 + 2 // hehehe `eval` is _not_ a Rust keyword!
                   // `eval`はRustのキーワード *じゃない* よね！
    }

    calculate! {
        eval (1 + 2) * (3 / 4)
    }
}
```

<!--
Output:
-->
出力はこうなります：

```txt
1 + 2 = 3
(1 + 2) * (3 / 4) = 0
```

<!--
This was a very simple example, but much more complex interfaces have been
developed, such as [`lazy_static`](https://crates.io/crates/lazy_static) or
[`clap`](https://crates.io/crates/clap).
-->
これはとても単純な例ですが、[`lazy_static`](https://crates.io/crates/lazy_static)や[`clap`](https://crates.io/crates/clap)のように、もっと複雑なインターフェースも開発されています。

<!--
Also, note the two pairs of braces in the macro. The outer ones are
part of the syntax of `macro_rules!`, in addition to `()` or `[]`.
-->
また、マクロの中に2組の括弧があることにも注目してください。
外側のは、`()`や`[]`に加え、`macro_rules!`の構文の一部です。
