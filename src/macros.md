# macro_rules!

<!--
Rust provides a powerful macro system that allows metaprogramming. As you've
seen in previous chapters, macros look like functions, except that their name
ends with a bang `!`, but instead of generating a function call, macros are
expanded into source code that gets compiled with the rest of the program.
-->
Rustはメタプログラミングを可能にする、パワフルなマクロシステムを備えています。これまで見てきたように、マクロは`!`で終わることを除けば関数のように見えます。関数と違うのは関数呼び出し(`function call`)を生成する代わりに、ソースコード中に展開され、周囲のプログラムとともにコンパイルされる点です。

<!--
However, unlike macros in C and other languages, Rust macros are expanded into
abstract syntax trees, rather than string preprocessing, so you don't get
unexpected precedence bugs.
-->
しかし、Cやその他の言語のマクロが文字列のプリプロセッシングをするのと異なり、Rustのマクロは抽象構文木へと展開されるので、予期せぬprecendece（演算子の優先順位）のバグに出くわすことがありません。

<!--
Macros are created using the `macro_rules!` macro.
-->
マクロを作成するには`macro_rules!`というマクロを使用します。

```rust,editable
// This is a simple macro named `say_hello`.
// `say_hello`という名のシンプルなマクロ
macro_rules! say_hello {
    // `()` indicates that the macro takes no argument.
    // `()`はマクロが引数をとらないことを示す。
    () => {
        // The macro will expand into the contents of this block.
        // マクロは（訳注: プリコンパイルの段階で）このブロック内の内容に展開されます。
        println!("Hello!");
    };
}

fn main() {
    // This call will expand into `println!("Hello");`
    // この呼び出しは`println!("Hello");`に置き換えられます。
    say_hello!()
}
```

<!--
So why are macros useful?
-->
ではどうしてマクロは便利なのでしょうか？

<!--
1. Don't repeat yourself. There are many cases where you may need similar
   functionality in multiple places but with different types. Often, writing a
   macro is a useful way to avoid repeating code. (More on this later)
-->
1. 同じことを繰り返し書いてはいけない (Don't repeat yourself) から。
   複数の場所で、別の型だけれど似たような機能が必要な時がよくあります。
   しばしば、マクロはコードを繰り返し書くのを避ける有用な手段なのです（あとで詳述）。

<!--
2. Domain-specific languages. Macros allow you to define special syntax for a
   specific purpose. (More on this later)
-->
2. ドメイン特化言語であるから。マクロを使うと、特定の目的のための特定の構文を定義することができます（あとで詳述）。

<!--
3. Variadic interfaces. Sometimes you want to define an interface that takes a
   variable number of arguments. An example is `println!` which could take any
   number of arguments, depending on the format string. (More on this later)
-->
3. 可変個引数によるインターフェース。
   取る引数の数が可変であるようなインターフェースを定義したくなることもあるでしょう。
   例えば、`println!`は、フォーマット文字列に依存した任意の数の引数を取ることができます（あとで詳述）！
