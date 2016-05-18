fn main() {
    let pair = (0, -2);
    // TODO ^ `pair`に別の値を入れてみましょう。

    println!("Tell me about {:?}", pair);
    // `match`を用いてタプルをデストラクトしてみましょう。
    match pair {
        // 2つ目の値をデストラクト
        (0, y) => println!("First is `0` and `y` is `{:?}`", y),
        (x, 0) => println!("`x` is `{:?}` and last is `0`", x),
        _      => println!("It doesn't matter what they are"),
        // ここでは`_`は、値を変数に束縛しないことを意味します。
    }
}
