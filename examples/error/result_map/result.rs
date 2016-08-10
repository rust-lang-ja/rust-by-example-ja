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
