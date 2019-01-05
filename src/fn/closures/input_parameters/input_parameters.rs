// クロージャを引数に取る関数
fn apply<F>(f: F) where
    // クロージャには引数も返り値もない。
    F: FnOnce() {
    // ^ TODO: ここを`Fn`あるいは`FnMut`に変えてみましょう。

    f()
}

// クロージャを引数に取り、`i32`を返す関数
fn apply_to_3<F>(f: F) -> i32 where
    // このクロージャは引数、返り値ともに`i32`
    F: Fn(i32) -> i32 {

    f(3)
}

fn main() {
    use std::mem;

    let greeting = "hello";
    // コピーではなくmoveが起きる型
    let mut farewell = "goodbye".to_owned();

    // 変数を2つ補足。`greeting`は参照を、
    // `farewell`は値をそれぞれ捕捉する。
    let diary = || {
        // `greeting`は参照なので、`Fn`が必要。
        println!("I said {}.", greeting);

        // `farewell`の値を変更するので、この時点で`FnMut`
        // が必要になる。
        farewell.push_str("!!!");
        println!("Then I screamed {}.", farewell);
        println!("Now I can sleep. zzzzz");

        // `mem::drop`を明示的に呼ぶと`farewell`が値で
        // 捕捉される必要性が発生する。よって`FnOnce`が必要になる。
        mem::drop(farewell);
    };

    // クロージャを適用する関数を実行。
    apply(diary);

    let double = |x| 2 * x;

    println!("3 doubled: {}", apply_to_3(double));
}
