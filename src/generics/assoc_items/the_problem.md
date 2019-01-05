<!-- A `trait` that is generic over its container type has type specification
requirements - users of the `trait` *must* specify all of its generic types. -->
コンテナ型に、その要素に対してジェネリックなトレイトを実装した場合、そのトレイトを使用する者は全てのジェネリック型を明記*しなくてはなりません*。

<!-- In the example below, the `Contains` `trait` allows the use of the generic
types `A` and `B`. The trait is then implemented for the `Container` type,
specifying `i32` for `A` and `B` so that it can be used with `fn difference()`. -->
以下の例では`Contains`トレイトはジェネリック型`A`と`B`の使用を許しています。その後、`Container`型に対して`Contains`を実装していますが、その際後に`fn difference()`が使用できるように、`A`、`B`はそれぞれ`i32`と明記されています。

<!-- Because `Contains` is generic, we are forced to explicitly state *all* of the
generic types for `fn difference()`. In practice, we want a way to express that
`A` and `B` are determined by the *input* `C`. As you will see in the next
section, associated types provide exactly that capability. -->
`Contains`はジェネリックトレイトなので、`fn difference()`では**全ての**ジェネリック型を宣言しなくてはなりません。実際のところ、`A`と`B`は**引数**である`C`によって決定されていて欲しいにも関わらず、です。これは次のページで紹介する関連型と呼ばれる機能によって可能です。

``` rust,editable
struct Container(i32, i32);

// ２つの要素がコンテナ型の中にあることをチェックするトレイト
// また、最初と最後の値を取得することもできる
trait Contains<A, B> {
    fn contains(&self, &A, &B) -> bool; // `A`と`B`の両方を明示的に要求する
    fn first(&self) -> i32; // `A`、`B`いずれも要求しない
    fn last(&self) -> i32;  // `A`、`B`いずれも要求しない

}

impl Contains<i32, i32> for Container {
    // コンテナ内の２つの要素が等しければTrueを返す
    fn contains(&self, number_1: &i32, number_2: &i32) -> bool {
        (&self.0 == number_1) && (&self.1 == number_2)
    }

    // ひとつ目の値を取得
    fn first(&self) -> i32 { self.0 }

    // 最後(2つめ)の値を取得
    fn last(&self) -> i32 { self.1 }
}

// `A`と`B`は`C`に保持されていることを考慮すると、`A`と`B`を
// ２度も書くのは面倒
fn difference<A, B, C>(container: &C) -> i32 where
    C: Contains<A, B> {
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

### See also:

[構造体][structs], [トレイト][traits]

[structs]: /custom_types/structs.html
[traits]: /trait.html
