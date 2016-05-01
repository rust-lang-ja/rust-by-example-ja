fn main() {
    // 変数に型を指定
    let logical: bool = true;

    let a_float: f64 = 1.0;  // 通常の型指定
    let an_integer   = 5i32; // サフィックスによる型指定

    // サフィックスを指定しない場合、デフォルトを選択
    let default_float   = 3.0; // `f64`
    let default_integer = 7;   // `i32`

    let mut mutable = 12; // ミュータブルな `i32`.

    // エラー！ ミュータブルな変数でも型は不変
    mutable = true;
}
