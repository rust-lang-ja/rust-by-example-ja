<!-- A consequence of how bounds work is that even if a `trait` doesn't
include any functionality, you can still use it as a bound. `Eq` and
`Ord` are examples of such `trait`s from the `std` library. -->
トレイト境界の仕組みから、「トレイトがなにも機能を持っていなくとも境界条件として使用できることには変わりはない」という帰結がもたらされます。`Eq`と`Ord`は`std`ライブラリにおけるそのような例です。

``` rust,editable
struct Cardinal;
struct BlueJay;
struct Turkey;

trait Red {}
trait Blue {}

impl Red for Cardinal {}
impl Blue for BlueJay {}

// 以下の関数はトレイト境界を設けているが、そのトレイトが空である
// か否かとは関係がない。
fn red<T: Red>(_: &T)   -> &'static str { "red" }
fn blue<T: Blue>(_: &T) -> &'static str { "blue" }

fn main() {
    // 訳注: 以下は全て鳥の名前
    let cardinal = Cardinal; // 猩々紅冠鳥
    let blue_jay = BlueJay; // アオカケス
    let _turkey   = Turkey; // 七面鳥

    // トレイト境界のため、`red`は`blue_jay`に対しては使用できない。
    // `blue`と`Cardinal`も同様、
    println!("A cardinal is {}", red(&cardinal));
    println!("A blue jay is {}", blue(&blue_jay));
    //println!("A turkey is {}", red(&_turkey));
    // ^ TODO: この行をアンコメントしてみましょう。
}

```

### See also:

[`std::cmp::Eq`][eq], [`std::cmp::Ord`s][ord], [トレイト][traits]

[eq]: http://doc.rust-lang.org/std/cmp/trait.Eq.html
[ord]: http://doc.rust-lang.org/std/cmp/trait.Ord.html
[traits]: ../trait.html
