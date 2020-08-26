<!--
# `Option` & `unwrap`
-->
# `Option` と `unwrap`

<!--
In the last example, we showed that we can induce program failure at will. 
We told our program to `panic` if the princess received an inappropriate 
gift - a snake. But what if the princess expected a gift and didn't receive 
one? That case would be just as bad, so it needs to be handled!
 -->
以前の例では、お姫様に不適切な贈り物（ヘビ）を渡した際に`panic`を呼び出すことによって、自由にプログラムの実行を失敗させられることが分かりました。では、お姫様が贈り物を期待しているにもかかわらず、何も受け取らなかったらどうなるでしょう？ヘビを受け取るのに劣らない悲惨なケースになるので、エラーハンドリングする必要があります！

<!--
We *could* test this against the null string (`""`) as we do with a snake. 
Since we're using Rust, let's instead have the compiler point out cases 
where there's no gift.
-->
このケースに対して、ヘビと同じように、空文字列（`""`）と比較することもできますが、せっかくRustを使っているので、その代わりにコンパイラに贈り物がないケースを指摘させてみましょう。

<!--
An `enum` called `Option<T>` in the `std` library is used when absence is a 
possibility. It manifests itself as one of two "options":
-->
`std`ライブラリの中の、`Option<T>`と呼ばれる`enum`は、任意の型`T`である変数の値が存在しない可能性がある場合に用いられます。値の状態によって、下記２つのパターンのうちの１つとして扱われます。

<!--
* `Some(T)`: An element of type `T` was found
* `None`: No element was found
-->
* `Some(T)`: 型`T`の値がある場合
* `None`: 値が存在しない場合。

<!--
These cases can either be explicitly handled via `match` or implicitly with 
`unwrap`. Implicit handling will either return the inner element or `panic`.
-->
これらは`match`を用いて明示的に扱うこともできますし、`unwrap`で暗黙に処理することもできます。後者は`Some`の中の値を返すか`panic`するかのどちらかです。

<!--
Note that it's possible to manually customize `panic` with [expect][expect], 
but `unwrap` otherwise leaves us with a less meaningful output than explicit 
handling. In the following example, explicit handling yields a more 
controlled result while retaining the option to `panic` if desired.
-->
[expect]メソッドを用いて、`panic`を手動でカスタマイズできることに触れておきましょう。これは（`unwrap`をそのまま用いた場合よりも）内容が理解しやすいエラーメッセージを出力するのに役立ちます。次の例では、結果をより明示的に、可能ならいつでも`panic`できるように扱っていきます。

```rust,editable,ignore,mdbook-runnable
// The commoner has seen it all, and can handle any gift well.
// All gifts are handled explicitly using `match`.
// 庶民(commoner)は経験豊富なので、大体どんな状況にも対処できます。
// あらゆる贈り物は`match`を用いて手動で処理されます。
fn give_commoner(gift: Option<&str>) {
    // Specify a course of action for each case.
    match gift {
        Some("snake") => println!("Yuck! I'm putting this snake back in the forest."),
        Some(inner)   => println!("{}? How nice.", inner),
        None          => println!("No gift? Oh well."),
    }
}

// Our sheltered princess will `panic` at the sight of snakes.
// All gifts are handled implicitly using `unwrap`.
// 温室育ちのお姫様はヘビを見ると`panic`します。
fn give_princess(gift: Option<&str>) {
    // `unwrap` returns a `panic` when it receives a `None`.
    // `unwrap`を使用すると値が`None`だった際に`panic`を返します。。
    let inside = gift.unwrap();
    if inside == "snake" { panic!("AAAaaaaa!!!!"); }

    println!("I love {}s!!!!!", inside);
}

fn main() {
    let food  = Some("cabbage");
    let snake = Some("snake");
    let void  = None;

    give_commoner(food);
    give_commoner(snake);
    give_commoner(void);

    let bird = Some("robin");
    let nothing = None;

    give_princess(bird);
    give_princess(nothing);
}
```

[expect]: https://doc.rust-lang.org/std/option/enum.Option.html#method.expect
