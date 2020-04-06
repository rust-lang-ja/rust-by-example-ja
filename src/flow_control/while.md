# while

<!--
The `while` keyword can be used to run a loop while a condition is true.
-->
`while`キーワードは条件が真である限り実行され続けるループのために使用します。

<!--
Let's write the infamous [FizzBuzz][fizzbuzz] using a `while` loop.
-->
悪名高い[FizzBuzz問題][fizzbuzz]を`while`を用いて解いてみましょう。

```rust,editable
fn main() {
    // A counter variable
    // カウンタとなる変数
    let mut n = 1;

    // Loop while `n` is less than 101
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

        // Increment counter
        // カウンタに1を追加
        n += 1;
    }
}
```

[fizzbuzz]: https://en.wikipedia.org/wiki/Fizz_buzz
