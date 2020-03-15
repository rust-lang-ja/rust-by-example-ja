#[derive(Clone, Copy)]
struct Point { x: i32, y: i32 }

fn main() {
    let c = 'Q';

    // 左辺に`ref`をつけることによる借用と、右辺に`&`をつけることによる借用は等価
    let ref ref_c1 = c;
    let ref_c2 = &c;

    println!("ref_c1 equals ref_c2: {}", *ref_c1 == *ref_c2);

    let point = Point { x: 0, y: 0 };

    // `ref`は構造体をデストラクトする際にも有用
    let _copy_of_x = {
        // `ref_to_x`は`point`の`x`フィールドへの参照
        let Point { x: ref ref_to_x, y: _ } = point;

        // `point`の`x`フィールドへのコピーを返す。
        *ref_to_x
    };

    // `point`へのミュータブルなコピー
    let mut mutable_point = point;

    {
        // `ref`は`mut`とともに使い、ミュータブルな参照を取ることもできる。
        let Point { x: _, y: ref mut mut_ref_to_y } = mutable_point;

        // `mutable_point`の`y`というミュータブルなフィールドの値を変更する。
        *mut_ref_to_y = 1;
    }

    println!("point is ({}, {})", point.x, point.y);
    println!("mutable_point is ({}, {})", mutable_point.x, mutable_point.y);

    // ポインタを含むミュータブルなタプル
    let mut mutable_tuple = (Box::new(5u32), 3u32);

    {
        // `mutable_ tuple`をデストラクトして、`last`の値を変更
        let (_, ref mut last) = mutable_tuple;
        *last = 2u32;
    }

    println!("tuple is {:?}", mutable_tuple);
}
