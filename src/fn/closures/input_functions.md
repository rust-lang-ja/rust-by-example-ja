<!--
# Input functions
-->
# 関数を受け取る関数

<!--
Since closures may be used as arguments, you might wonder if the same can be said
about functions. And indeed they can! If you declare a function that takes a
closure as parameter, then any function that satisfies the trait bound of that
closure can be passed as a parameter.
-->
これまで、クロージャを引数として渡せることを見てきました。すると次の疑問が浮かんできます

「クロージャではない普通の関数を引数として渡すことは可能なのだろうか?」

可能です!もしパラメータとしてクロージャを取る関数を定義すれば、そのクロージャのトレイト境界を満たす任意の関数をパラメータとして渡すことができます。

```rust,editable
// Define a function which takes a generic `F` argument
// bounded by `Fn`, and calls it
// 関数を引数として取り、即座に実行する関数を定義
fn call_me<F: Fn()>(f: F) {
    f();
}

// Define a wrapper function satisfying the `Fn` bound
// `Fn`境界を満たすラッパ関数を定義
fn function() {
    println!("I'm a function!");
}

fn main() {
    // Define a closure satisfying the `Fn` bound
    // `Fn`境界を満たすクロージャを定義
    let closure = || println!("I'm a closure!");

    call_me(closure);
    call_me(function);
}
```

<!--
As an additional note, the `Fn`, `FnMut`, and `FnOnce` `traits` dictate how
a closure captures variables from the enclosing scope.
-->
クロージャによる変数の補足がどのように行われているかを詳しく見たいときは`Fn`、`FnMut`、`FnOnce`を参照してください。

<!--
### See also:
-->
### 参照

[`Fn`][fn], [`FnMut`][fn_mut], and [`FnOnce`][fn_once]

[fn]: https://doc.rust-lang.org/std/ops/trait.Fn.html
[fn_mut]: https://doc.rust-lang.org/std/ops/trait.FnMut.html
[fn_once]: https://doc.rust-lang.org/std/ops/trait.FnOnce.html
