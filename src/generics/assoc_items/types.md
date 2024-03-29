<!--
# Associated types
-->
# 関連型

<!--
The use of "Associated types" improves the overall readability of code 
by moving inner types locally into a trait as *output* types. Syntax
for the `trait` definition is as follows:
-->
関連型を使用すると、コンテナ型の中の要素をトレイトの中に *出力型* として書くことで、全体の可読性を上げることができます。トレイトを定義する際の構文は以下のようになります。

```rust
// `A` and `B` are defined in the trait via the `type` keyword.
// (Note: `type` in this context is different from `type` when used for
// aliases).
// `A`と`B`は`type`キーワードを用いてトレイト内で宣言されている。
// (注意: この文脈で使用する`type`は型エイリアスを宣言する際の`type`とは
// 異なることに注意しましょう。)
trait Contains {
    type A;
    type B;

    // Updated syntax to refer to these new types generically.
    // これらの新しい型をジェネリックに使用するために、構文が
    // アップデートされています。
    fn contains(&self, _: &Self::A, _: &Self::B) -> bool;
}
```

<!--
Note that functions that use the `trait` `Contains` are no longer required
to express `A` or `B` at all:
-->
`Contains`トレイトを使用する関数において、`A`と`B`を明示する必要がなくなっていることに注目しましょう。

```rust,ignore
// Without using associated types
// 関連型を使用しない場合
fn difference<A, B, C>(container: &C) -> i32 where
    C: Contains<A, B> { ... }

// Using associated types
// 使用する場合
fn difference<C: Contains>(container: &C) -> i32 { ... }
```

<!--
Let's rewrite the example from the previous section using associated types:
-->
前セクションの例を関連型を使用して書きなおしてみましょう。

```rust,editable
struct Container(i32, i32);

// A trait which checks if 2 items are stored inside of container.
// Also retrieves first or last value.
// 2つの要素がコンテナ型の中に保持されていることを確認するトレイト。
// また、最初あるいは最後の要素を取り出すこともできる。
trait Contains {
    // Define generic types here which methods will be able to utilize.
    // メソッドが使用できるジェネリック型を定義
    type A;
    type B;

    fn contains(&self, _: &Self::A, _: &Self::B) -> bool;
    fn first(&self) -> i32;
    fn last(&self) -> i32;
}

impl Contains for Container {
    // Specify what types `A` and `B` are. If the `input` type
    // is `Container(i32, i32)`, the `output` types are determined
    // as `i32` and `i32`.
    // `A`と`B`がどの型であるかを明示。インプットの型（訳注: つまり`Self`の型）
    // が`Container(i32, i32)`である場合、出力型は`i32`と`i32`となる。
    type A = i32;
    type B = i32;

    // `&Self::A` and `&Self::B` are also valid here.
    // `&i32`の代わりに`&Self::A`または`&self::B`と書いても良い
    fn contains(&self, number_1: &i32, number_2: &i32) -> bool {
        (&self.0 == number_1) && (&self.1 == number_2)
    }
    // Grab the first number.
    // 1つ目の数を取得
    fn first(&self) -> i32 { self.0 }

    // Grab the last number.
    // 最後の数を取得
    fn last(&self) -> i32 { self.1 }
}

fn difference<C: Contains>(container: &C) -> i32 {
    container.last() - container.first()
}

fn main() {
    let number_1 = 3;
    let number_2 = 10;

    let container = Container(number_1, number_2);

    println!("Does container contain {} and {}: {}",
        &number_1, &number_2,
        container.contains(&number_1, &number_2));
    println!("First number: {}", container.first());
    println!("Last number: {}", container.last());
    
    println!("The difference is: {}", difference(&container));
}
```
