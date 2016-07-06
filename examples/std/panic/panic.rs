// 整数の除法(/)の再実装
fn division(dividend: i32, divisor: i32) -> i32 {
    if divisor == 0 {
        // ゼロによる除算はパニックを引き起こす
        panic!("division by zero");
    } else {
        dividend / divisor
    }
}

// `main`のタスク
fn main() {
    // ヒープ上の整数
    let _x = Box::new(0i32);

    // This operation will trigger a task failure
    // このオペレーションはタスクの失敗を引き起こす
    division(3, 0);

    println!("This point won't be reached!");

    // `_x`はここに到達する前に破棄される。
}
