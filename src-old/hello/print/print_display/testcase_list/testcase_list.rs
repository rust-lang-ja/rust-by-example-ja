use std::fmt; // Import the `fmt` module.

// `Vec`を含む`List`という名の構造体を定義
struct List(Vec<i32>);

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // `self`をデリファレンスし、 デストラクトすることで
        // `vec`へのリファレンスを作成する
        let List(ref vec) = *self;

        try!(write!(f, "["));

        // `v`を介して`vec`をイテレーションし、同時にカウントを
        // `enumerate`で取得する
        for (count, v) in vec.iter().enumerate() {
            // 初回を除き、`write!`を呼ぶ前にカンマを加える。
            // エラーを返すために`try!`を使用する
            if count != 0 { try!(write!(f, ", ")); }
            try!(write!(f, "{}", v));
        }

        // 開きっぱなしのブラケットを閉じて、`fmt::Result`の値を返す。
        write!(f, "]")
    }
}

fn main() {
    let v = List(vec![1, 2, 3]);
    println!("{}", v);
}
