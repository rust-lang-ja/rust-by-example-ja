fn main() {
    let pair = (2, -2);
    // TODO ^ `pair`の値を変更してみましょう。

    println!("Tell me about {:?}", pair);
    match pair {
        (x, y) if x == y => println!("These are twins"),
        //     ^ `if`とそれに続く条件式がガードです。
        (x, y) if x + y == 0 => println!("Antimatter, kaboom!"),
        (x, _) if x % 2 == 1 => println!("The first one is odd"),
        _ => println!("No correlation..."),
    }
}
