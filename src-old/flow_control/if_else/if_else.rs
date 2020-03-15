fn main() {
    let n = 5;

    if n < 0 {
        print!("{} is negative", n);
    } else if n > 0 {
        print!("{} is positive", n);
    } else {
        print!("{} is zero", n);
    }

    let big_n =
        if n < 10 && n > -10 {
            println!(", and is a small number, increase ten-fold");

            // この式は`i32`を返す。
            10 * n
        } else {
            println!(", and is a big number, reduce by two");

            // ここでも返り値の型は`i32`でなくてはならない。
            n / 2
            // TODO ^ セミコロン(`;`)をつけて、返り値を返さないようにしてみましょう
        };
    //   ここにセミコロンを付けるのを忘れないように!
    //   `let`による変数束縛の際には必ず必要です!

    println!("{} -> {}", n, big_n);
}
