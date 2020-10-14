<!--
# Designators
-->
# 識別子

<!--
The arguments of a macro are prefixed by a dollar sign `$` and type annotated
with a *designator*:
-->
macroの引数は`$`が頭につきます。型は *識別子* (`designator`)でアノテーションされます。

```rust,editable
macro_rules! create_function {
    // This macro takes an argument of designator `ident` and
    // creates a function named `$func_name`.
    // The `ident` designator is used for variable/function names.
    // このマクロは`ident`識別子に対応する値を引数として取り
    // `$func_name`という名の関数を作成する。
    // `ident`識別子は関数・変数の名前用の識別子である。
    ($func_name:ident) => {
        fn $func_name() {
            // The `stringify!` macro converts an `ident` into a string.
            // `stringify!`というマクロは`ident`を文字列に変える。
            println!("You called {:?}()",
                     stringify!($func_name));
        }
    };
}

// Create functions named `foo` and `bar` with the above macro.
// 上のマクロを利用して`foo`、`bar`という名の関数を作成する。
create_function!(foo);
create_function!(bar);

macro_rules! print_result {
    // This macro takes an expression of type `expr` and prints
    // it as a string along with its result.
    // The `expr` designator is used for expressions.
    // このマクロは`expr`識別子に対応する値を引数として取り、
    // その結果を文字列としてプリントする。
    // `expr`識別子は式に対応する。
    ($expression:expr) => {
        // `stringify!` will convert the expression *as it is* into a string.
        // `stringify!`は式を *そのままの形で* 文字列に変換する
        println!("{:?} = {:?}",
                 stringify!($expression),
                 $expression);
    };
}

fn main() {
    foo();
    bar();

    print_result!(1u32 + 1);

    // Recall that blocks are expressions too!
    // ブロックも式の一種であることを思い出しましょう!
    print_result!({
        let x = 1u32;

        x * x + 2 * x - 1
    });
}
```

<!--
These are some of the available designators:
-->
使用できる識別子には以下のようなものがあります。

<!--
* `block`
* `expr` is used for expressions
* `ident` is used for variable/function names
* `item`
* `literal` is used for literal constants
* `pat` (*pattern*)
* `path`
* `stmt` (*statement*)
* `tt` (*token tree*)
* `ty` (*type*)
* `vis` (*visibility qualifier*)
-->
* `block`
* `expr` 式に使用
* `ident` 関数、変数の名前に使用
* `item`
* `literal` はリテラル定数（訳注：文字だけではない。[Literal expressions][literal_expressions]を参照）に使用
* `pat` (*パターン*)
* `path`
* `stmt` (*宣言*)
* `tt` (*トークンツリー*)
* `ty` (*型*)
* `vis` (*可視性修飾子*)（訳注：`pub (crate)`とか）

[literal_expressions]:https://doc.rust-lang.org/reference/expressions/literal-expr.html

<!--
For a complete list, see the [Rust Reference].
-->
完全なリストを見るには、[Rustリファレンス][Rust Reference]を読んでください。

[Rust Reference]: https://doc.rust-lang.org/reference/macros-by-example.html
