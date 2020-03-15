<!-- As an introduction to this section, to borrow from [the official docs](
http://doc.rust-lang.org/book/unsafe.html), "one should try to minimize the
amount of unsafe code in a code base." With that in mind, let's get started!
Unsafe blocks in Rust are used to bypass protections put in place by the
compiler; specifically, there are four primary things that unsafe blocks are
used for: -->
この章の内容を見る前に、[公式ドキュメント](http://doc.rust-lang.org/book/unsafe.html)から引用した以下の文章をお読みください。

> コードベース中の、アンセーフな操作をするコードの量は、可能な限り小さく無くてはならない。

この戒めを頭に叩き込んだ上で、さあはじめましょう！

Rustにおいて、アンセーフなブロックはコンパイラのチェックをスルーするために使われます。具体的には以下の4つの主要なユースケースがあります。

<!-- * dereferencing raw pointers
* calling a function over FFI (but this is covered in a different part of the
  book)
* changing types through `std::cast::transmute`
* inline assembly -->
* 生ポインタのデリファレンス
* FFIを介した他言語関数の実行(別の章ですでに解説したので、ここではしません。)
* `std::cast::transmute`による型の変更
* インラインアセンブリ

<!-- ### Raw Pointers
Raw pointers `*` and references `&T` function similarly, but references are
always safe because they are guaranteed to point to valid data due to the
borrow checker. Dereferencing a raw pointer can only be done through an unsafe
block. -->
### 生ポインタ
生ポインタ`*`と参照`&T`はよく似た機能を持ちますが、後者は必ず有効なデータを指していることが借用チェッカーによって保証されているので、常に安全です。生ポインタのデリファレンスはアンセーフなブロックでしか実行できません。

{pointer.rs}

<!-- ### Transmute
Allows simple conversion from one type to another, however both types must have
the same size and alignment: -->
### トランスミュート
ある型から別の型へのシンプルな変換を可能にします。ただし双方の型がサイズとメモリ上への配置の仕方において同じでなくてはなりません。

{transmute.rs}
