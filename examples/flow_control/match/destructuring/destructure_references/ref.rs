fn main() {
    // 通常の2つの値からスタート
    let value = 5;
    let mut mut_value = 6;

    // `&5`(5へのリファレンス)へとデストラクトする場合、`ref`キーワードを使う。
    match value {
        // `println!`は通常の値でもリファレンスでも扱うことができる
        // ため、どちらを渡すかは重要ではない。`r`の型は`&i32`になる。
        ref r => println!("Got a reference to a value: {:?}", r),
    }

    // 同様にミュータブルなリファレンスである`&mut 6`を
    // 取得するためには`ref mut`を使用する。
    match mut_value {
        ref mut m => {
            // リファレンスを取得。
            // 値を変更するためにはデリファレンスする必要がある。
            *m += 10;
            println!("We added 10. `mut_value`: {:?}", m);
        },
    }
}
