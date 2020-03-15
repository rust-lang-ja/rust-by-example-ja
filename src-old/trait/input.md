<!-- A `trait` is a collection of methods defined for an unknown type:
`Self`. They can access other methods declared in the same trait. -->
トレイト(`trait`)とは任意の型となりうる`Self`に対して定義されたメソッドの集合のことです。同じトレイト内で宣言されたメソッド同士はお互いにアクセスすることができます。

<!-- Traits can be implemented for any data type. In the example below,
we define `Animal`, a group of methods. The `Animal` `trait` is
then implemented for the `Sheep` data type, allowing the use of
methods from `Animal` with a `Sheep`. -->
トレイトはあらゆるデータ型に実装することができます。以下の例ではまず`Animal`というメソッドの集合を定義し、その後`Animal`トレイトを`Sheep`というデータ型に対して実装します。これにより`Animal`のメソッドを`Sheep`が使用することが可能になります。

{trait.play}
