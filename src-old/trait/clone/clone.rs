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
