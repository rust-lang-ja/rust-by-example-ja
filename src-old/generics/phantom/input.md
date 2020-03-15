<!-- A phantom type parameter is one that doesn't show up at runtime,
but is checked statically (and only) at compile time. -->
幽霊型(Phantom Type)とは実行時には存在しないけれども、コンパイル時に静的に型チェックされるような型のことです。

<!-- Data types can use extra generic type parameters to act as markers
or to perform compile-time type checking. These extra parameters
hold no storage values, and have no run-time behavior. -->
構造体などのデータ型は、ジェネリック型パラメータを一つ余分に持ち、それをマーカーとして使ったりコンパイル時の型検査に使ったりすることができます。このマーカーは実際の値を何も持たず、したがって実行時の挙動そのものにはいかなる影響ももたらしません。

<!-- In the following example, we combine [std::marker::PhantomData]
with the phantom type parameter concept to create tuples containing
different data types. -->
以下の例では、そのようなマーカーとして幽霊型([std::marker::PhantomData])を用い、それぞれ異なった型の値を持つタプルを作成します。

{phantom.play}

### See also:

[継承(`Derive`)][Derive], [構造体][struct], [タプル][TupleStructs]

[Derive]: ../trait/derive.html
[struct]: ../custom_types/structs.html
[TupleStructs]: ../custom_types/structs.html
[std::marker::PhantomData]: https://doc.rust-lang.org/std/marker/struct.PhantomData.html
