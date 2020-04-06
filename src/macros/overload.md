<!--
# Overload
-->
# オーバーロード

<!--
Macros can be overloaded to accept different combinations of arguments. 
In that regard, `macro_rules!` can work similarly to a match block:
-->
マクロは異なる引数の組み合わせを取るようにオーバーロードすることができるため、`macro_rules!`はマッチと似たような使い方をすることができます。

```rust,editable
// `test!` will compare `$left` and `$right`
// in different ways depending on how you invoke it:
// `test!`は`$left`と`$right`を異なる呼び出し方に応じて
// 比較する。
macro_rules! test {
    // Arguments don't need to be separated by a comma.
    // Any template can be used!
    // 引数はカンマでくぎらなくてもよい
    // テンプレートの形態は自由！
    ($left:expr; and $right:expr) => {
        println!("{:?} and {:?} is {:?}",
                 stringify!($left),
                 stringify!($right),
                 $left && $right)
    };
    // ^ each arm must end with a semicolon.
    // それぞれの`=>`節はセミコロンで終わる必要がある。
    ($left:expr; or $right:expr) => {
        println!("{:?} or {:?} is {:?}",
                 stringify!($left),
                 stringify!($right),
                 $left || $right)
    };
}

fn main() {
    test!(1i32 + 1 == 2i32; and 2i32 * 2 == 4i32);
    test!(true; or false);
}
```
