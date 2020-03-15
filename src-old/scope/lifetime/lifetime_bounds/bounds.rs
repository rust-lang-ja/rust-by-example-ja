use std::fmt::Debug; // ライフタイムを紐付けるトレイト

#[derive(Debug)]
struct Ref<'a, T: 'a>(&'a T);
// `Ref`は`'a`というライフタイムを持つジェネリック型`T`に対する参照を持ち、
// `T`の値*に対する参照*は必ず`'a`よりも長生きでなくてはならない。
// さらに、`Ref`のライフタイムは`'a`を超えてはならない。

// `Debug`トレイトを利用してプリントを行うジェネリック関数
fn print<T>(t: T) where
    T: Debug {
    println!("`print`: t is {:?}", t);
}

// `Debug`を実装している`T`への参照を取る。`T`への*参照*は
// 必ず`'a`よりも長生きでなくてはならない。さらに、`'a`は
// 関数自体よりも長生きでなくてはならない。
fn print_ref<'a, T>(t: &'a T) where
    T: Debug + 'a {
    println!("`print_ref`: t is {:?}", t);
}

fn main() {
    let x = 7;
    let ref_x = Ref(&x);

    print_ref(&ref_x);
    print(ref_x);
}
