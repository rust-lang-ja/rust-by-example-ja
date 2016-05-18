fn main() {
    let number = 13;

    println!("Tell me about {}", number);
    match number {
        // 単一の値とのマッチをチェック
        1 => println!("One!"),
        // いくつかの値とのマッチをチェック
        2 | 3 | 5 | 7 | 11 => println!("This is a prime"),
        // 特定の範囲の値とのマッチをチェック
        13...19 => println!("A teen"),
        // その他の場合の処理
        _ => println!("Ain't special"),
    }

    let boolean = true;
    // マッチは式文でもある。
    let binary = match boolean {
        // マッチは全ての可能な値をカバーしなくてはならない
        false => 0,
        true => 1,
        // TODO ^ 試しに片方をコメントアウトしてみましょう。
    };

    println!("{} -> {}", boolean, binary);
}
