<!-- Consider a `HashSet` as a `HashMap` where we just care about the keys (
`HashSet<T>` is, in actuality, just a wrapper around `HashMap<T, ()>`). -->
値がなく、キーだけの`HashMap`を想像してみてください。これはハッシュ集合(`HashSet`)と呼ばれるものです。(`HashSet<T>`は、実際には`HashMap<T, ()>`のラッパーです。)

<!-- "What's the point of that?" you ask. "I could just store the keys in a `Vec`." -->
「何の意味があるの？フツーにキーを`Vec`に入れればいいじゃん」そう思いましたね？

<!-- A `HashSet`'s unique feature is that
it is guaranteed to not have duplicate elements.
That's the contract that any set collection fulfills.
`HashSet` is just one implementation. (see also: [`BTreeSet`][treeset]) -->
それは、`HashSet`独自の機能として、要素に重複がないということが保証されるためです。これは全ての集合(`set`)型がもつ機能です。`HashSet`はその実装の1つであり、他には[`BTreeSet`][treeset]等があります。

<!-- If you insert a value that is already present in the `HashSet`,
(i.e. the new value is equal to the existing and they both have the same hash),
then the new value will replace the old. -->
`HashSet`に、すでに存在する値を加えようとすると、(すなわち、加えようとしている値のハッシュ値と、要素中のいずれかの値のハッシュ値が等しい場合、)新しい値によって古い値が上書きされます。

<!-- This is great for when you never want more than one of something,
or when you want to know if you've already got something. -->
これは、同じ値を2つ以上欲しくない場合や、すでにある値を持っているか知りたい場合にとても有効です。

<!-- But sets can do more than that. -->
しかし、集合型の機能はそれだけではありません。

<!-- Sets have 4 primary operations (all of the following calls return an iterator): -->
集合型には4つの主要なメソッドがあり、(すべてイテレータを返します。)

<!-- * `union`: get all the unique elements in both sets. -->
* `union`: 2つの集合型のどちらか一方にある値を全て取得

<!-- * `difference`: get all the elements that are in the first set but not the second. -->
* `difference`: 1つ目の集合にあり、かつ2つ目には存在しない値を全て取得。

<!-- * `intersection`: get all the elements that are only in *both* sets. -->
* `intersection`: 両方の集合にある値のみを取得。

<!-- * `symmetric_difference`:
get all the elements that are in one set or the other, but *not* both. -->
* `symmetric_difference`: どちらか一方の集合には存在するが、両方には**ない**値を取得

<!-- Try all of these in the following example. -->
以下の例でこれらをすべて見ていきましょう。

{hashset.play}

<!-- (Examples adapted from the [documentation.][hash-set]) -->
例は[公式ドキュメント][hash-set]から持ってきています。

[treeset]: http://doc.rust-lang.org/std/collections/struct.BTreeSet.html
[hash-set]: http://doc.rust-lang.org/std/collections/struct.HashSet.html#method.difference
