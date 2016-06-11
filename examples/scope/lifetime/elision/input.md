<!-- Some lifetime patterns are overwelmingly common and so the borrow checker
will implicitly add them to save typing and to improve readability.
This process of implicit addition is called elision. Elision exists in Rust
solely because these patterns are common. -->
ライフタイムのパターンのうちのいくつかは、他と比べてあまりにも一般的に使用されるため、明示的に入力せずとも借用チェッカーが暗黙のうちに補完してくれます。これにより可読性とタイプ量を減らすことができます。


<!-- The following code shows a few examples of elision. For a more comprehensive
description of elision, see [lifetime elision][elision] in the book. -->
以下のコードでは省略の例を幾つかお見せします。より完全な説明を見たい場合は、「プログラミング言語Rust」の[ライフタイムの省略](elision-ja)の項を見てください。

{elision.play}

### See also:

[ライフタイムの省略][elision-ja]

[elision-ja]: https://rust-lang-ja.github.io/the-rust-programming-language-ja/1.6/book/lifetimes.html#ライフタイムの省略
