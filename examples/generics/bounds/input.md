<!-- When working with generics, the type parameters often must use traits as *bounds* to
stipulate what functionality a type implements. For example, the following
example uses the trait `Display` to print and so it requires `T` to be bound
by `Display`; that is, `T` *must* implement `Display`. -->
ジェネリックプログラミングをしていると、型パラメータが特定の機能を持っていることを規定するため、トレイトに境界(`bound`)を設ける必要があることがよくあります。例えば、以下の例では、引数の`Display`トレイトを用いてプリントを行うため、`T`が`Display`を持っていることを規定しています。つまり、「`T`は`Display`を実装*していなくてはならない*」という意味です。

```rust
// `Display`トレイトを実装している`T`を引数として取る
// `printer`という関数を定義。
fn printer<T: Display>(t: T) {
    println!("{}", t);
}
```

<!-- Bounding restricts the generic to types that conform to the bounds. That is: -->
境界は、ジェネリクスを全ての型ではなく、一定条件を満たす型に対してのみ適用するためにあります。つまり

> 訳注: `<T: Display>`は`<T>`の部分集合であることを意識すると、「境界」という言葉の意味がしっくり来ると思います。

```rust
struct S<T: Display>(T);

// エラー! `Vec<T>`は`Display`を実装していないため、この特殊化
// は失敗します。
let s = S(vec![1]);
```

<!-- Another effect of bounding is that generic instances are allowed to access the
[methods] of traits specified in the bounds. For example: -->
境界のもう一つの効果は、ジェネリック型のインスタンスが、境界条件となるトレイトの[メソッド][methods]にアクセスすることができるようになる点です。以下がその例です。

{bounds.play}

<!-- As an additional note, [`where`][where] clauses can also be used to apply bounds in
some cases to be more expressive. -->
付け加えておくと、[`where`][where]句を用いて境界を適用することもできます。場合によってはこちらの記法を使用したほうが読みやすくなる場合もあります。

### See also:

[`std::fmt`][fmt], [構造体(`struct`)][structs], [トレイト][traits]

[fmt]: ../hello/print.html
[methods]: ../fn/methods.html
[structs]: ../custom_types/structs.html
[traits]: ../trait.html
[where]: ../generics/where.html
