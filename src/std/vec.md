<!-- Vectors are re-sizable arrays. Like slices, their size is not known at compile
time, but they can grow or shrink at any time. A vector is represented using
3 words: a pointer to the data, its length, and its capacity. The capacity
indicates how much memory is reserved for the vector. The vector can grow as
long as the length is smaller than the capacity. When this threshold needs to
be surpassed, the vector is reallocated with a larger capacity. -->
「ベクタ」はサイズを変更可能な配列です。スライスと同様、そのサイズはコンパイル時には不定ですが、いつでも要素を追加したり削除したりすることができます。ベクタは3つの要素で、その特徴が完全に決まります。

1. データへのポインタ
2. 長さ
3. 容量 ... あらかじめメモリ上にベクタのために確保された領域

ベクタはその容量を超えない限りにおいて長くしていくことができます。超えた場合には、より大きな容量を持つように割り当てなおされます。


``` rust,editable,ignore,mdbook-runnable
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

```

<!-- More `Vec` methods can be found under the
[std::vec][vec] module -->
`Vec`型のメソッドの一覧は[std::vec][vec]モジュールを見てください。

[vec]: http://doc.rust-lang.org/std/vec/
