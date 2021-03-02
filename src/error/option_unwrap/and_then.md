# Combinators: `and_then`

<!--
`map()` was described as a chainable way to simplify `match` statements. 
However, using `map()` on a function that returns an `Option<T>` results 
in the nested `Option<Option<T>>`. Chaining multiple calls together can 
then become confusing. That's where another combinator called `and_then()`, 
known in some languages as flatmap, comes in.
-->
先ほどは`map()`を、チェイン構文を用いて`match`文を単純化する物として説明しました。しかし`Option<T>`を返す関数に対しての`map()`の使用はネストした`Option<Option<T>>`を生じさせます。ですので、複数の関数呼び出しをチェインさせることは混乱を招く場合があります。そんな時こそ`and_then()`の出番です。他の言語ではflatmapと呼ばれることもあります。

<!--
`and_then()` calls its function input with the wrapped value and returns the result. If the `Option` is `None`, then it returns `None` instead.
-->
`and_then()`は引数として与えられた関数にラップされた値を渡しますが、その値が`None`だった場合は`None`を返します。

<!--
In the following example, `cookable_v2()` results in an `Option<Food>`. 
Using `map()` instead of `and_then()` would have given an 
`Option<Option<Food>>`, which is an invalid type for `eat()`.
-->
以下の例では`cookable_v2()`は`Option<Food>`を返すため、`and_then()`ではなく`map()`を使用すると最終的に`Option<Option<Food>>`になります。これは`eat()`には不適切な型です。

```rust,editable
#![allow(dead_code)]

#[derive(Debug)] enum Food { CordonBleu, Steak, Sushi }
#[derive(Debug)] enum Day { Monday, Tuesday, Wednesday }

// We don't have the ingredients to make Sushi.
// 我々は寿司の材料を持っていない
fn have_ingredients(food: Food) -> Option<Food> {
    match food {
        Food::Sushi => None,
        _           => Some(food),
    }
}

// We have the recipe for everything except Cordon Bleu.
// コルドン・ブルー(Cordon Bleu)のレシピも持っていない。
fn have_recipe(food: Food) -> Option<Food> {
    match food {
        Food::CordonBleu => None,
        _                => Some(food),
    }
}

// To make a dish, we need both the recipe and the ingredients.
// We can represent the logic with a chain of `match`es:
// 料理を作るためには、材料とレシピの両方が必要。
// ロジックの流れを`match`のチェインで表す。
fn cookable_v1(food: Food) -> Option<Food> {
    match have_recipe(food) {
        None       => None,
        Some(food) => match have_ingredients(food) {
            None       => None,
            Some(food) => Some(food),
        },
    }
}

// This can conveniently be rewritten more compactly with `and_then()`:
// `and_then()`を用いることで、同じことをよりコンパクトに表現できる。
fn cookable_v2(food: Food) -> Option<Food> {
    have_recipe(food).and_then(have_ingredients)
}

fn eat(food: Food, day: Day) {
    match cookable_v2(food) {
        Some(food) => println!("Yay! On {:?} we get to eat {:?}.", day, food),
        None       => println!("Oh no. We don't get to eat on {:?}?", day),
    }
}

fn main() {
    let (cordon_bleu, steak, sushi) = (Food::CordonBleu, Food::Steak, Food::Sushi);

    eat(cordon_bleu, Day::Monday);
    eat(steak, Day::Tuesday);
    eat(sushi, Day::Wednesday);
}
```

<!--
### See also:
-->
### 参照

<!--
[closures][closures], [`Option`][option], and [`Option::and_then()`][and_then]
-->
[closures][closures], [`Option`][option], [`Option::and_then()`][and_then]

[closures]: ../../fn/closures.md
[option]: https://doc.rust-lang.org/std/option/enum.Option.html
[and_then]: https://doc.rust-lang.org/std/option/enum.Option.html#method.and_then
