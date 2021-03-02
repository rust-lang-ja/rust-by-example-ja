<!--
# Elision
-->
# 省略

<!--
Some lifetime patterns are overwhelmingly common and so the borrow checker
will allow you to omit them to save typing and to improve readability.
This is known as elision. Elision exists in Rust solely because these patterns
are common.
-->
ライフタイムのパターンのうちのいくつかは、他と比べてあまりにも一般的に使用されるため、タイプ量を減らし可読性を上げるために省くことができます。これは省略として知られており、それらのパターンが一般的であるというだけの理由で存在しています。

<!--
The following code shows a few examples of elision. For a more comprehensive
description of elision, see [lifetime elision][elision] in the book.
-->
以下のコードでは省略の例を幾つかお見せします。より完全な説明を見たい場合は、「プログラミング言語Rust」の[ライフタイムの省略](elision-ja)の項を見てください。

```rust,editable
// `elided_input` and `annotated_input` essentially have identical signatures
// because the lifetime of `elided_input` is inferred by the compiler:
// `elided_input`のライフタイムはコンパイラによって自動的に付与されるため
// 以下の2つは同一のライフタイムシグネチャを持つ。
fn elided_input(x: &i32) {
    println!("`elided_input`: {}", x);
}

fn annotated_input<'a>(x: &'a i32) {
    println!("`annotated_input`: {}", x);
}

// Similarly, `elided_pass` and `annotated_pass` have identical signatures
// because the lifetime is added implicitly to `elided_pass`:
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

<!--
### See also:
-->
### 参照

<!--
[elision][elision]
-->
[ライフタイムの省略][elision]

[elision]: https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html#lifetime-elision
