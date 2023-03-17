<!--
# Derive
-->
# 導出(Derive)

<!--
The compiler is capable of providing basic implementations for some traits via
the `#[derive]` [attribute][attribute]. These traits can still be
manually implemented if a more complex behavior is required.
-->
コンパイラには、`#[derive]`[アトリビュート][attribute]を用いることで型に対して特定のトレイトの標準的な実装を提供する機能があります。より複雑なことを行わせたい場合には、同名のトレイトを手動で実装することも可能です。

<!--
The following is a list of derivable traits:
* Comparison traits:
  [`Eq`][eq], [`PartialEq`][partial-eq], [`Ord`][ord], [`PartialOrd`][partial-ord].
* [`Clone`][clone], to create `T` from `&T` via a copy.
* [`Copy`][copy], to give a type 'copy semantics' instead of 'move semantics'.
* [`Hash`][hash], to compute a hash from `&T`.
* [`Default`][default], to create an empty instance of a data type.
* [`Debug`][debug], to format a value using the `{:?}` formatter.
-->
以下はderive可能なトレイトの一覧です。
* 型の比較に関連するトレイト:
  [`Eq`][eq], [`PartialEq`][partial-eq], [`Ord`][ord], [`PartialOrd`][partial-ord]
* [`Clone`][clone], これはコピーによって`&T`から`T`を作成するトレイト
* [`Copy`][copy], to give a type 'copy semantics' instead of 'move semantics'.
* [`Hash`][hash], これは`&T`からハッシュ値を計算するためのトレイト
* [`Default`][default], これは空っぽのインスタンスを作成するためのトレイト
* [`Debug`][debug], これは`{:?}`フォーマッタを利用して値をフォーマットするためのトレイト
 
```rust,editable
// `Centimeters`, a tuple struct that can be compared
// `Centimeters`は比較可能なタプルになる
#[derive(PartialEq, PartialOrd)]
struct Centimeters(f64);

// `Inches`, a tuple struct that can be printed
// `Inches`はプリント可能なタプルになる
#[derive(Debug)]
struct Inches(i32);

impl Inches {
    fn to_centimeters(&self) -> Centimeters {
        let &Inches(inches) = self;

        Centimeters(inches as f64 * 2.54)
    }
}

// `Seconds`, a tuple struct with no additional attributes
// `Seconds`には特にアトリビュートを付け加えない。
struct Seconds(i32);

fn main() {
    let _one_second = Seconds(1);

    // Error: `Seconds` can't be printed; it doesn't implement the `Debug` trait
    // エラー: `Seconds`はプリントできない。これは`Debug`トレイトを実装していないため
    //println!("One second looks like: {:?}", _one_second);
    // TODO ^ Try uncommenting this line
    // TODO ^ この行をアンコメントしてみましょう。

    // Error: `Seconds` can't be compared; it doesn't implement the `PartialEq` trait
    // エラー: `Seconds`は比較できない。これは`PartialEq`トレイトを実装していないため
    //let _this_is_true = (_one_second == _one_second);
    // TODO ^ Try uncommenting this line
    // TODO ^ この行をアンコメントしてみましょう

    let foot = Inches(12);

    println!("One foot equals {:?}", foot);

    let meter = Centimeters(100.0);

    let cmp =
        if foot.to_centimeters() < meter {
            "smaller"
        } else {
            "bigger"
        };

    println!("One foot is {} than one meter.", cmp);
}
```

<!--
### See also:
-->
### 参照

[`derive`][derive]

[attribute]: ../attribute.md
[eq]: https://doc.rust-lang.org/std/cmp/trait.Eq.html
[partial-eq]: https://doc.rust-lang.org/std/cmp/trait.PartialEq.html
[ord]: https://doc.rust-lang.org/std/cmp/trait.Ord.html
[partial-ord]: https://doc.rust-lang.org/std/cmp/trait.PartialOrd.html
[clone]: https://doc.rust-lang.org/std/clone/trait.Clone.html
[copy]: https://doc.rust-lang.org/core/marker/trait.Copy.html
[hash]: https://doc.rust-lang.org/std/hash/trait.Hash.html
[default]: https://doc.rust-lang.org/std/default/trait.Default.html
[debug]: https://doc.rust-lang.org/std/fmt/trait.Debug.html
[derive]: https://doc.rust-lang.org/reference/attributes.html#derive
