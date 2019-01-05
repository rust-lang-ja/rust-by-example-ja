<!-- Sometimes it's desirable to catch the failure of some parts of a program
instead of calling `panic!`; this can be accomplished using the `Option` enum. -->
プログラムの一部が失敗した際、`panic!`するよりも、エラーを補足する方が望ましい場合があります。これは`Option`という列挙型を用いることで可能になります。

<!-- The `Option<T>` enum has two variants: -->
列挙型`Option<T>`には2つの値があります。

<!-- * `None`, to indicate failure or lack of value, and
* `Some(value)`, a tuple struct that wraps a `value` with type `T`. -->
* `None`、これは実行の失敗か値の欠如を示します。
* `Some(value)`、型`T`の`value`をラップするタプルです。

``` rust,editable,ignore,mdbook-runnable
// `panic!`を起こさない整数の割り算
fn checked_division(dividend: i32, divisor: i32) -> Option<i32> {
    if divisor == 0 {
        // 失敗は`None`としてあらわされる。
        None
    } else {
        // 結果は`Some`にラップされる。
        Some(dividend / divisor)
    }
}

// この関数は失敗する割り算を扱うことができる
fn try_division(dividend: i32, divisor: i32) {
    // `Option` の値は、他のあらゆる列挙型と同様パターンマッチに使用できる。
    match checked_division(dividend, divisor) {
        None => println!("{} / {} failed!", dividend, divisor),
        Some(quotient) => {
            println!("{} / {} = {}", dividend, divisor, quotient)
        },
    }
}

fn main() {
    try_division(4, 2);
    try_division(1, 0);

    // `None`を変数にアサインする際は、型を明示しなくてはならない。
    let none: Option<i32> = None;
    let _equivalent_none = None::<i32>;

    let optional_float = Some(0f32);

    // `Some`をアンラップすると中の値を取得できる。
    println!("{:?} unwraps to {:?}", optional_float, optional_float.unwrap());

    // `None`をアンラップしようとすると`panic!`る
    println!("{:?} unwraps to {:?}", none, none.unwrap());
}

```
