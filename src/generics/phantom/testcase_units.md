<!-- A useful method of unit conversions can be examined by implementing `Add`
with a phantom type parameter. The `Add` `trait` is examined below: -->
共通の単位同士を扱う際のチェックのために、`Add`を幽霊型を用いた実装にすると便利な場合があります。その場合`Add`トレイトは以下のようになります。

> 訳注: RHSはRight Hand Side、つまりAdd(`+`)演算時の右辺のことです

``` rust,ignore
// このように定義しておくと、`Self + RHS = Output`であることが保証され、
// かつ、impl時にRHSの型が明示されていない場合、デフォルトでSelfと同じに
// なる。
pub trait Add<RHS = Self> {
    type Output;

    fn add(self, rhs: RHS) -> Self::Output;
}

// `Output`は`T<U>`でなくてはならないので`T<U> + T<U> = T<U>`となる。
impl<U> Add for T<U> {
    type Output = T<U>;
    ...
}
```

以下は全体を示した例です。:

``` rust,editable
use std::ops::Add;
use std::marker::PhantomData;

/// 単位を定義するため、空の列挙型を作成。
#[derive(Debug, Clone, Copy)]
enum Inch {}
#[derive(Debug, Clone, Copy)]
enum Mm {}

/// `Length`は`Unit`という幽霊型パラメータを持つ型
///
/// `f64`ははじめから`Clone`、`Copy`トレイトを持っている。
#[derive(Debug, Clone, Copy)]
struct Length<Unit>(f64,PhantomData<Unit>);

/// `Add`トレイトは加算演算子(`+`)の挙動を定義する。
impl<Unit> Add for Length<Unit> {
     type Output = Length<Unit>;

    // add()は`Length`の新しいインスタンスを返す。
    // Lengthの中の値は合計値になっている。
    fn add(self, rhs: Length<Unit>) -> Length<Unit> {
        // ここでの`+`は`f64`の`Add`実装を呼び出す。
        Length(self.0 + rhs.0, PhantomData)
    }
}

fn main() {
    // `one_foot`が幽霊型`Inch`を持つように明示する。
    let one_foot:  Length<Inch> = Length(12.0, PhantomData);
    // `one_meter`が幽霊型`Mm`を持つように明示する。
    let one_meter: Length<Mm>   = Length(1000.0, PhantomData);

    // 以下の`+`は上で定義した`Length<Unit>`用の`add()`メソッドを呼び出す。
    //
    // `Length`は`Copy`トレイトを持っているため、`add()`は`one_foot`及び`one_meter`
    // を消費する代わりにそのコピーを作り、`self`、`rhs`として扱う。
    let two_feet = one_foot + one_foot;
    let two_meters = one_meter + one_meter;

    // 加算が問題なく実行されていることを確認
    println!("one foot + one_foot = {:?}", two_feet);
    println!("one meter + one_meter = {:?}", two_meters);

    // 異なる単位間の加算は意味を成さないので、
    // 以下はきちんとコンパイルエラーになる。
    // コンパイルエラー: タイプミスマッチ
    //let one_feter = one_foot + one_meter;
}


```

### See also:

[借用(`&`)][Borrow], [トレイトバウンド][bound], [列挙型][enum], [impl & self],
[演算子のオーバーロード][Overloading], [参照][ref], [トレイト (`X for Y`)][trait], [タプル][Tuple].

[Borrow]: ../scope/borrow.html
[bound]: ../trait/bounds.html
[enum]: ../custom_types/enum.html
[impl & self]: ../fn/methods.html
[Overloading]: ../trait/ops.html
[ref]: ../scope/borrow/ref.html
[trait]: ../trait.html
[Tuple]: ../custom_types/structs.html
[std::marker::PhantomData]: https://doc.rust-lang.org/std/marker/struct.PhantomData.html
