use std::io::prelude::*;
use std::fs::File;

type Result<T> = std::result::Result<T, String>;

// 以下が動作するための下準備。ファイルを作成して書き込んでおく。
// 返り値はここではどうでもよいので無視する。
fn setup() {
    File::create("a")
        .and_then(|mut file| file.write_all(b"grape"))
        .unwrap();

    File::create("b")
        .and_then(|mut file| file.write_all(b"fruit"))
        .unwrap();
}

// ファイルからデータを取り出し、値を`Result`として返す。
fn get_data(path: &str) -> Result<String> {
    File::open(path)
        .map_err(|err| err.to_string())
        .and_then(|mut file| {
            let mut contents = String::new();

            // データを`contents`に読み込む。
            file.read_to_string(&mut contents)
                .map_err(|err| err.to_string())
                // `read_to_string`の返り値は無視し、`contents`を返す。
                .map(|_| contents)
        })
}

// 二つのファイルの中身を結合し、新しい`Result`にして返す。
fn concat(filename_a: &str, filename_b: &str) -> Result<String> {
    let (data_a, data_b) = (get_data(filename_a), get_data(filename_b));

    data_a.and_then(|a|
        // `a`と`b`が両方とも`Ok`ならば`Ok`を返し、そうでなければ
        // 先に`Err`を挙げた方の`Err`を返す。
        data_b.and_then(|b| Ok(a + &b))
    )
}

fn main() {
    setup();

    match concat("a", "b") {
        Ok(n)  => println!("{}", n),
        Err(e) => println!("Error: {}", e),
    }
}
