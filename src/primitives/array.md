<!--- An array is a collection of objects of the same type `T`, stored in contiguous --->
<!--- memory. Arrays are created using brackets `[]`, and their size, which is known --->
<!--- at compile time, is part of their type signature `[T; size]`. --->
配列は`T`という単一の型(訳注: [ジェネリック型](https://rust-lang-ja.github.io/the-rust-programming-language-ja/1.6/book/generics.html)でも可)のオブジェクトの集合です。それらのオブジェクトはメモリ上の連続した領域に保存されます。配列は`[]`を用いて生成されます。サイズはコンパイル時には決定されていて、`[T; size]`という形で指定できます。

<!--- Slices are similar to arrays, but their size is not known at compile time. --->
<!--- Instead, a slice is a two-word object, the first word is a pointer to the data, --->
<!--- and the second word is the length of the slice. Slices can be used to borrow a --->
<!--- section of an array, and have the type signature `&[T]`. --->
スライスは配列に似ていますが、コンパイル時にサイズが決定されていません。2つの部分からなり、1つは別のデータへのポインタ、もう1つはスライスの長さです。配列の一部を借用するのに使用され`&[T]`という型シグネチャを持ちます。

``` rust,editable,ignore,mdbook-runnable
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

```
