<!-- A bound can also be expressed using a `where` clause immediately
before the opening `{`, rather than at the type's first mention.
Additionally, `where` clauses can apply bounds to arbitrary types,
rather than just to type parameters. -->
トレイト境界は、`{`の直前に`where`句を導入することでも設けることができます。`where`はさらに、型パラメータだけでなく任意の型に対してのみ適用できます。

<!-- Some cases that a `where` clause is useful: -->
`where`句のほうが有効なケースには例えば

<!-- * When specifying generic types and bounds separately is clearer: -->
* ジェネリック型とジェネリック境界に別々に制限を加えたほうが明瞭になる場合
つまり、

```rust
impl <A: TraitB + TraitC, D: TraitE + TraitF> MyTrait<A, D> for YourType {}

// `where`を用いてジェネリック境界を設ける。
impl <A, D> MyTrait<A, D> for YourType where
    A: TraitB + TraitC,
    D: TraitE + TraitF {}
```

<!-- * When using a `where` clause is more expressive than using normal syntax.
The `impl` in this example cannot be directly expressed without a `where` clause: -->
* `where`句の方が通常の構文より表現力が高い場合

があります。

以下の例では`impl`は`where`句なしで直接に表現することができません。

{where.play}

### See also:

[RFC][where], [構造体][struct], [トレイト][trait], [エラーハンドリングの日本語による解説記事][unwrap_option]

[struct]: ../custom_types/structs.html
[trait]: ../trait.html
[where]: https://github.com/rust-lang/rfcs/blob/master/text/0135-where.md
[unwrap_option]: http://qiita.com/tatsuya6502/items/cd41599291e2e5f38a4a
