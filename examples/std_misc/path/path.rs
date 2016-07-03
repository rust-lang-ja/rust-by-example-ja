use std::path::Path;

fn main() {
    // `&'static str`から`Path`を作成
    let path = Path::new(".");

    // `display`メソッドは`Show`可能な構造体を返す。
    let display = path.display();

    // `join`はOS固有のセパレータによってバイトのコンテナ型であるパス
    // を結合し、新しいパスを返す。
    let new_path = path.join("a").join("b");

    // パスを文字列のスライスに変換する。
    match new_path.to_str() {
        None => panic!("new path is not a valid UTF-8 sequence"),
        Some(s) => println!("new path is {}", s),
    }
}
