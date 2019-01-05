<!-- The arguments of a macro are prefixed by a dollar sign `$` and type annotated
with a *designator*: -->
macroの引数は`$`が頭につきます。型は*識別子*(`designator`)でアノテーションされます。

``` rust,editable
macro_rules! create_function {
    // このマクロは`ident`識別子に対応する値を引数として取り
    // `$func_name`という名の関数を作成する。
    // `ident`識別子は関数・変数の名前用の識別子である。
    ($func_name:ident) => (
        fn $func_name() {
            // `stringify!`というマクロは`ident`を文字列に変える。
            println!("You called {:?}()",
                     stringify!($func_name))
        }
    )
}

// 上のマクロを利用して`foo`、`bar`という名の関数を作成する。
create_function!(foo);
create_function!(bar);

macro_rules! print_result {
    // このマクロは`expr`識別子に対応する値を引数として取り、
    // その結果を文字列としてプリントする。
    // `expr`識別子は式文に対応する。
    ($expression:expr) => (
        // `stringify!`は式文を*そのままの形で*文字列に変換する
        println!("{:?} = {:?}",
                 stringify!($expression),
                 $expression)
    )
}

fn main() {
    foo();
    bar();

    print_result!(1u32 + 1);

    // ブロックも式文の一種であることを思い出しましょう!
    print_result!({
        let x = 1u32;

        x * x + 2 * x - 1
    });
}

```

<!-- This is a list of all the designators: -->
以下が全識別子のリストです。

* `block`
* `expr` 式文に使用
* `ident` 関数、変数の名前に使用
* `item`
* `pat` (*pattern*)
* `path`
* `stmt` (*statement*)
* `tt` (*token tree*)
* `ty` (*type*)
