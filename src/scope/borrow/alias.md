<!--
# Aliasing
-->
# エイリアス

<!--
Data can be immutably borrowed any number of times, but while immutably
borrowed, the original data can't be mutably borrowed. On the other hand, only
*one* mutable borrow is allowed at a time. The original data can be borrowed
again only *after* the mutable reference has been used for the last time.
-->
データは一度にいくつでもイミュータブルに借用することができますが、その間オリジナルのデータをミュータブルに借用することはできません。一方でミュータブルな借用は一度に*一つ*しか借用することができません。オリジナルのデータをもう一度借用できるのはミュータブルな参照が最後に使われた場所より*あとで*なければいけません。

```rust,editable
struct Point { x: i32, y: i32, z: i32 }

fn main() {
    let mut point = Point { x: 0, y: 0, z: 0 };

    let borrowed_point = &point;
    let another_borrow = &point;

    // Data can be accessed via the references and the original owner
    // データは元々の持ち主と参照の両方からアクセスすることができます。
    println!("Point has coordinates: ({}, {}, {})",
                borrowed_point.x, another_borrow.y, point.z);

    // Error! Can't borrow `point` as mutable because it's currently
    // borrowed as immutable.
    // エラー！pointはすでにイミュータブルに借用されているため、
    // ミュータブルに借用することができない。
    // let mutable_borrow = &mut point;
    // TODO ^ Try uncommenting this line

    // The borrowed values are used again here
    println!("Point has coordinates: ({}, {}, {})",
                borrowed_point.x, another_borrow.y, point.z);

    // The immutable references are no longer used for the rest of the code so
    // it is possible to reborrow with a mutable reference.
    let mutable_borrow = &mut point;

    // Change data via mutable reference
    // ミュータブルなリファレンスを介してデータを変更する
    mutable_borrow.x = 5;
    mutable_borrow.y = 2;
    mutable_borrow.z = 1;

    // Error! Can't borrow `point` as immutable because it's currently
    // borrowed as mutable.
    // エラー！`point`はすでにミュータブルに借用されているため、
    // イミュータブルに借用することはできない。
    // let y = &point.y;
    // TODO ^ Try uncommenting this line
    // TODO ^ この行をアンコメントしてみましょう。

    // Error! Can't print because `println!` takes an immutable reference.
    // エラー！`println!`はイミュータブルなリファレンスを取るため、printできません。
    // println!("Point Z coordinate is {}", point.z);
    // TODO ^ Try uncommenting this line
    // TODO ^ この行をアンコメントしてみましょう。

    // Ok! Mutable references can be passed as immutable to `println!`
    println!("Point has coordinates: ({}, {}, {})",
                mutable_borrow.x, mutable_borrow.y, mutable_borrow.z);

    // The mutable reference is no longer used for the rest of the code so it
    // is possible to reborrow
    let new_borrowed_point = &point;
    println!("Point now has coordinates: ({}, {}, {})",
             new_borrowed_point.x, new_borrowed_point.y, new_borrowed_point.z);
}
```
