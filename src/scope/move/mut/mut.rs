fn main() {
    let immutable_box = Box::new(5u32);

    println!("immutable_box contains {}", immutable_box);

    // ミュータビリティエラー
    //*immutable_box = 4;

    // boxを*ムーブ*する、同時に所有権とミュータビリティを変更する。
    let mut mutable_box = immutable_box;

    println!("mutable_box contains {}", mutable_box);

    // boxの内容を変更
    *mutable_box = 4;

    println!("mutable_box now contains {}", mutable_box);
}
