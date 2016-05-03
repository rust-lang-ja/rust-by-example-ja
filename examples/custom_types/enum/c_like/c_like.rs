// 使用されていないコードによる警告を抑えるアトリビュート
#![allow(dead_code)]

// 値を明示しない場合、0から整数が順に入る。
enum Number {
    Zero,
    One,
    Two,
}

// 値を明示する場合
enum Color {
    Red = 0xff0000,
    Green = 0x00ff00,
    Blue = 0x0000ff,
}

fn main() {
    // 列挙型の中身を整数としてキャストする。
    println!("zero is {}", Number::Zero as i32);
    println!("one is {}", Number::One as i32);

    println!("roses are #{:06x}", Color::Red as i32);
    println!("violets are #{:06x}", Color::Blue as i32);
}
