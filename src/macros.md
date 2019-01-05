<!-- Rust provides a powerful macro system that allows metaprogramming. As you've
seen in previous chapters, macros look like functions, except that their name
ends with a bang `!`, but instead of generating a function call, macros are
expanded into source code that gets compiled with the rest of the program. -->
Rustはメタプログラミングを可能にする、パワフルなマクロシステムを備えています。これまで見てきたように、マクロは`!`で終わることを除けば関数のように見えます。関数と違うのは関数呼び出し(`function call`)を生成する代わりに、ソースコード中に展開され、周囲のプログラムとともにコンパイルされる点です。

<!-- Macros are created using the `macro_rules!` macro. -->
マクロを作成するには`macro_rules!`というマクロを使用します。

``` rust,editable
// `say_hello`という名のシンプルなマクロ
macro_rules! say_hello {
    // `()`はマクロが引数をとらないことを示す。
    () => (
        // マクロは(訳注: プリコンパイルの段階で)このブロック内の内容に展開されます。
        println!("Hello!");
    )
}

fn main() {
    // この呼び出しは`println!("Hello");`に置き換えられます。
    say_hello!()
}

```
