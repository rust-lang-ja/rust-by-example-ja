fn main() {
    // イテレータは要素を収集してベクタにすることができる。
    let collected_iterator: Vec<i32> = (0..10).collect();
    println!("Collected (0..10) into: {:?}", collected_iterator);

    // ベクタの初期化には`vec!`マクロが使用できる。
    let mut xs = vec![1i32, 2, 3];
    println!("Initial vector: {:?}", xs);

    // 新しい要素をベクタの最後に挿入することができる。
    println!("Push 4 into the vector");
    xs.push(4);
    println!("Vector: {:?}", xs);

    // エラー！イミュータブルなベクタは成長できない
    collected_iterator.push(0);
    // FIXME ^ この行をコメントアウトしましょう。

    // `len`メソッドは現在のベクタのサイズを返す。
    println!("Vector size: {}", xs.len());

    // 鍵括弧(`[]`)を用いてインデックスによる要素へのアクセスができる
    // (インデックスは0から開始する)
    println!("Second element: {}", xs[1]);

    // `pop`はベクタの最後の要素を削除すると同時に返す。
    println!("Pop last element: {:?}", xs.pop());

    // 不正なインデックスアクセスはpanicを引き起こします。
    println!("Fourth element: {}", xs[3]);
}
