# `impl Trait`

<!--
`impl Trait` can be used in two locations:
-->
`impl Trait`は2つの利用方法があります：

<!--
1. as an argument type
2. as a return type
-->
1. 引数の型
2. リターン型

<!--
## As an argument type
-->
## 引数の型

<!--
If your function is generic over a trait but you don't mind the specific type, you can simplify the function declaration using `impl Trait` as the type of the argument.
-->
あなたの関数がジェネリックなトレイトを使用していて、特定の型を意識していない場合、`impl Trait`を引数の型として利用して、関数宣言をシンプルにできます。

<!--
For example, consider the following code:
-->
例えば、次のコードを考えてみましょう：

```rust,editable
fn parse_csv_document<R: std::io::BufRead>(src: R) -> std::io::Result<Vec<Vec<String>>> {
    src.lines()
        .map(|line| {
            // For each line in the source
            // ソースのそれぞれの行について
            line.map(|line| {
                // If the line was read successfully, process it, if not, return the error
                // 行を読み込んだら処理する。そうでなければエラーを返す。
                line.split(',') // Split the line separated by commas
                                // 行をカンマで分割する
                    .map(|entry| String::from(entry.trim())) // Remove leading and trailing whitespace
                                                             // 前後の空白を削除する
                    .collect() // Collect all strings in a row into a Vec<String>
                               // 各行の全ての文字列をVec<String>に集める
            })
        })
        .collect() // Collect all lines into a Vec<Vec<String>>
                   // 全ての行をVec<Vec<String>>に集める
}
```

<!--
`parse_csv_document` is generic, allowing it to take any type which implements BufRead, such as `BufReader<File>` or `[u8]`,
but it's not important what type `R` is, and `R` is only used to declare the type of `src`, so the function can also be written as:
-->
`parse_csv_document`はジェネリックなので、BufReadを実装する任意の型を取ることができます。
例えば、`BufReader<File>`や`[u8]`です。
`R`がどんな型かは重要ではなく、`src`の型宣言に使われているだけなので、この関数は以下のように書くこともできます：

```rust,editable
fn parse_csv_document(src: impl std::io::BufRead) -> std::io::Result<Vec<Vec<String>>> {
    src.lines()
        .map(|line| {
            // For each line in the source
            // ソースのそれぞれの行について
            line.map(|line| {
                // If the line was read successfully, process it, if not, return the error
                // 行を読み込んだら処理する。そうでなければエラーを返す。
                line.split(',') // Split the line separated by commas
                                // 行をカンマで分割する
                    .map(|entry| String::from(entry.trim())) // Remove leading and trailing whitespace
                                                             // 前後の空白を削除する
                    .collect() // Collect all strings in a row into a Vec<String>
                               // 各行の全ての文字列をVec<String>に集める
            })
        })
        .collect() // Collect all lines into a Vec<Vec<String>>
                   // 全ての行をVec<Vec<String>>に集める
}
```

<!--
Note that using `impl Trait` as an argument type means that you cannot explicitly state what form of the function you use, i.e. `parse_csv_document::<std::io::Empty>(std::io::empty())` will not work with the second example.
-->
`impl Trait`を引数の型として利用するということは、どのような形式の関数であるか明示できないので、注意してください。
例えば、`parse_csv_document::<std::io::Empty>(std::io::empty())`は2番目の例では動作しません。


<!--
## As a return type
-->
## リターン型

<!--
If your function returns a type that implements `MyTrait`, you can write its
return type as `-> impl MyTrait`. This can help simplify your type signatures quite a lot!
-->
あなたの関数が`MyTrait`を実装する型を返す場合、
リターン型を`-> impl MyTrait`のように書けます。
これで型シグネチャをとてもシンプルにできます。

```rust,editable
use std::iter;
use std::vec::IntoIter;

// This function combines two `Vec<i32>` and returns an iterator over it.
// Look how complicated its return type is!
// この関数は2つの`Vec<i32>を組み合わせて、そのイテレータを返します。
// リターン型がとても複雑です！
fn combine_vecs_explicit_return_type(
    v: Vec<i32>,
    u: Vec<i32>,
) -> iter::Cycle<iter::Chain<IntoIter<i32>, IntoIter<i32>>> {
    v.into_iter().chain(u.into_iter()).cycle()
}

// This is the exact same function, but its return type uses `impl Trait`.
// Look how much simpler it is!
// これは全く同じ関数ですが、リターン型に`impl Trait`を使っています。
// とてもシンプルですね！
fn combine_vecs(
    v: Vec<i32>,
    u: Vec<i32>,
) -> impl Iterator<Item=i32> {
    v.into_iter().chain(u.into_iter()).cycle()
}

fn main() {
    let v1 = vec![1, 2, 3];
    let v2 = vec![4, 5];
    let mut v3 = combine_vecs(v1, v2);
    assert_eq!(Some(1), v3.next());
    assert_eq!(Some(2), v3.next());
    assert_eq!(Some(3), v3.next());
    assert_eq!(Some(4), v3.next());
    assert_eq!(Some(5), v3.next());
    println!("all done");
}
```

<!--
More importantly, some Rust types can't be written out. For example, every
closure has its own unnamed concrete type. Before `impl Trait` syntax, you had
to allocate on the heap in order to return a closure. But now you can do it all
statically, like this:
-->
より重要なことに、Rustの型には書き表せないものがあるのです。
例えば、あらゆるクロージャは独自の無名な具象型を持ちます。
`impl Trait`構文がない時は、クロージャを返すにはヒープ上に置かねばなりませんでした。
しかし今では次のようにすべて静的に行えます。

```rust,editable
// Returns a function that adds `y` to its input
// 入力に`y`を加える関数を返す。
fn make_adder_function(y: i32) -> impl Fn(i32) -> i32 {
    let closure = move |x: i32| { x + y };
    closure
}

fn main() {
    let plus_one = make_adder_function(1);
    assert_eq!(plus_one(2), 3);
}
```

<!--
You can also use `impl Trait` to return an iterator that uses `map` or `filter`
closures! This makes using `map` and `filter` easier. Because closure types don't
have names, you can't write out an explicit return type if your function returns
iterators with closures. But with `impl Trait` you can do this easily:
-->
`impl Trait`を使って、`map`や`filter`クロージャを使うイテレータを返すこともできます。
おかげで`map`や`filter`を簡単に使えます。
クロージャ型は名前を持たないので、あなたの関数がクロージャを持つイテレータを返す場合、
明示的なリターン型を書くことはできません。
しかし`impl Trait`を使うことで簡単にできます：

```rust,editable
fn double_positives<'a>(numbers: &'a Vec<i32>) -> impl Iterator<Item = i32> + 'a {
    numbers
        .iter()
        .filter(|x| x > &&0)
        .map(|x| x * 2)
}

fn main() {
    let singles = vec![-3, -2, 2, 3];
    let doubles = double_positives(&singles);
    assert_eq!(doubles.collect::<Vec<i32>>(), vec![4, 6]);
}
```
