// `test!`は`$left`と`$right`を異なる呼び出し方に応じて
// 比較する。
macro_rules! test {
    // 引数はカンマでくぎらなくてもよい
    // テンプレートの形態は自由！
    ($left:expr; and $right:expr) => (
        println!("{:?} and {:?} is {:?}",
                 stringify!($left),
                 stringify!($right),
                 $left && $right)
    );
    // それぞれの`=>`節はセミコロンで終わる必要がある。
    ($left:expr; or $right:expr) => (
        println!("{:?} or {:?} is {:?}",
                 stringify!($left),
                 stringify!($right),
                 $left || $right)
    );
}

fn main() {
    test!(1i32 + 1 == 2i32; and 2i32 * 2 == 4i32);
    test!(true; or false);
}
