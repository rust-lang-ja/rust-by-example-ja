<!--
# Testcase: empty bounds
-->
# テストケース: 空トレイト

<!--
A consequence of how bounds work is that even if a `trait` doesn't
include any functionality, you can still use it as a bound. `Eq` and
`Ord` are examples of such `trait`s from the `std` library.
-->
トレイト境界の仕組みから、「トレイトがなにも機能を持っていなくとも境界条件として使用できることには変わりはない」という帰結がもたらされます。`Eq`と`Ord`は`std`ライブラリにおけるそのような例です。

```rust,editable
struct Cardinal;
struct BlueJay;
struct Turkey;

trait Red {}
trait Blue {}

impl Red for Cardinal {}
impl Blue for BlueJay {}

// These functions are only valid for types which implement these
// traits. The fact that the traits are empty is irrelevant.
// 以下の関数はトレイト境界を設けているが、そのトレイトが空である
// か否かとは関係がない。
fn red<T: Red>(_: &T)   -> &'static str { "red" }
fn blue<T: Blue>(_: &T) -> &'static str { "blue" }

fn main() {
    // 訳注: 以下は全て鳥の名前
    // 猩々紅冠鳥
    let cardinal = Cardinal;
    // アオカケス
    let blue_jay = BlueJay;
    // 七面鳥
    let _turkey   = Turkey;

    // `red()` won't work on a blue jay nor vice versa
    // because of the bounds.
    // トレイト境界のため、`red`は`blue_jay`に対しては使用できない。
    // `blue`と`Cardinal`も同様、
    println!("A cardinal is {}", red(&cardinal));
    println!("A blue jay is {}", blue(&blue_jay));
    //println!("A turkey is {}", red(&_turkey));
    // ^ TODO: Try uncommenting this line.
    // ^ TODO: この行をアンコメントしてみましょう。
}
```

<!--
### See also:
-->
### 参照

<!--
[`std::cmp::Eq`][eq], [`std::cmp::Ord`s][ord], and [`trait`s][traits]
-->
[`std::cmp::Eq`][eq], [`std::cmp::Ord`s][ord], [トレイト][traits]

[eq]: https://doc.rust-lang.org/std/cmp/trait.Eq.html
[ord]: https://doc.rust-lang.org/std/cmp/trait.Ord.html
[traits]: ../../trait.md
