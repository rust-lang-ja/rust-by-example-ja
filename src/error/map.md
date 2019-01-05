<!-- `match` is a valid method for handling `Option`s. However, you may eventually
find heavy usage tedious; this is the case especially with operations that
are only valid with an input. -->
`match`は`Option`は扱うのに適したメソッドです。しかし、大量にこれを使用しているとじきに億劫になってくるでしょう。引数の値が有効である(訳注: この場合は`None`でない)必要がある関数を扱う際には特にそうです。

<!-- For situations where a simplistic mapping of `Some -> Some` and
`None -> None` is needed, `Option` has a built in method called `map()`. -->
`Some -> Some`あるいは`None -> None`の単純な操作を適用する必要がある場合には、`Option`は`map()`というビルトインのメソッドを提供していますので、これを使用しましょう。

<!-- Multiple `map()` calls can be chained together for even more flexibility.
In the following example, `process()` easily replaces all functions previous
to it while staying compact. -->
`map()`のフレキシビリティは、複数の`map()`をチェインしなければならない場合にさらに際立ちます。以下の例では、`process()`が直前の関数全てを用いた場合と同じ機能を、よりコンパクトに果たしているのがわかります。

``` rust,editable
#![allow(dead_code)]

#[derive(Debug)] enum Food { Apple, Carrot, Potato }

#[derive(Debug)] struct Peeled(Food);
#[derive(Debug)] struct Chopped(Food);
#[derive(Debug)] struct Cooked(Food);

// 食べ物の皮をむく。存在しない場合は単純に`None`を返す。
// そうでなければ皮を向いた食べ物を返す。
fn peel(food: Option<Food>) -> Option<Peeled> {
    match food {
        Some(food) => Some(Peeled(food)),
        None       => None,
    }
}

// 上と同じように、食べ物を切る前に、皮を向いた食べ物の有無を知る必要がある。
fn chop(peeled: Option<Peeled>) -> Option<Chopped> {
    match peeled {
        Some(Peeled(food)) => Some(Chopped(food)),
        None               => None,
    }
}

// 上のチェックと同様だが`match`の代わりに`map()`を使用している。
fn cook(chopped: Option<Chopped>) -> Option<Cooked> {
    chopped.map(|Chopped(food)| Cooked(food))
}

// 複数の`map()`をチェインさせて、上のプロセスをシンプルにすることもできる。
fn process(food: Option<Food>) -> Option<Cooked> {
    food.map(|f| Peeled(f))
        .map(|Peeled(f)| Chopped(f))
        .map(|Chopped(f)| Cooked(f))
}

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
    // よりシンプルな見た目の`process()`を使用する。
    let cooked_potato = process(potato);

    eat(cooked_apple);
    eat(cooked_carrot);
    eat(cooked_potato);
}

```

### See also:

[closures][closures]

[closures]: ../fn/closures.html
