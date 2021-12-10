<!--
# Repeat
-->
# 繰り返し

<!--
Macros can use `+` in the argument list to indicate that an argument may
repeat at least once, or `*`, to indicate that the argument may repeat zero or
more times.
-->
マクロは引数のリストの中で`+`を使うことができ、そうすることによって、引数が少なくとも1回以上繰り返されるということを示すことができます。同様に`*`の場合は、0以上を示します。

<!--
In the following example, surrounding the matcher with `$(...),+` will
match one or more expression, separated by commas.
Also note that the semicolon is optional on the last case.
-->
以下の例では、マッチ対象を `$(...),+`で囲むことにより、カンマで区切られた1つ以上の式とマッチします。最後のセミコロンは必須ではないことに注目しましょう。

```rust,editable
// `find_min!` will calculate the minimum of any number of arguments.
// `find_min!`は引数として与えられた数字の中の最低の値を計算する。
macro_rules! find_min {
    // Base case:
    // 基本となるケース
    ($x:expr) => ($x);
    // `$x` followed by at least one `$y,`
    // `$x`に少なくとも1つの`$y`が続く場合
    ($x:expr, $($y:expr),+) => (
        // Call `find_min!` on the tail `$y`
        // `find_min!`を残りの`$y`に対して再帰的に適用
        std::cmp::min($x, find_min!($($y),+))
    )
}

fn main() {
    println!("{}", find_min!(1));
    println!("{}", find_min!(1 + 2, 2));
    println!("{}", find_min!(5, 2 * 3, 4));
}
```
