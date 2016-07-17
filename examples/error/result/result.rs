fn double_number(number_str: &str) -> i32 {
    // 文字列は常に他の型にパースできるとは限らない。
    // ゆえに、`parse()`は失敗の可能性があることを意味する`Result`型を返す。
    // ここでは単に`unwrap`して数字を取り出すことを試みている。
    // これが、良くない事態を招く可能性とは？
    2 * number_str.parse::<i32>().unwrap()
}

fn main() {
    let twenty = double_number("10");
    println!("double is {}", twenty);

    let tt = double_number("t");
    println!("double is {}", tt);
}
