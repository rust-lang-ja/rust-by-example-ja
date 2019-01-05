<!-- All values in Rust are stack allocated by default. Values can be *boxed*
(allocated in the heap) by creating a `Box<T>`. A box is a smart pointer to a
heap allocated value of type `T`. When a box goes out of scope, its destructor
is called, the inner object is destroyed, and the memory in the heap is freed. -->
Rustにおいて、すべての値はデフォルトでスタックに割り当てられます。`Box<T>`を作成することで、値を*ボックス化*、すなわちヒープ上に割り当てることができます。ボックスとは正確にはヒープ上におかれた`T`の値へのスマートポインタです。ボックスがスコープを抜けると、デストラクタが呼ばれて内包するオブジェクトが破棄され、ヒープメモリが解放されます。

<!-- Boxed values can be dereferenced using the `*` operator; this removes one layer
of indirection. -->
ボックス化された値は`*`オペレータを用いてデリファレンスすることができます。これにより一段と直接的な操作が可能になります。

{box.play}
