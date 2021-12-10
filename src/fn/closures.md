<!--
# Closures
-->
# クロージャ

<!--
Closures are functions that can capture the enclosing environment. For
example, a closure that captures the x variable:
-->
Rustにおけるクロージャは、その外側の環境を捕捉した関数のことです。例えば、次のコードは変数xを捕捉したクロージャです。

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
    // Increment via closures and functions.
    // 関数とクロージャのそれぞれで数値をインクリメントする
    fn function(i: i32) -> i32 { i + 1 }

    // Closures are anonymous, here we are binding them to references
    // Annotation is identical to function annotation but is optional
    // as are the `{}` wrapping the body. These nameless functions
    // are assigned to appropriately named variables.
    // 型アノテーションは、通常の関数と同様の方法で行えるが、必須ではない。
    // `{}`も必須ではない。
    // クロージャは一種の無名関数なので、適切な変数にバインディングしてやるとよい
    let closure_annotated = |i: i32| -> i32 { i + 1 };
    let closure_inferred  = |i     |          i + 1  ;

    let i = 1;
    // Call the function and closures.
    // 関数とクロージャを呼び出す。
    println!("function: {}", function(i));
    println!("closure_annotated: {}", closure_annotated(i));
    println!("closure_inferred: {}", closure_inferred(i));

    // A closure taking no arguments which returns an `i32`.
    // The return type is inferred.
    // 引数なしで`i32`を返すクロージャ。
    // 戻り値の型は推論された。
    let one = || 1;
    println!("closure returning one: {}", one());

}
```
