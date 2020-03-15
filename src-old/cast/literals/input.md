<!-- Numeric literals can be type annotated by adding the type as a suffix, with the
exception of `usize` that uses the `usize` suffix and `isize` that uses the
`isize` suffix. -->
数値型リテラルはサフィックスを指定することで型を指定することが可能です。例外は`usize`、`isize`で、これらはそれぞれ同名のサフィックスを使用します。

<!-- The type of unsuffixed numeric literals will depend on how they are used. If no
constraint exists, the compiler will use `i32` for integers, and `f64` for
floating-point numbers. -->
サフィックスを指定しない数値型リテラルの場合、その型はどのように使用されるかに依存して決められます。デフォルトでは整数型の場合`i32`が、浮動小数点型は`f64`を使用します。

{literals.play}

<!--
There are some concepts used in the previous code that haven't been explained
yet, here's a brief explanation for the impatient readers:

* `fun(&foo)` is used to pass an argument to a function *by reference*, rather
  than by value (`fun(foo)`). For more details see [borrowing][borrow].
* `std::mem::size_of_val` is a function, but called with its *full path*. Code
  can be split in logical units called *modules*. In this case, the
  `size_of_val` function is defined in the `mem` module, and the `mem` module
  is defined in the `std` *crate*. For more details, see
  [modules][mod] and [crates][crate].
-->
上のコードには現時点では解説していない考えがいくつか使用されています。気になる方のために簡単に説明をしておきましょう。

* `fun(&foo)`は値そのものではなく、その参照を関数に渡す時の書き方です。詳しくは[借用(`borrowing`)][borrow]を見てください。
* `std::mem::size_of_val`は関数ですが、*絶対パス(`full path`)*で呼び出されています。ソースコードは論理的に区切られた*モジュール*と呼ばれるものにわけられることができます。今回の場合は`size_of_val`関数は`mem`モジュール内で定義されており、`mem`モジュールは`std`*クレイト*内で定義されています。より詳しくは[クレイト(`crates`)][crate]を参照してください。

[borrow]: ../scope/borrow.html
[mod]: ../mod.html
[crate]: ../crates.html
