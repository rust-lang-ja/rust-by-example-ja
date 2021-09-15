<!--
# Bounds
-->
# ライフタイム境界

<!--
Just like generic types can be bounded, lifetimes (themselves generic)
use bounds as well. The `:` character has a slightly different meaning here, 
but `+` is the same. Note how the following read:
-->
ジェネリック型に境界(bound)を与え、特定のトレイトを実装していることを保証できるのと同様、ライフタイム（それ自身ジェネリック型）にも境界を与えることができます。`:`は、ここでは多少異なる意味を持ちますが`+`は同じです。以下の構文の意味をチェックしてください。

<!--
1. `T: 'a`: *All* references in `T` must outlive lifetime `'a`.
2. `T: Trait + 'a`: Type `T` must implement trait `Trait` and *all* references
in `T` must outlive `'a`.
-->
1. `T: 'a`: `T`内の *全ての* 参照は`'a`よりも長生きでなくてはならない
2. `T: Trait + 'a`: 上に加えて`T`は`Trait`という名のトレイトを実装してなくてはならない。

<!--
The example below shows the above syntax in action used after keyword `where`:
-->
上記の構文を実際に動く例で見ていきましょう。`where`キーワードの後に注目してください。

```rust,editable
use std::fmt::Debug; // Trait to bound with.
                     // ライフタイムを紐付けるトレイト

#[derive(Debug)]
struct Ref<'a, T: 'a>(&'a T);
// `Ref` contains a reference to a generic type `T` that has
// an unknown lifetime `'a`. `T` is bounded such that any
// *references* in `T` must outlive `'a`. Additionally, the lifetime
// of `Ref` may not exceed `'a`.
// `Ref`は`'a`というライフタイムを持つジェネリック型`T`に対する参照を持ち、
// `T`の値に対する *参照* は必ず`'a`よりも長生きでなくてはならない。
// さらに、`Ref`のライフタイムは`'a`を超えてはならない。

// A generic function which prints using the `Debug` trait.
// `Debug`トレイトを利用してプリントを行うジェネリック関数
fn print<T>(t: T) where
    T: Debug {
    println!("`print`: t is {:?}", t);
}

// Here a reference to `T` is taken where `T` implements
// `Debug` and all *references* in `T` outlive `'a`. In
// addition, `'a` must outlive the function.
// `Debug`を実装している`T`への参照を取る。`T`への *参照* は
// 必ず`'a`よりも長生きでなくてはならない。さらに、`'a`は
// 関数自体よりも長生きでなくてはならない。
fn print_ref<'a, T>(t: &'a T) where
    T: Debug + 'a {
    println!("`print_ref`: t is {:?}", t);
}

fn main() {
    let x = 7;
    let ref_x = Ref(&x);

    print_ref(&ref_x);
    print(ref_x);
}
```

<!--
### See also:
-->
### 参照

<!--
[generics][generics], [bounds in generics][bounds], and 
[multiple bounds in generics][multibounds]
-->
[ジェネリクス][generics], [ジェネリック境界][bounds],
[境界が複数の場合][multibounds]

[generics]: ../../generics.md
[bounds]: ../../generics/bounds.md
[multibounds]: ../../generics/multi_bounds.md
