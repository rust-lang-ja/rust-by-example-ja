<!--
# Closures
-->
# クロージャ

<!--
Closures are functions that can capture the enclosing environment. For
example, a closure that captures the `x` variable:
-->
Rustにおけるクロージャは、その外側の環境を捕捉した関数のことです。例えば、次のコードは変数`x`を捕捉したクロージャです。

```Rust
|val| val + x
```

<!--
The syntax and capabilities of closures make them very convenient for
on the fly usage. Calling a closure is exactly like calling a function.
However, both input and return types *can* be inferred and input
variable names *must* be specified.
-->
クロージャの構文や機能は、その場限りの用途で何かを作るのに便利です。クロージャの呼び出しは関数の呼び出しと全く同じです。しかし、入力の型と戻り値の型は推論させることができますが、入力変数の名前は必ず指定しなくてはなりません。

<!--
Other characteristics of closures include:
* using `||` instead of `()` around input variables.
* optional body delimination (`{}`) for a single expression (mandatory otherwise).
* the ability to capture the outer environment variables.
-->
クロージャの他の特徴を以下に示します。
* 入力変数を囲むのに、`()`の代わりに`||`を用います。
* 本体が単一の式の場合は、本体の区切り文字（`{}`）を省略できます。（それ以外の場合は必須です）
* 外側の環境にある変数を捕捉することができます。

```rust,editable
fn main() {
    let outer_var = 42;
    
    // A regular function can't refer to variables in the enclosing environment
    // Increment via closures and functions.
    // 関数とクロージャのそれぞれで数値をインクリメントする
    //fn function(i: i32) -> i32 { i + outer_var }
    // TODO: uncomment the line above and see the compiler error. The compiler
    // suggests that we define a closure instead.

    // Closures are anonymous, here we are binding them to references
    // Annotation is identical to function annotation but is optional
    // as are the `{}` wrapping the body. These nameless functions
    // are assigned to appropriately named variables.
    // 型アノテーションは、通常の関数と同様の方法で行えるが、必須ではない。
    // `{}`も必須ではない。
    // クロージャは一種の無名関数なので、適切な変数にバインディングしてやるとよい
    let closure_annotated = |i: i32| -> i32 { i + outer_var };
    let closure_inferred  = |i     |          i + outer_var  ;

    // Call the closures.
    // クロージャを呼び出す。
    println!("closure_annotated: {}", closure_annotated(1));
    println!("closure_inferred: {}", closure_inferred(1));
    // Once closure's type has been inferred, it cannot be inferred again with another type.
    //println!("cannot reuse closure_inferred with another type: {}", closure_inferred(42i64));
    // TODO: uncomment the line above and see the compiler error.

    // A closure taking no arguments which returns an `i32`.
    // The return type is inferred.
    // 引数なしで`i32`を返すクロージャ。
    // 戻り値の型は推論された。
    let one = || 1;
    println!("closure returning one: {}", one());

}
```
