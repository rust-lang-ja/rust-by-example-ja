fn main() {
    let mut count = 0u32;

    println!("Let's count until infinity!");

    // 無限ループ
    loop {
        count += 1;

        if count == 3 {
            println!("three");

            // 残りの処理をスキップ
            continue;
        }

        println!("{}", count);

        if count == 5 {
            println!("OK, that's enough");

            // Exit this loop
            // ループを抜ける。
            break;
        }
    }
}
