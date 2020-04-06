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
