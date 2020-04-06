fn main() {
    // サフィックスを指定したリテラル。型は初期化とともに確定する。
    let x = 1u8;
    let y = 2u32;
    let z = 3f32;

    // サフィックスを指定しないリテラル。型は使用方法に依存する。
    let i = 1;
    let f = 1.0;

    // `size_of_val` 関数は変数のサイズをバイトで返す。
    println!("size of `x` in bytes: {}", std::mem::size_of_val(&x));
    println!("size of `y` in bytes: {}", std::mem::size_of_val(&y));
    println!("size of `z` in bytes: {}", std::mem::size_of_val(&z));
    println!("size of `i` in bytes: {}", std::mem::size_of_val(&i));
    println!("size of `f` in bytes: {}", std::mem::size_of_val(&f));
}
