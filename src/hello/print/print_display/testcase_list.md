<!--- Implementing `fmt::Display` for a structure where the elements must each be --->
<!--- handled sequentially is tricky. The problem is that each `write!` generates a --->
<!--- `fmt::Result`. Proper handling of this requires dealing with *all* the --->
<!--- results. Rust provides the `try!` macro for exactly this purpose. --->
構造体のそれぞれの要素を別々に扱う`fmt::Display`を実装するのはトリッキーです。というのも、それぞれの`write!`が別々の`fmt::Result`を生成するためです。適切に処理するためには*すべての*resultに対して処理を書かなくてはなりません。このような場合は`try!`マクロを使用するのが適当です。


<!--- Using `try!` on `write!` looks like this: --->
以下のように`try!`を`write!`に対して使用します。

``` rust,ignore
// `write!`を実行し、エラーが生じた場合はerrorを返す。そうでなければ実行を継続する。
try!(write!(f, "{}", value));
```

<!--- With `try!` available, implementing `fmt::Display` for a `Vec` is --->
<!--- straightforward: --->
`try!`を使用できれば、`Vec`用の`fmt::Display`はより簡単に実装できます。

``` rust,editable
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

```

### See also

[`for`][for], [`ref`][ref], [`Result`][result], [構造体][struct],
[`try!`][try], and [`vec!`][vec]

[for]: ../../../flow_control/for.html
[result]: ../../../std/result.html
[ref]: ../../../scope/borrow/ref.html
[struct]: ../../../custom_types/structs.html
[try]: ../../../std/result/try.html
[vec]: ../../../std/vec.html
