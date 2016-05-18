fn main() {
    struct Foo { x: (u32, u32), y: u32 }

    // 構造体のメンバをデストラクト
    let foo = Foo { x: (1, 2), y: 3 };
    let Foo { x: (a, b), y } = foo;

    println!("a = {}, b = {},  y = {} ", a, b, y);

    // 構造体をデストラクトして変数をリネーム
    // 順番は重要ではない。

    let Foo { y: i, x: j } = foo;
    println!("i = {:?}, j = {:?}", i, j);

    // 一部の変数を無視することもできる。
    let Foo { y, .. } = foo;
    println!("y = {}", y);

    // `x`に言及していないため、以下はエラーになる。
    // let Foo { y } = foo;
}
