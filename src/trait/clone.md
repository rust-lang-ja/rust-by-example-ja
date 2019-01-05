<!-- When dealing with resources, the default behavior is to transfer them during
assignments or function calls. However, sometimes we need to make a
copy of the resource as well. -->
メモリ上の資源を扱う際、変数束縛や関数呼び出しを介して移動させるのがデフォルトの挙動です。しかしながら、場合によっては資源のコピーを作るのが適切なこともあります。

<!-- The [`Clone`][clone] trait helps us do exactly this. Most commonly, we can
use the `.clone()` method defined by the `Clone` trait. -->
[`Clone`][clone]トレイトはまさにこのためにあります。普通は`Clone`トレイトで定義されている`.clone()`を用います。

``` rust,editable
// いかなる資源も持たない構造体
#[derive(Debug, Clone, Copy)]
struct Nil;

// `Clone`トレイトを実装する型の変数を資源として持つタプル
#[derive(Clone, Debug)]
struct Pair(Box<i32>, Box<i32>);

fn main() {
    // `Nil`のインスタンスを作成
    let nil = Nil;
    // `Nil`をコピー、移動させる資源は存在しない
    let copied_nil = nil;

    // いずれの`Nil`も独立に使用できる。
    println!("original: {:?}", nil);
    println!("copy: {:?}", copied_nil);

    // `Pair`のインスタンスを作成
    let pair = Pair(Box::new(1), Box::new(2));
    println!("original: {:?}", pair);

    // `pair`を`moved_pair`にコピー、資源は移動(`move`)する。
    let moved_pair = pair;
    println!("copy: {:?}", moved_pair);

    // エラー! `pair`は資源を失っている。
    //println!("original: {:?}", pair);
    // TODO ^ この行をアンコメントしてみましょう。

    // `moved_pair`を`cloned_pair`にクローンする。(資源もクローンされる。)
    let cloned_pair = moved_pair.clone();
    // std::mem::dropを用いて元のpairをドロップする
    drop(moved_pair);

    // エラー! `moved_pair`はドロップされている。
    //println!("copy: {:?}", moved_pair);
    // TODO ^ この行をアンコメントしてみましょう。

    // .clone()した値はまだ使用可能！
    println!("clone: {:?}", cloned_pair);
}

```

[clone]: https://doc.rust-lang.org/std/clone/trait.Clone.html
