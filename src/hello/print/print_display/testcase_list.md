<!--
# Testcase: List
-->
# テストケース: リスト

<!--
Implementing `fmt::Display` for a structure where the elements must each be
handled sequentially is tricky. The problem is that each `write!` generates a
`fmt::Result`. Proper handling of this requires dealing with *all* the
results. Rust provides the `?` operator for exactly this purpose.
-->
構造体のそれぞれの要素を別々に扱う`fmt::Display`を実装するのはトリッキーです。というのも、それぞれの`write!`が別々の`fmt::Result`を生成するためです。適切に処理するためには *すべての* resultに対して処理を書かなくてはなりません。このような場合は`?`演算子を使用するのが適当です。

<!--
Using `?` on `write!` looks like this:
-->
以下のように`?`を`write!`に対して使用します。

```rust,ignore
// Try `write!` to see if it errors. If it errors, return
// the error. Otherwise continue.
// `write!`を実行し、エラーが生じた場合はerrorを返す。そうでなければ実行を継続する。
write!(f, "{}", value)?;
```

<!--
Alternatively, you can also use the `try!` macro, which works the same way. 
This is a bit more verbose and no longer recommended, but you may still see it in
older Rust code. Using `try!` looks like this:
-->
代わりに、同じよう働く`try!`マクロを使うこともできます。
この書き方はやや冗長でもはや推奨されていませんが、古いRustのコードで出会うかもしれません。
`try!`を使うとこのようになります：

```rust,ignore
try!(write!(f, "{}", value));
```

<!--
With `?` available, implementing `fmt::Display` for a `Vec` is
straightforward:
-->
`?`を使用できれば、`Vec`用の`fmt::Display`はより簡単に実装できます。

```rust,editable
use std::fmt; // Import the `fmt` module.

// Define a structure named `List` containing a `Vec`.
// `Vec`を含む`List`という名の構造体を定義
struct List(Vec<i32>);

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Extract the value using tuple indexing,
        // and create a reference to `vec`.
        let vec = &self.0;

        write!(f, "[")?;

        // Iterate over `v` in `vec` while enumerating the iteration
        // count in `count`.
        // `v`を介して`vec`をイテレーションし、同時にカウントを
        // `enumerate`で取得する
        for (count, v) in vec.iter().enumerate() {
            // For every element except the first, add a comma.
            // Use the ? operator to return on errors.
            if count != 0 { write!(f, ", ")?; }
            write!(f, "{}", v)?;
        }

        // Close the opened bracket and return a fmt::Result value.
        // 開きっぱなしのブラケットを閉じて、`fmt::Result`の値を返す。
        write!(f, "]")
    }
}

fn main() {
    let v = List(vec![1, 2, 3]);
    println!("{}", v);
}
```

<!--
### Activity
-->
### 演習

<!--
Try changing the program so that the index of each element in the vector is also
printed. The new output should look like this:
-->
上記のプログラムを変更して、ベクタの各要素のインデックスも表示するようにしてみましょう。変更後の出力は次のようになります。

```rust,ignore
[0: 1, 1: 2, 2: 3]
```

<!--
### See also:
-->
### 参照

<!--
[`for`][for], [`ref`][ref], [`Result`][result], [`struct`][struct],
[`?`][q_mark], and [`vec!`][vec]
-->
[`for`][for], [`ref`][ref], [`Result`][result], [構造体][struct],
[`?`][q_mark], [`vec!`][vec]

[for]: ../../../flow_control/for.md
[result]: ../../../std/result.md
[ref]: ../../../scope/borrow/ref.md
[struct]: ../../../custom_types/structs.md
[q_mark]: ../../../std/result/question_mark.md
[vec]: ../../../std/vec.md
