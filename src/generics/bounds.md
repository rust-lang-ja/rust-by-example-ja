<!-- When working with generics, the type parameters often must use traits as *bounds* to
stipulate what functionality a type implements. For example, the following
example uses the trait `Display` to print and so it requires `T` to be bound
by `Display`; that is, `T` *must* implement `Display`. -->
ジェネリックプログラミングをしていると、型パラメータが特定の機能を持っていることを規定するため、トレイトに境界(`bound`)を設ける必要があることがよくあります。例えば、以下の例では、引数の`Display`トレイトを用いてプリントを行うため、`T`が`Display`を持っていることを規定しています。つまり、「`T`は`Display`を実装*していなくてはならない*」という意味です。

``` rust,ignore
// `Display`トレイトを実装している`T`を引数として取る
// `printer`という関数を定義。
fn printer<T: Display>(t: T) {
    println!("{}", t);
}
```

<!-- Bounding restricts the generic to types that conform to the bounds. That is: -->
境界は、ジェネリクスを全ての型ではなく、一定条件を満たす型に対してのみ適用するためにあります。つまり

> 訳注: `<T: Display>`は`<T>`の部分集合であることを意識すると、「境界」という言葉の意味がしっくり来ると思います。

``` rust,ignore
struct S<T: Display>(T);

// エラー! `Vec<T>`は`Display`を実装していないため、この特殊化
// は失敗します。
let s = S(vec![1]);
```

<!-- Another effect of bounding is that generic instances are allowed to access the
[methods] of traits specified in the bounds. For example: -->
境界のもう一つの効果は、ジェネリック型のインスタンスが、境界条件となるトレイトの[メソッド][methods]にアクセスすることができるようになる点です。以下がその例です。

``` rust,editable
// プリント時のマーカー`{:?}`を実装するトレイト
use std::fmt::Debug;

trait HasArea {
    fn area(&self) -> f64;
}

impl HasArea for Rectangle {
    fn area(&self) -> f64 { self.length * self.height }
}

#[derive(Debug)]
struct Rectangle { length: f64, height: f64 }
#[allow(dead_code)]
struct Triangle  { length: f64, height: f64 }

// ジェネリック型`T`は`Debug`トレイトを実装していなくてはならない。
// その限りにおいて、`T`がどのような具象型であろうとも次の関数は動作する。
fn print_debug<T: Debug>(t: &T) {
    println!("{:?}", t);
}

// 「`T`は`HasArea`を実装していなくてはならない」という境界条件を
// 満たしていれば、`HasArea`の関数`area`にアクセスできる。
fn area<T: HasArea>(t: &T) -> f64 { t.area() }

fn main() {
    let rectangle = Rectangle { length: 3.0, height: 4.0 };
    let _triangle = Triangle  { length: 3.0, height: 4.0 };

    print_debug(&rectangle);
    println!("Area: {}", area(&rectangle));

    //print_debug(&_triangle);
    //println!("Area: {}", area(&_triangle));
    // ^ TODO: これらの行をアンコメントしてみましょう。
    // | Error: `Debug` も `HasArea`もどちらも実装されていません!
}

```

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
