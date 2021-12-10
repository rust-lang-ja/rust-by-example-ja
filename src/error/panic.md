# `panic`

<!--
The simplest error handling mechanism we will see is `panic`. It prints an
error message, starts unwinding the stack, and usually exits the program.
Here, we explicitly call `panic` on our error condition:
-->
`panic`は、最もシンプルなエラーハンドリングの仕組みです。エラーメッセージの出力、スタックの巻き戻し、そして多くの場合プログラムの終了を実行します。
例として、エラー条件に対して明示的に`panic`を呼び出してみましょう。

```rust,editable,ignore,mdbook-runnable
fn drink(beverage: &str) {
    // You shouldn't drink too much sugary beverages.
    // Princesses hate snakes, so we need to stop if she disapproves!
    // 甘すぎる飲み物を飲むべきではありません。
    if beverage == "lemonade" { panic!("AAAaaaaa!!!!"); }

    println!("Some refreshing {} is all I need.", beverage);
}

fn main() {
    drink("water");
    drink("lemonade");
}
```
