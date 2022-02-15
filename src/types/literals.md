<!--
# Literals
-->
# リテラル

<!--
Numeric literals can be type annotated by adding the type as a suffix. As an example, 
to specify that the literal `42` should have the type `i32`, write `42i32`.
-->
数値型リテラルはサフィックスにより型を指定することが可能です。例えば、`42`というリテラルに対して`i32`型を指定するには`42i32`とします。

<!--
The type of unsuffixed numeric literals will depend on how they are used. If no
constraint exists, the compiler will use `i32` for integers, and `f64` for
floating-point numbers.
-->
サフィックスを指定しない数値型リテラルの場合、その型はどのように使用されるかに依存して決められます。デフォルトでは整数型の場合`i32`が、浮動小数点型は`f64`を使用します。

```rust,editable
fn main() {
    // Suffixed literals, their types are known at initialization
    // サフィックスを指定したリテラル。型は初期化とともに確定する。
    let x = 1u8;
    let y = 2u32;
    let z = 3f32;

    // Unsuffixed literals, their types depend on how they are used
    // サフィックスを指定しないリテラル。型は使用方法に依存する。
    let i = 1;
    let f = 1.0;

    // `size_of_val` returns the size of a variable in bytes
    // `size_of_val` 関数は変数のサイズをバイトで返す。
    println!("size of `x` in bytes: {}", std::mem::size_of_val(&x));
    println!("size of `y` in bytes: {}", std::mem::size_of_val(&y));
    println!("size of `z` in bytes: {}", std::mem::size_of_val(&z));
    println!("size of `i` in bytes: {}", std::mem::size_of_val(&i));
    println!("size of `f` in bytes: {}", std::mem::size_of_val(&f));
}
```

<!--
There are some concepts used in the previous code that haven't been explained
yet, here's a brief explanation for the impatient readers:
-->
上のコードには現時点では解説していない考えがいくつか使用されています。気になる方のために簡単に説明をしておきましょう。

<!--
* `std::mem::size_of_val` is a function, but called with its *full path*. Code
  can be split in logical units called *modules*. In this case, the
  `size_of_val` function is defined in the `mem` module, and the `mem` module
  is defined in the `std` *crate*. For more details, see
  [modules][mod] and [crates][crate].
-->
* `std::mem::size_of_val`は関数ですが、 *絶対パス(`full path`)* で呼び出されています。ソースコードは論理的に区切られた *モジュール* と呼ばれるものにわけられることができます。今回の場合は`size_of_val`関数は`mem`モジュール内で定義されており、`mem`モジュールは`std` *クレート* 内で定義されています。より詳しくは[クレート(`crates`)][crate]を参照してください。

[borrow]: ../scope/borrow.md
[mod]: ../mod.md
[crate]: ../crates.md
