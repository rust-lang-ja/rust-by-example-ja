<!-- The use of "Associated types" improves the overall readability of code
by moving inner types locally into a trait as *output* types. Syntax
for the `trait` definition is as follows: -->
関連型を使用すると、コンテナ型の中の要素をトレイトの中に*出力型*として書くことで、全体の可読性を上げることができます。トレイトを定義する際の構文は以下のようになります。

```rust
// `A`と`B`は`type`キーワードを用いてトレイト内で宣言されている。
// (注意: この文脈で使用する`type`は型エイリアスを宣言する際の`type`とは
// 異なることに注意しましょう。)
trait Contains {
    type A;
    type B;

    // これらの新しい型をジェネリックに使用するために、構文が
    // アップデートされています。
    fn contains(&self, &Self::A, &Self::B) -> bool;
}
```

<!-- Note that functions that use the `trait` `Contains` are no longer required
to express `A` or `B` at all: -->
`Contains`トレイトを使用する関数において、`A`と`B`を明示する必要がなくなっていることに注目しましょう。

```rust
// 関連型を使用しない場合
fn difference<A, B, C>(container: &C) -> i32 where
    C: Contains<A, B> { ... }

// 使用する場合
fn difference<C: Contains>(container: &C) -> i32 { ... }
```

<!-- Let's rewrite the example from the previous section using associated types: -->
前セクションの例を関連型を使用して書きなおしてみましょう。

{types.play}
