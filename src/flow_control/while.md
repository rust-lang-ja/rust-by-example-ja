<!-- The `while` keyword can be used to loop until a condition is met. -->
`while`キーワードは条件が満たされるまでのループのために使用します。

<!-- Let's write the infamous [FizzBuzz][fizzbuzz] using a `while` loop. -->
悪名高い[FizzBuzz問題][fizzbuzz]を`while`を用いて解いてみましょう。

``` rust,editable
fn main() {
    // カウンタとなる変数
    let mut n = 1;

    // `n`が101以下である場合のループ
    while n < 101 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }

        // カウンタに1を追加
        n += 1;
    }
}

```

[fizzbuzz]: http://en.wikipedia.org/wiki/Fizz_buzz
