<!-- The compiler is capable of providing basic implementations for some traits via
the `#[derive]` [attribute][attribute]. These traits can still be
manually implemented if a more complex behavior is required. -->
コンパイラには、`[#derive]`[アトリビュート][attribute]を用いることで型に対して特定のトレイトの標準的な実装を提供する機能があります。より複雑なことを行わせたい場合には、同名のトレイトを手動で実装することも可能です。

<!-- The following is a list of the "derivable" traits: -->
以下はderive可能なトレイトの一覧です。

<!-- * Comparison traits:
  [`Eq`][eq],
  [`PartialEq`][partial-eq],
  [`Ord`][ord],
  [`PartialOrd`][partial-ord]
* [`Clone`][clone],
  to create `T` from `&T` via a copy.
* [`Hash`][hash], to
  compute a hash from `&T`.
* [`Default`][default],
  to create an empty instance of a data type.
* `Zero`, to
  create a zero instance of a numeric data type.
* [`Debug`][debug], to
  format a value using the `{:?}` formatter. -->
* 型の比較に関連するトレイト:
  [`Eq`][eq],
  [`PartialEq`][partial-eq],
  [`Ord`][ord],
  [`PartialOrd`][partial-ord]
* [`Clone`][clone], これは
  コピーによって`&T`から`T`を作成するトレイト
* [`Hash`][hash], これは
  `&T`からハッシュ値を計算するためのトレイト
* [`Default`][default], これは
  空っぽのインスタンスを作成するためのトレイト
* `Zero`, これは
  数値型に対してゼロに相当するインスタンスを作成するためのトレイト
* [`Debug`][debug], これは
  `{:?}`フォーマッタを利用して値をフォーマットするためのトレイト


``` rust,editable
// `Centimeters`は比較可能なタプルになる
#[derive(PartialEq, PartialOrd)]
struct Centimeters(f64);

// `Inches`はプリント可能なタプルになる
#[derive(Debug)]
struct Inches(i32);

impl Inches {
    fn to_centimeters(&self) -> Centimeters {
        let &Inches(inches) = self;

        Centimeters(inches as f64 * 2.54)
    }
}

// `Seconds`には特にアトリビュートを付け加えない。
struct Seconds(i32);

fn main() {
    let _one_second = Seconds(1);

    // エラー: `Seconds`はプリントできない。これは`Debug`トレイトを実装していないため
    //println!("One second looks like: {:?}", _one_second);
    // TODO ^ この行をアンコメントしてみましょう。

    // エラー: `Seconds`は比較できない。これは`PartialEq`トレイトを実装していないため
    //let _this_is_true = (_one_second == _one_second);
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

[attribute]: ../attribute.html
[eq]: http://doc.rust-lang.org/std/cmp/trait.Eq.html
[partial-eq]: http://doc.rust-lang.org/std/cmp/trait.PartialEq.html
[ord]: http://doc.rust-lang.org/std/cmp/trait.Ord.html
[partial-ord]: http://doc.rust-lang.org/std/cmp/trait.PartialOrd.html
[clone]: http://doc.rust-lang.org/std/clone/trait.Clone.html
[hash]: http://doc.rust-lang.org/std/hash/trait.Hash.html
[default]: http://doc.rust-lang.org/std/default/trait.Default.html
[debug]: http://doc.rust-lang.org/std/fmt/trait.Debug.html
