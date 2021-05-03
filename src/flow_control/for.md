<!--
# for loops
-->
# forループ

<!--
## for and range
-->
## for と range

<!--
The `for in` construct can be used to iterate through an `Iterator`.
One of the easiest ways to create an iterator is to use the range
notation `a..b`. This yields values from `a` (inclusive) to `b`
(exclusive) in steps of one.
-->
`for in`文を用いることで、イテレータ(`Iterator`)のそれぞれの要素に対して処理をすることが可能です。イテレータを作る最も単純な方法は`a..b`のような書き方をすることです。これは「`a`」から「`b`のひとつ前」までの要素を順に産出(`yield`)するというものです。

<!--
Let's write FizzBuzz using `for` instead of `while`.
-->
では`for`と`while`を用いてFizzBuzzを書いてみましょう。

```rust,editable
fn main() {
    // `n` will take the values: 1, 2, ..., 100 in each iteration
    // `n`は1, 2, ...., 100のそれぞれの値を取ります。
    for n in 1..101 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
    }
}
```

<!--
Alternatively, `a..=b` can be used for a range that is inclusive on both ends.
The above can be written as:
-->
上記の代わりに`a..=b`を用いると、両端の値を含む範囲を指定できます。上記の例は次のように書けます。

```rust,editable
fn main() {
    // `n` will take the values: 1, 2, ..., 100 in each iteration
    // `n`は1, 2, ...., 100のそれぞれの値を取ります。
    for n in 1..=100 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
    }
}
```

<!--
## for and iterators
-->
## forとイテレータ

<!--
The `for in` construct is able to interact with an `Iterator` in several ways.
As discussed in the section on the [Iterator][iter] trait, by default the `for`
loop will apply the `into_iter` function to the collection. However, this is
not the only means of converting collections into iterators.
-->
`for in`構文は`Iterator`とさまざまな方法でやり取りできます。[Iterator][iter]トレイトの章で説明したように、デフォルトでは`for`ループにおいて`into_iter`関数がコレクションに対して適用されます。しかし、コレクションをイテレータに変換する方法はこれだけではありません。

<!--
`into_iter`, `iter` and `iter_mut` all handle the conversion of a collection
into an iterator in different ways, by providing different views on the data
within.
-->
`into_iter`、`iter`、`iter_mut`はいずれもコレクションのイテレータへの変換を行いますが、データの「見せ方」の違いにより、そのやり方はそれぞれ異なります。

<!--
* `iter` - This borrows each element of the collection through each iteration.
  Thus leaving the collection untouched and available for reuse after the loop.
-->
* `iter` - この関数は、各周回においてコレクションの要素を借用します。よってコレクションには手を加えないので、ループの実行後もコレクションを再利用できます。

```rust, editable
fn main() {
    let names = vec!["Bob", "Frank", "Ferris"];

    for name in names.iter() {
        match name {
            &"Ferris" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }
}
```

<!--
* `into_iter` - This consumes the collection so that on each iteration the exact
  data is provided. Once the collection has been consumed it is no longer
  available for reuse as it has been 'moved' within the loop.
-->
* `into_iter` - この関数はコレクションからデータを取り出すので、各周回において要素のデータそのものが提供されます。データを取り出してしまうと、データはループ内に「移動」してしまうので、ループ実行後にコレクションを再利用することはできません。

```rust, editable
fn main() {
    let names = vec!["Bob", "Frank", "Ferris"];

    for name in names.into_iter() {
        match name {
            "Ferris" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }
}
```

<!--
* `iter_mut` - This mutably borrows each element of the collection, allowing for
  the collection to be modified in place.
-->
* `iter_mut` - この関数はコレクションの各要素をミュータブル（変更可能）で借用するので、コレクションの要素をその場で変更できます。

```rust, editable
fn main() {
    let mut names = vec!["Bob", "Frank", "Ferris"];

    for name in names.iter_mut() {
        *name = match name {
            &mut "Ferris" => "There is a rustacean among us!",
            _ => "Hello",
        }
    }

    println!("names: {:?}", names);
}
```

<!--
In the above snippets note the type of `match` branch, that is the key
difference in the types of iteration. The difference in type then of course
implies differing actions that are able to be performed.
-->
上記に示した3つのコードにおいて、`match`の選択肢の型の違いに注意してください。ここがそれぞれの方法の違いを生む鍵になっています。型が異なれば、当然ながらそれに対して行える処理も変わります。

<!--
### See also:
-->
### 参照

<!--
[Iterator][iter]
-->
[イテレータ][iter]

[iter]: ../trait/iter.md
