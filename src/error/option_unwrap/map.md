# Combinators: `map`

<!--
`match` is a valid method for handling `Option`s. However, you may 
eventually find heavy usage tedious, especially with operations only valid 
with an input. In these cases, [combinators][combinators] can be used to 
manage control flow in a modular fashion.
-->
`match`は`Option`は扱うのに適したメソッドです。しかし、大量にこれを使用しているとじきに億劫になってくるでしょう。引数の値が有効である(訳注: この場合は`None`でない)必要がある関数を扱う際には特にそうです。
In these cases, [combinators][combinators] can be used to manage control flow in a modular fashion.

<!--
`Option` has a built in method called `map()`, a combinator for the simple 
mapping of `Some -> Some` and `None -> None`. Multiple `map()` calls can be 
chained together for even more flexibility.
-->
`Some -> Some`あるいは`None -> None`の単純な操作を適用する必要がある場合には、`Option`は`map()`というビルトインのメソッドを提供していますので、これを使用しましょう。
`map()`のフレキシビリティは、複数の`map()`をチェインしなければならない場合にさらに際立ちます。

<!--
In the following example, `process()` replaces all functions previous
to it while staying compact.
-->
以下の例では、`process()`が直前の関数全てを用いた場合と同じ機能を、よりコンパクトに果たしているのがわかります。

```rust,editable
#![allow(dead_code)]

#[derive(Debug)] enum Food { Apple, Carrot, Potato }

#[derive(Debug)] struct Peeled(Food);
#[derive(Debug)] struct Chopped(Food);
#[derive(Debug)] struct Cooked(Food);

// Peeling food. If there isn't any, then return `None`.
// Otherwise, return the peeled food.
// 食べ物の皮をむく。存在しない場合は単純に`None`を返す。
// そうでなければ皮を向いた食べ物を返す。
fn peel(food: Option<Food>) -> Option<Peeled> {
    match food {
        Some(food) => Some(Peeled(food)),
        None       => None,
    }
}

// Chopping food. If there isn't any, then return `None`.
// Otherwise, return the chopped food.
// 上と同じように、食べ物を切る前に、皮を向いた食べ物の有無を知る必要がある。
fn chop(peeled: Option<Peeled>) -> Option<Chopped> {
    match peeled {
        Some(Peeled(food)) => Some(Chopped(food)),
        None               => None,
    }
}

// Cooking food. Here, we showcase `map()` instead of `match` for case handling.
// 上のチェックと同様だが`match`の代わりに`map()`を使用している。
fn cook(chopped: Option<Chopped>) -> Option<Cooked> {
    chopped.map(|Chopped(food)| Cooked(food))
}

// A function to peel, chop, and cook food all in sequence.
// We chain multiple uses of `map()` to simplify the code.
// 複数の`map()`をチェインさせて、上のプロセスをシンプルにすることもできる。
fn process(food: Option<Food>) -> Option<Cooked> {
    food.map(|f| Peeled(f))
        .map(|Peeled(f)| Chopped(f))
        .map(|Chopped(f)| Cooked(f))
}

// Check whether there's food or not before trying to eat it!
// 食べる前に、食べ物の有無をチェックするのは大事ですよね!
fn eat(food: Option<Cooked>) {
    match food {
        Some(food) => println!("Mmm. I love {:?}", food),
        None       => println!("Oh no! It wasn't edible."),
    }
}

fn main() {
    let apple = Some(Food::Apple);
    let carrot = Some(Food::Carrot);
    let potato = None;

    let cooked_apple = cook(chop(peel(apple)));
    let cooked_carrot = cook(chop(peel(carrot)));
    // Let's try the simpler looking `process()` now.
    // よりシンプルな見た目の`process()`を使用する。
    let cooked_potato = process(potato);

    eat(cooked_apple);
    eat(cooked_carrot);
    eat(cooked_potato);
}
```

### See also:

[closures][closures], [`Option`][option], [`Option::map()`][map]

[combinators]: https://doc.rust-lang.org/book/glossary.html#combinators
[closures]: ../../fn/closures.md
[option]: https://doc.rust-lang.org/std/option/enum.Option.html
[map]: https://doc.rust-lang.org/std/option/enum.Option.html#method.map
