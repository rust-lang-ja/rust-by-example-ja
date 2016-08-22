<!-- A useful method of unit conversions can be examined by implementing `Add`
with a phantom type parameter. The `Add` `trait` is examined below: -->
共通の単位同士を扱う際のチェックのために、`Add`を幽霊型を用いた実装にすると便利な場合があります。その場合`Add`トレイトは以下のようになります。

> 訳注: RHSはRight Hand Side、つまりAdd(`+`)演算時の右辺のことです

```rust
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

{units.play}

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
