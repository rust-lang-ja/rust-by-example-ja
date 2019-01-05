<!-- What if the specific `Result` type is reused many many times? Then quickly it becomes tedious
to write out the full type name. Instead, a generic alias for the specific `Result` may be
defined. -->
特定の`Result`型が何度も何度も使用されると何が起きるでしょう？正解は「型名を書き下すのがすぐに面倒になる」です。このような場合は特定の`Result`のジェネリックエイリアスを定義すると吉です。

``` rust,editable
use std::num::ParseIntError;
use std::result;

// `ParseIntError`を`Err`の型として持つ全ての`Result`のジェネリックエイリアス
type Result<T> = result::Result<T, ParseIntError>;

// 上で定義したエイリアス(この場所特有の`Result`型)を使用
fn double_number(number_str: &str) -> Result<i32> {
    number_str.parse::<i32>().map(|n| 2 * n)
}

// もう一度使用。エイリアスによって再度明記する必要性がない。
fn print(result: Result<i32>) {
    match result {
        Ok(n)  => println!("n is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

fn main() {
    print(double_number("10"));
    print(double_number("t"));
}

```

<!-- This is particularly helpful at a module level because all errors found in a specific module
may have the same `Err` type; a single alias succinctly defines *all* module `Results`. This
is so useful that the std library even supplies one: `io::Result` which refers to IO errors. -->
これはモジュールのレベルでは特に役立ちます。というのも特定のモジュールで見つかるエラーは同じ`Err`型(そのモジュール内の**全ての**`Result`を完結に表現する唯一のエイリアス)を持つ可能性があるからです。これは本当に有用なので標準ライブラリ内にもその例があります。IOエラー全般を代表する`io::Result`です。

> 訳注: ピンと来ない方は[Python プログラマーのための Rust 入門](http://qiita.com/t2y/items/434854fab16159a7c0f7)が参考になるかもしれません。

### See also:

[`Result`][result]、[`io::Result`][io_result]

[result]: http://doc.rust-lang.org/std/result/enum.Result.html
[io_result]: http://doc.rust-lang.org/std/io/type.Result.html
