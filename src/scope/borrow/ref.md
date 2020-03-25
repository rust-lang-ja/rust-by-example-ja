<!--
# The ref pattern
-->
# refパターン

<!--
When doing pattern matching or destructuring via the `let` binding, the `ref`
keyword can be used to take references to the fields of a struct/tuple. The 
example below shows a few instances where this can be useful:
-->
`let`を介してデストラクトやパターンマッチングを行う場合、`ref`キーワードを用いて構造体・タプルのフィールドへのリファレンスを取得することができます。以下の例ではこれが有用になる例を幾つかお見せします。

```rust,editable
#[derive(Clone, Copy)]
struct Point { x: i32, y: i32 }

fn main() {
    let c = 'Q';

    // A `ref` borrow on the left side of an assignment is equivalent to
    // an `&` borrow on the right side.
    // 左辺に`ref`をつけることによる借用と、右辺に`&`をつけることによる借用は等価
    let ref ref_c1 = c;
    let ref_c2 = &c;

    println!("ref_c1 equals ref_c2: {}", *ref_c1 == *ref_c2);

    let point = Point { x: 0, y: 0 };

    // `ref` is also valid when destructuring a struct.
    // `ref`は構造体をデストラクトする際にも有用
    let _copy_of_x = {
        // `ref_to_x` is a reference to the `x` field of `point`.
        // `ref_to_x`は`point`の`x`フィールドへの参照
        let Point { x: ref ref_to_x, y: _ } = point;

        // Return a copy of the `x` field of `point`.
        // `point`の`x`フィールドへのコピーを返す。
        *ref_to_x
    };

    // A mutable copy of `point`
    // `point`へのミュータブルなコピー
    let mut mutable_point = point;

    {
        // `ref` can be paired with `mut` to take mutable references.
        // `ref`は`mut`とともに使い、ミュータブルな参照を取ることもできる。
        let Point { x: _, y: ref mut mut_ref_to_y } = mutable_point;

        // Mutate the `y` field of `mutable_point` via a mutable reference.
        // `mutable_point`の`y`というミュータブルなフィールドの値を変更する。
        *mut_ref_to_y = 1;
    }

    println!("point is ({}, {})", point.x, point.y);
    println!("mutable_point is ({}, {})", mutable_point.x, mutable_point.y);

    // A mutable tuple that includes a pointer
    // ポインタを含むミュータブルなタプル
    let mut mutable_tuple = (Box::new(5u32), 3u32);
    
    {
        // Destructure `mutable_tuple` to change the value of `last`.
        // `mutable_ tuple`をデストラクトして、`last`の値を変更
        let (_, ref mut last) = mutable_tuple;
        *last = 2u32;
    }
    
    println!("tuple is {:?}", mutable_tuple);
}
```
