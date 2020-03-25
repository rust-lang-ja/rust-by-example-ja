<!--
# Borrowing
-->
# 借用

<!--
Most of the time, we'd like to access data without taking ownership over
it. To accomplish this, Rust uses a *borrowing* mechanism. Instead of
passing objects by value (`T`), objects can be passed by reference (`&T`).
-->
実際には、データの所有権を完全に受け渡すことなく一時的にアクセスしたいという場合がほとんどです。そのために、Rustでは*借用(`borrowing`)*という仕組みを用います。値そのもの(`T`)を受け渡すのではなく、そのリファレンス(`&T`)を渡すのです。

<!--
The compiler statically guarantees (via its borrow checker) that references 
*always* point to valid objects. That is, while references to an object
exist, the object cannot be destroyed.
-->
コンパイラは借用チェッカを用いてリファレンスが*常に*有効なオブジェクトへの参照であることを、コンパイル時に保証します。つまり、あるオブジェクトへのリファレンスが存在しているならば、そのオブジェクトを破壊することはできないということです。

```rust,editable,ignore,mdbook-runnable
// This function takes ownership of a box and destroys it
// この関数はボックスの所有権を奪い、破棄する。
fn eat_box_i32(boxed_i32: Box<i32>) {
    println!("Destroying box that contains {}", boxed_i32);
}

// This function borrows an i32
// この関数はi32を借用する
fn borrow_i32(borrowed_i32: &i32) {
    println!("This int is: {}", borrowed_i32);
}

fn main() {
    // Create a boxed i32, and a stacked i32
    // ボックス化された整数を作成
    let boxed_i32 = Box::new(5_i32);
    let stacked_i32 = 6_i32;

    // Borrow the contents of the box. Ownership is not taken,
    // so the contents can be borrowed again.
    // Boxの中身を借用。所有権を奪うわけではないため、
    // 直後にもう一度借用できる。
    borrow_i32(&boxed_i32);
    borrow_i32(&stacked_i32);

    {
        // Take a reference to the data contained inside the box
        // ボックス内の要素に対する参照を取得
        let _ref_to_i32: &i32 = &boxed_i32;

        // Error!
        // Can't destroy `boxed_i32` while the inner value is borrowed later in scope.
        // エラー!
        // ボックス内の要素が借用されているため、`boxed_int`を破棄する
        // ことはできない。
        eat_box_i32(boxed_i32);
        // FIXME ^ Comment out this line
        // FIXME ^ この行をコメントアウトしましょう。

        // Attempt to borrow `_ref_to_i32` after inner value is destroyed
        borrow_i32(_ref_to_i32);
        // `_ref_to_i32` goes out of scope and is no longer borrowed.
        // ここで`_ref_to_int`はスコープを抜け、借用もなくなります。
    }

    // `boxed_i32` can now give up ownership to `eat_box` and be destroyed
    // ここでようやく、`eat_box`は所有権を移譲し、破棄することができます。
    eat_box_i32(boxed_i32);
}
```
