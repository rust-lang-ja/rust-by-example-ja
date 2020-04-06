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
