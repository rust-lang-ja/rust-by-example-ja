// 関数を引数として取り、即座に実行する関数を定義
fn call_function<F: Fn()>(f: F) {
    f()
}

// 引数として渡すための簡単な関数を定義
fn print() {
    println!("I'm a function!")
}

fn main() {
    // 上で定義した`print()`に似たクロージャを定義
    let closure = || println!("I'm a closure!");

    call_function(closure);
    call_function(print);
}
