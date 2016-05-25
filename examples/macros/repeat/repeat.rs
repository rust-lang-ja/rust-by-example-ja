// `min!`は引数として与えられた数字の中の最低の値を計算する。
macro_rules! find_min {
    // 基本となるケース
    ($x:expr) => ($x);
    // `$x`に少なくとも1つの`$y`が続く場合
    ($x:expr, $($y:expr),+) => (
        // `find_min!`を残りの`$y`に対して再帰的に適用
        std::cmp::min($x, find_min!($($y),+))
    )
}

fn main() {
    println!("{}", find_min!(1u32));
    println!("{}", find_min!(1u32 + 2 , 2u32));
    println!("{}", find_min!(5u32, 2u32 * 3, 4u32));
}
