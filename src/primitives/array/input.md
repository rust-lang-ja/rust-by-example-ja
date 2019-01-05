<!--- An array is a collection of objects of the same type `T`, stored in contiguous --->
<!--- memory. Arrays are created using brackets `[]`, and their size, which is known --->
<!--- at compile time, is part of their type signature `[T; size]`. --->
配列は`T`という単一の型(訳注: [ジェネリック型](https://rust-lang-ja.github.io/the-rust-programming-language-ja/1.6/book/generics.html)でも可)のオブジェクトの集合です。それらのオブジェクトはメモリ上の連続した領域に保存されます。配列は`[]`を用いて生成されます。サイズはコンパイル時には決定されていて、`[T; size]`という形で指定できます。

<!--- Slices are similar to arrays, but their size is not known at compile time. --->
<!--- Instead, a slice is a two-word object, the first word is a pointer to the data, --->
<!--- and the second word is the length of the slice. Slices can be used to borrow a --->
<!--- section of an array, and have the type signature `&[T]`. --->
スライスは配列に似ていますが、コンパイル時にサイズが決定されていません。2つの部分からなり、1つは別のデータへのポインタ、もう1つはスライスの長さです。配列の一部を借用するのに使用され`&[T]`という型シグネチャを持ちます。

{array.play}
