<!-- Some lifetime patterns are overwelmingly common and so the borrow checker
will implicitly add them to save typing and to improve readability.
This process of implicit addition is called elision. Elision exists in Rust
solely because these patterns are common. -->
ライフタイムのパターンのうちのいくつかは、他と比べてあまりにも一般的に使用されるため、明示的に入力せずとも借用チェッカーが暗黙のうちに補完してくれます。これにより可読性とタイプ量を減らすことができます。


<!-- The following code shows a few examples of elision. For a more comprehensive
description of elision, see [lifetime elision][elision] in the book. -->
以下のコードでは省略の例を幾つかお見せします。より完全な説明を見たい場合は、「プログラミング言語Rust」の[ライフタイムの省略](elision-ja)の項を見てください。

``` rust,editable
// `elided_input`のライフタイムはコンパイラによって自動的に付与されるため
// 以下の2つは同一のライフタイムシグネチャを持つ。
fn elided_input(x: &i32) {
    println!("`elided_input`: {}", x)
}

fn annotated_input<'a>(x: &'a i32) {
    println!("`annotated_input`: {}", x)
}

// 同様に、以下の2つの関数も全く同じライフタイムシグネチャを持つ。
fn elided_pass(x: &i32) -> &i32 { x }

fn annotated_pass<'a>(x: &'a i32) -> &'a i32 { x }

fn main() {
    let x = 3;

    elided_input(&x);
    annotated_input(&x);

    println!("`elided_pass`: {}", elided_pass(&x));
    println!("`annotated_pass`: {}", annotated_pass(&x));
}

```

### See also:

[ライフタイムの省略][elision-ja]

[elision-ja]: https://rust-lang-ja.github.io/the-rust-programming-language-ja/1.6/book/lifetimes.html#ライフタイムの省略
