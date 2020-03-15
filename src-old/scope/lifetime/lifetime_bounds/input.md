<!-- Just like generic types can be bounded, lifetimes (themselves generic)
use bounds as well. The `:` character has a slightly different meaning here,
but `+` is the same. Note how the following read: -->
ジェネリック型に境界(bound)を与え、特定のトレイトを実装していることを保証できるのと同様、ライフタイム(それ自身ジェネリック型)にも境界を与えることができます。`:`は、ここでは多少異なる意味を持ちますが`+`は同じです。以下の構文の意味をチェックしてください。

<!-- 1. `T: 'a`: *All* references in `T` must outlive lifetime `'a`.
2. `T: Trait + 'a`: Type `T` must implement trait `Trait` and *all* references
in `T` must outlive `'a`. -->
1. `T: 'a`: `T`内の*全ての*参照は`'a`よりも長生きでなくてはならない
2. `T: Trait + 'a`: 上に加えて`T`は`Trait`という名のトレイトを実装してなくてはならない。

<!-- The example below shows the above syntax in action: -->
上記の構文を実際に動く例で見ていきましょう。

{bounds.play}

### See also:

[ジェネリクス][generics], [ジェネリック境界][bounds],
[境界が複数の場合][multibounds]

[generics]: /generics.html
[bounds]: /generics/bounds.html
[multibounds]: /generics/multi_bounds.html
