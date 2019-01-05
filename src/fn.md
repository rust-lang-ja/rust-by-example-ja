<!-- Functions are declared using the `fn` keyword. Its arguments are type
annotated, just like variables, and, if the function returns a value, the
return type must be specified after an arrow `->`. -->
関数は`fn`キーワードを用いて定義することができます。引数は変数と同様に型を指定する必要があり、もし関数が値を返すならば`->`の後にその型も指定する必要があります。

<!-- The final expression in the function will be used as return value.
Alternatively, the `return` statement can be used to return a value earlier
from within the function, even from inside loops or `if`s. -->
関数内の最後の式文が返り値となります。関数の途中で値を返したい場合は`return`文を使用します。`loop`の最中や`if`文の中からも値を返すことができます。

<!-- Let's rewrite FizzBuzz using functions! -->
では、もう一度FizzBuzz問題を解く関数を書いてみましょう！

``` rust,editable
// C/C++とは違い、関数の定義を行う順番に制限はない。
fn main() {
    // ここで関数を使用し、後ほど定義してもかまわない。
    fizzbuzz_to(100);
}

// ブーリアン型を返す関数
fn is_divisible_by(lhs: u32, rhs: u32) -> bool {
    // 例外的な引数を受けた場合、早めに返す。
    if rhs == 0 {
        return false;
    }

    // これは式文であり、`return`キーワードは必要ではない。
    lhs % rhs == 0
}

// 値を「返さない」関数、実際にはユニット型(`()`)を返している。
fn fizzbuzz(n: u32) -> () {
    if is_divisible_by(n, 15) {
        println!("fizzbuzz");
    } else if is_divisible_by(n, 3) {
        println!("fizz");
    } else if is_divisible_by(n, 5) {
        println!("buzz");
    } else {
        println!("{}", n);
    }
}

// 関数が`()`を返すとき、返り値の方を書く必要はない。
fn fizzbuzz_to(n: u32) {
    for n in 1..n + 1 {
        fizzbuzz(n);
    }
}

```
