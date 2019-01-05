<!-- Of course `trait`s can also be generic. Here we define one which reimplements
the `Drop` `trait` as a generic method to `drop` itself and an input. -->
もちろんトレイトもジェネリクスを活用することができます。ここでは`Drop`トレイトをジェネリックメソッドとして再実装し、自身と引数として受け取った値の両方を`drop`するようなメソッドにします。

``` rust,editable
// コピー不可な型
// 訳注: `clone()`メソッドを用いないかぎり、値のコピーではなくムーブが起きる型
struct Empty;
struct Null;

// ジェネリック型 `T`に対するトレイト
trait DoubleDrop<T> {
    // `self`に加えてもう一つジェネリック型を受け取り、
    // 何もしないメソッドのシグネチャを定義
    fn double_drop(self, _: T);
}

// `U`を`self`として、`T`をもう一つの引数として受け取る`DoubleDrop<T>`
// を実装する。`U`,`T`はいずれもジェネリック型
impl<T, U> DoubleDrop<T> for U {
    // このメソッドは2つの引数の所有権を取り、メモリ上から開放する。
    fn double_drop(self, _: T) {}
}

fn main() {
    let empty = Empty;
    let null  = Null;

    // `empty`と`null`を開放
    empty.double_drop(null);

    //empty;
    //null;
    // ^ TODO: これらの行をアンコメントしてみましょう。
}

```

### See also:

[`Drop`][Drop], [構造体(`struct`)][structs], [トレイト(`trait`)][traits]

[Drop]: http://doc.rust-lang.org/std/ops/trait.Drop.html
[structs]: /custom_types/structs.html
[traits]: /trait.html
