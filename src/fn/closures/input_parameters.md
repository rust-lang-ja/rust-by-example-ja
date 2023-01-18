<!--
# As input parameters
-->
# 捕捉時の型推論

<!--
While Rust chooses how to capture variables on the fly mostly without type
annotation, this ambiguity is not allowed when writing functions. When
taking a closure as an input parameter, the closure's complete type must be
annotated using one of a few `traits`, and they're determined by what the
closure does with captured value. In order of decreasing restriction,
they are:
-->

Rustはたいていの場合、型アノテーションなしでも変数を捕捉する方法を臨機応変に選択してくれますが、関数を書く場合にはこの曖昧さは許されません。
引数のパラメータとしてクロージャを取る場合、そのクロージャの完全な型はいくつかの`traits`の中の1つを使って明示されなければなりません。
制限の少ない順に並べると、下記の通りです。

<!--
* `Fn`: the closure uses the captured value by reference (`&T`)
* `FnMut`: the closure uses the captured value by mutable reference (`&mut T`)
* `FnOnce`: the closure uses the captured value by value (`T`)
-->

* `Fn`: 参照(`&T`)によって捕捉するクロージャ
* `FnMut`: ミュータブルな参照(`&mut T`)によって捕捉するクロージャ
* `FnOnce`: 値(`T`)によって捕捉するクロージャ

<!--
On a variable-by-variable basis, the compiler will capture variables in the
least restrictive manner possible.
-->

変数ごとに、コンパイラは可能な限り制約の少ない方法でその変数を捕捉します。

<!--
For instance, consider a parameter annotated as `FnOnce`. This specifies
that the closure *may* capture by `&T`, `&mut T`, or `T`, but the compiler
will ultimately choose based on how the captured variables are used in the
closure.
-->

例えば、`FnOnce`というアノテーションの付けられたパラメータを考えてみましょう。
これはそのクロージャが`&T`、`&mut T`もしくは`T`の *どれか* で捕捉することを指定するものですが、コンパイラは捕捉した変数がそのクロージャの中でどのように使用されるかに基づき、最終的に捕捉する方法を選択することになります。

<!--
This is because if a move is possible, then any type of borrow should also
be possible. Note that the reverse is not true. If the parameter is
annotated as `Fn`, then capturing variables by `&mut T` or `T` are not
allowed. However, `&T` is allowed.
-->

これは、もし移動が可能であれば、いずれの種類の借用であっても同様に可能だからです。
その逆は正しくないことに注意してください。パラメータが`Fn`としてアノテーションされている場合、変数を`&mut T`や`T`で捕捉することは許可されません。
しかし`&T`は許可されます。

<!--
In the following example, try swapping the usage of `Fn`, `FnMut`, and
`FnOnce` to see what happens:
-->

以下の例では、`Fn`、`FnMut`、および`FnOnce`を入れ替えて、何が起こるのかを見てみましょう。

```rust,editable
// A function which takes a closure as an argument and calls it.
// <F> denotes that F is a "Generic type parameter"
fn apply<F>(f: F) where
    // The closure takes no input and returns nothing.
    // クロージャには引数も返り値もない。
    F: FnOnce() {
    // ^ TODO: Try changing this to `Fn` or `FnMut`.
    // ^ TODO: ここを`Fn`あるいは`FnMut`に変えてみましょう。

    f();
}

// A function which takes a closure and returns an `i32`.
// クロージャを引数に取り、`i32`を返す関数
fn apply_to_3<F>(f: F) -> i32 where
    // The closure takes an `i32` and returns an `i32`.
    // このクロージャは引数、返り値ともに`i32`
    F: Fn(i32) -> i32 {

    f(3)
}

fn main() {
    use std::mem;

    let greeting = "hello";
    // A non-copy type.
    // `to_owned` creates owned data from borrowed one
    // コピーではなくmoveが起きる型
    let mut farewell = "goodbye".to_owned();

    // Capture 2 variables: `greeting` by reference and
    // `farewell` by value.
    // 変数を2つ捕捉。`greeting`は参照を、
    // `farewell`は値をそれぞれ捕捉する。
    let diary = || {
        // `greeting` is by reference: requires `Fn`.
        // `greeting`は参照なので、`Fn`が必要。
        println!("I said {}.", greeting);

        // Mutation forces `farewell` to be captured by
        // mutable reference. Now requires `FnMut`.
        // `farewell`の値を変更するので、この時点で`FnMut`
        // が必要になる。
        farewell.push_str("!!!");
        println!("Then I screamed {}.", farewell);
        println!("Now I can sleep. zzzzz");

        // Manually calling drop forces `farewell` to
        // be captured by value. Now requires `FnOnce`.
        // `mem::drop`を明示的に呼ぶと`farewell`が値で
        // 捕捉される必要性が発生する。よって`FnOnce`が必要になる。
        mem::drop(farewell);
    };

    // Call the function which applies the closure.
    // クロージャを適用する関数を実行。
    apply(diary);

    // `double` satisfies `apply_to_3`'s trait bound
    let double = |x| 2 * x;

    println!("3 doubled: {}", apply_to_3(double));
}
```

<!--
### See also:
-->
### 参照

[`std::mem::drop`][drop], [`Fn`][fn], [`FnMut`][fnmut], [Generics][generics], [where][where] and [`FnOnce`][fnonce]

[drop]: https://doc.rust-lang.org/std/mem/fn.drop.html
[fn]: https://doc.rust-lang.org/std/ops/trait.Fn.html
[fnmut]: https://doc.rust-lang.org/std/ops/trait.FnMut.html
[fnonce]: https://doc.rust-lang.org/std/ops/trait.FnOnce.html
[generics]: ../../generics.md
[where]: ../../generics/where.md
