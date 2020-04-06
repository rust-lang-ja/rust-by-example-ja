fn is_odd(n: u32) -> bool {
    n % 2 == 1
}

fn main() {
    // 1000以下の奇数を2乗した値の合計を求める。
    println!("Find the sum of all the squared odd numbers under 1000");
    let upper = 1000;

    // 宣言型プログラミングによるアプローチ
    // 値を蓄積する変数を宣言
    let mut acc = 0;
    // 0から無限までイテレートする
    for n in 0.. {
        // 値を2乗
        let n_squared = n * n;

        if n_squared >= upper {
            // 上限に達した場合、ループを終了
            break;
        } else if is_odd(n_squared) {
            // 奇数ならば値を値を足しあわせていく。
            acc += n_squared;
        }
    }
    println!("imperative style: {}", acc);

    // 関数型プログラミングによるアプローチ
    let sum_of_squared_odd_numbers: u32 =
        (0..).map(|n| n * n)             // 全自然数を2乗し
             .take_while(|&n| n < upper) // そのうち上限より小さい値で
             .filter(|n| is_odd(*n))     // かつ奇数のものを
             .fold(0, |sum, i| sum + i); // 足し合わせる。
    println!("functional style: {}", sum_of_squared_odd_numbers);
}
