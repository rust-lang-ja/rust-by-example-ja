// `F`は`Fn`を実装していなくてはならず、`Fn`は引数と返り値を持たない。
// `print`は文字をプリントするだけのクロージャなので、これが正しい。
fn apply<F>(f: F) where
    F: Fn() {
    f()
}

fn main() {
    let x = 7;

    // `x`を無名の構造体に入れ、それに対し`Fn`を実装する。
    // (訳注: ここでは`Fn`は`fn Fn(&self) -> {println!("{}", &self)}`)
    // その構造体を`print`にアサインする。
    let print = || println!("{}", x);

    apply(print);
}
