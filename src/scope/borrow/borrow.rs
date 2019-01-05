// この関数はボックスの所有権を奪い、破棄する。
fn eat_box(boxed_int: Box<i32>) {
    println!("Destroying box that contains {}", boxed_int);
}

// この関数はi32を借用する
fn borrow_box(borrowed_int: &i32) {
    println!("This int is: {}", borrowed_int);
}

fn main() {
    // ボックス化された整数を作成
    let boxed_int = Box::new(5);

    // Boxの中身を借用。所有権を奪うわけではないため、
    // 直後にもう一度借用できる。
    borrow_box(&boxed_int);
    borrow_box(&boxed_int);

    {
        // ボックス内の要素に対する参照を取得
        let _ref_to_int: &i32 = &boxed_int;

        // エラー!
        // ボックス内の要素が借用されているため、`boxed_int`を破棄する
        // ことはできない。
        eat_box(boxed_int);
        // FIXME ^ この行をコメントアウトしましょう。

        // ここで`_ref_to_int`はスコープを抜け、借用もなくなります。
    }

    // ここでようやく、`eat_box`は所有権を移譲し、破棄することができます。
    eat_box(boxed_int);
}
