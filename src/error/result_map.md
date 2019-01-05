<!-- To avoid the `unwrap()` in the previous example, we will have to rewrite the example to be
specific about what type it returns. In this case, the regular element should definitely
be `i32` but what about the `Err` type? Well, `parse()` is implemented with the
[`FromStr trait`][from_str] for [`i32`][i32]. That implementation specifies the
`Err` type as [`ParseIntError`][parse_int_error]. -->
先ほどの例において`unwrap()`の使用を避けるため、どのような型を返すのかについてを明示するように書き換える必要があります。この場合、通常時の要素は間違いなく`i32`とすべきですが、`Err`の型はどうでしょう。えーと、`parse()`は[`i32`][i32]の[`FromStr trait`][from_str]にて実装されています。その実装から、`Err`の型は[`ParseIntErr`][parse_int_error]となっていることがわかります。

``` rust,editable
use std::num::ParseIntError;

// 返り値の型を書き直し、`unwrap()`を用いないパターンマッチに変更したが、
// まだ少しごちゃごちゃしている。`Option`の場合と同様に
// スッキリさせられないだろうか？答えはYes
fn double_number(number_str: &str) -> Result<i32, ParseIntError> {
    match number_str.parse::<i32>() {
        Ok(n)  => Ok(2 * n),
        Err(e) => Err(e),
    }
}

// 上と全く同じ機能を、`map()`を用いて記述する。
// 値がparse可能な時のみその値を変更し、そうでなければエラーを返す。
fn double_number_map(number_str: &str) -> Result<i32, ParseIntError> {
    number_str.parse::<i32>().map(|n| 2 * n)
}

fn print(result: Result<i32, ParseIntError>) {
    match result {
        Ok(n)  => println!("n is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

fn main() {
    // 以前と同様、問題なく想定通りの値を表示する。
    let twenty = double_number("10");
    print(twenty);

    // 以前の`panic`の内容よりも遥かに良い。
    let tt = double_number_map("t");
    print(tt);
}

```

<!-- Similar to `Option`, `Result` has many other combinators besides `map` such as `and_then`
and `unwrap_or`; even ones to handle the errors specifically such as `map_err`.
`Result` contains the complete listing. -->
`Option`と同様、`Result`は`map`以外にも`and_then`や`unwrap_or`のような便利なコンビネータを多く持っています。中には`map_err`のようにエラーのみを扱うようなものも存在します。完全な一覧は[`Result`][result]を御覧ください。

### See also:

[`i32`][i32], [`FromStr`][from_str], [`ParseIntErr`][parse_int_error],
[`Result`][result]

[result]: http://doc.rust-lang.org/std/result/enum.Result.html
[parse_int_error]: http://doc.rust-lang.org/std/num/struct.ParseIntError.html
[from_str]: http://doc.rust-lang.org/std/str/trait.FromStr.html
[i32]: http://doc.rust-lang.org/std/primitive.i32.html
