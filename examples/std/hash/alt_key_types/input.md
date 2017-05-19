<!-- Any type that implements the `Eq` and `Hash` traits can be a key in `HashMap`.
This includes: -->
`Eq`と`Hash`トレイトを実装している型ならば、なんでも`HashMap`のキーになることができます。例えば以下です。

<!-- * `bool` (though not very useful since there is only two possible keys)
* `int`, `uint`, and all variations thereof
* `String` and `&str` (protip: you can have a `HashMap` keyed by `String`
and call `.get()` with an `&str`) -->
* `bool` (キーになりうる値が2つしかないので実用的ではないですが…)
* `int`、`uint`、あるいは他の整数型
* `String`と`&str`(Tips: `String`をキーにしたハッシュマップを作製した場合、`.get()`メソッドの引数に`&str`を与えて値を取得することができます。)

<!-- Note that `f32` and `f64` do *not* implement `Hash`,
likely because [floating-point precision errors][floating]
would make using them as hashmap keys horribly error-prone. -->
`f32`と`f64`は`Hash`を実装して**いない**ことに注意しましょう。おそらくこれは[浮動小数点演算時に誤差が発生する][floating]ため、キーとして使用すると、恐ろしいほどエラーの元となるためです。

<!-- All collection classes implement `Eq` and `Hash`
if their contained type also respectively implements `Eq` and `Hash`.
For example, `Vec<T>` will implement `Hash` if `T` implements `Hash`. -->
集合型は、その要素となっている全ての型が`Eq`を、あるいは`Hash`を実装している場合、必ず同じトレイトを実装しています。例えば、`Vec<T>`は`T`が`Hash`を実装している場合、`Hash`を実装します。

<!-- You can easily implement `Eq` and `Hash` for a custom type with just one line:
`#[derive(PartialEq, Eq, Hash)]` -->
独自の型に`Eq`あるいは`Hash`を実装するのは簡単です。以下の一行で済みます。
`#[derive(PartialEq, Eq, Hash)]`

<!-- The compiler will do the rest. If you want more control over the details,
you can implement `Eq` and/or `Hash` yourself.
This guide will not cover the specifics of implementing `Hash`. -->
後はコンパイラがよしなにしてくれます。これらのトレイトの詳細をコントロールしたい場合、`Eq`や`Hash`を自分で実装することもできます。この文書では`Hash`トレイトを実装する方法の詳細については触れません。

<!-- To play around with using a `struct` in `HashMap`,
let's try making a very simple user logon system: -->
`struct`を`HashMap`で扱う際の例として、とてもシンプルなユーザーログインシステムを作成してみましょう。

{alt_key_types.play}

[hash]: http://en.wikipedia.org/wiki/Hash_function
[floating]: http://en.wikipedia.org/wiki/Floating_point#Accuracy_problems
