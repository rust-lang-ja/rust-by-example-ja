<!--
# `Option` & `unwrap`
-->
# `Option` と `unwrap`

<!--
In the last example, we showed that we can induce program failure at will.
We told our program to `panic` if we drink a sugary lemonade.
But what if we expect _some_ drink but don't receive one?
That case would be just as bad, so it needs to be handled!
 -->
以前の例では、甘いレモネードを飲んだ際に`panic`を呼び出すことによって、自由にプログラムの実行を失敗させられることが分かりました。では、何らかの飲み物を期待しているにもかかわらず、何も受け取らなかったらどうなるでしょう？これは悲惨なケースになるので、エラーハンドリングする必要があります！

<!--
We *could* test this against the null string (`""`) as we do with a lemonade.
Since we're using Rust, let's instead have the compiler point out cases
where there's no drink.
-->
このケースに対して、レモネードと同じように、空文字列（`""`）と比較することもできますが、せっかくRustを使っているので、その代わりにコンパイラに飲み物がないケースを指摘させてみましょう。

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
// The adult has seen it all, and can handle any drink well.
// All drinks are handled explicitly using `match`.
// 大人は経験豊富なので、大体どんな飲み物にも対処できます。
// あらゆる飲み物は`match`を用いて手動で処理されます。
fn give_adult(drink: Option<&str>) {
    // Specify a course of action for each case.
    match drink {
        Some("lemonade") => println!("Yuck! Too sugary."),
        Some(inner)   => println!("{}? How nice.", inner),
        None          => println!("No drink? Oh well."),
    }
}

// Others will `panic` before drinking sugary drinks.
// All drinks are handled implicitly using `unwrap`.
// 他の人たちは甘い飲み物を飲む前に`panic`します。
// 全ての飲み物は`unwrap`を使って暗黙的に処理されます。
fn drink(drink: Option<&str>) {
    // `unwrap` returns a `panic` when it receives a `None`.
    // `unwrap`を使用すると値が`None`だった際に`panic`を返します。
    let inside = drink.unwrap();
    if inside == "lemonade" { panic!("AAAaaaaa!!!!"); }

    println!("I love {}s!!!!!", inside);
}

fn main() {
    let water  = Some("water");
    let lemonade = Some("lemonade");
    let void  = None;

    give_adult(water);
    give_adult(lemonade);
    give_adult(void);

    let coffee = Some("coffee");
    let nothing = None;

    drink(coffee);
    drink(nothing);
}
```

[expect]: https://doc.rust-lang.org/std/option/enum.Option.html#method.expect
