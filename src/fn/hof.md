<!--
# Higher Order Functions
-->
# 高階関数

<!--
Rust provides Higher Order Functions (HOF). These are functions that
take one or more functions and/or produce a more useful function. HOFs
and lazy iterators give Rust its functional flavor.
-->
Rustには高階関数(`Higher Order Functions, HOF`)を扱う機能が備わっています。

```rust,editable
fn is_odd(n: u32) -> bool {
    n % 2 == 1
}

fn main() {
    // 1000以下の奇数を2乗した値の合計を求める。
    println!("Find the sum of all the squared odd numbers under 1000");
    let upper = 1000;

    // Imperative approach
    // Declare accumulator variable
    // 宣言型プログラミングによるアプローチ
    // 値を蓄積する変数を宣言
    let mut acc = 0;
    // Iterate: 0, 1, 2, ... to infinity
    // 0から無限までイテレートする
    for n in 0.. {
        // Square the number
        // 値を2乗
        let n_squared = n * n;

        if n_squared >= upper {
            // Break loop if exceeded the upper limit
            // 上限に達した場合、ループを終了
            break;
        } else if is_odd(n_squared) {
            // Accumulate value, if it's odd
            // 奇数ならば値を値を足しあわせていく。
            acc += n_squared;
        }
    }
    println!("imperative style: {}", acc);

    // Functional approach
    // 関数型プログラミングによるアプローチ
    let sum_of_squared_odd_numbers: u32 =
        (0..).map(|n| n * n)                             // All natural numbers squared
                                                         // 全自然数を2乗し
             .take_while(|&n_squared| n_squared < upper) // Below upper limit
                                                         // そのうち上限より小さい値で
             .filter(|&n_squared| is_odd(n_squared))     // That are odd
                                                         // かつ奇数のものを
             .fold(0, |acc, n_squared| acc + n_squared); // Sum them
             .sum();                                     // Sum them
    println!("functional style: {}", sum_of_squared_odd_numbers);
}
```

<!--
[Option][option]
and
[Iterator][iter]
implement their fair share of HOFs.
-->
[オプション型][option]
と
[イテレータ][iter]には高階関数が使用されています。

[option]: https://doc.rust-lang.org/core/option/enum.Option.html
[iter]: https://doc.rust-lang.org/core/iter/trait.Iterator.html
