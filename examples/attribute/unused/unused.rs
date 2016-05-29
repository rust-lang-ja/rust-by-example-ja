fn used_function() {}

// `#[allow(dead_code)]`は`dead_code`リントを抑制するアトリビュートです。
#[allow(dead_code)]
fn unused_function() {}

fn noisy_unused_function() {}
// FIXME ^ 警告を抑制するアトリビュートを追加しましょう。

fn main() {
    used_function();
}
