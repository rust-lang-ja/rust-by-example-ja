use std::mem;

// この関数はスライスを借用する
fn analyze_slice(slice: &[i32]) {
    println!("first element of the slice: {}", slice[0]);
    println!("the slice has {} elements", slice.len());
}

fn main() {
    // 固定長の配列（型シグネチャは冗長なので、なくても可）
    let xs: [i32; 5] = [1, 2, 3, 4, 5];

    // すべての要素を0にする場合
    let ys: [i32; 500] = [0; 500];

    // インデックスは０から
    println!("first element of the array: {}", xs[0]);
    println!("second element of the array: {}", xs[1]);

    // `len`は配列のサイズを返す。
    println!("array size: {}", xs.len());

    // 配列はスタック上に置かれる
    println!("array occupies {} bytes", mem::size_of_val(&xs));

    // 配列は自動的にスライスとして借用される。
    println!("borrow the whole array as a slice");
    analyze_slice(&xs);

    // 配列の一部分だけのスライスを渡す
    println!("borrow a section of the array as a slice");
    analyze_slice(&ys[1 .. 4]);

    // インデックスの範囲が配列のサイズを超えた場合パニックする
    println!("{}", xs[5]);
}
