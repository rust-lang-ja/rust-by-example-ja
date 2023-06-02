<!--
# `From` and `Into`
-->
# `From`および`Into`

<!--
The [`From`] and [`Into`] traits are inherently linked, and this is actually part of
its implementation. If you are able to convert type A from type B, then it
should be easy to believe that we should be able to convert type B to type A.
-->
[`From`]トレイトと[`Into`]トレイトは本質的に結びついており、そのことが実際に実装に反映されています。もし型Aから型Bへの変換ができるのであれば、型Bから型Aへの変換もできると思うのが自然です。

## `From`

<!--
The [`From`] trait allows for a type to define how to create itself from another
type, hence providing a very simple mechanism for converting between several
types. There are numerous implementations of this trait within the standard
library for conversion of primitive and common types.
-->
[`From`]トレイトは、ある型に対し、別の型からその型を作る方法を定義できるようにするものです。そのため、複数の型の間で型変換を行うための非常にシンプルな仕組みを提供しています。標準ライブラリでは、基本データ型やよく使われる型に対して、このトレイトが多数実装されています。

<!--
For example we can easily convert a `str` into a `String`
-->
例えば、`str`から`String`への型変換は簡単です。

```rust
let my_str = "hello";
let my_string = String::from(my_str);
```

<!--
We can do similar for defining a conversion for our own type.
-->
自作の型に対しても、型変換を定義すれば同じように行えます。

```rust,editable
use std::convert::From;

#[derive(Debug)]
struct Number {
    value: i32,
}

impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}

fn main() {
    let num = Number::from(30);
    println!("My number is {:?}", num);
}
```

## `Into`

<!--
The [`Into`] trait is simply the reciprocal of the `From` trait. That is, if you
have implemented the `From` trait for your type, `Into` will call it when
necessary.
-->
[`Into`]トレイトは、単に`From`トレイトの逆の働きをします。もし自作の型に`From`トレイトが実装されていたら、`Into`は必要に応じてそれを呼び出します。

<!--
Using the `Into` trait will typically require specification of the type to
convert into as the compiler is unable to determine this most of the time.
However this is a small trade-off considering we get the functionality for free.
-->
`Into`トレイトを使用すると、ほとんどの場合、コンパイラが型を決定することができないため、変換する型を指定する必要があります。しかし、この機能を無料で得られることを考えれば、これは小さなトレードオフです。

```rust,editable
use std::convert::From;

#[derive(Debug)]
struct Number {
    value: i32,
}

impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}

fn main() {
    let int = 5;
    // Try removing the type annotation
    // ここの型アノテーションを消してみましょう。
    let num: Number = int.into();
    println!("My number is {:?}", num);
}
```

[`From`]: https://doc.rust-lang.org/std/convert/trait.From.html
[`Into`]: https://doc.rust-lang.org/std/convert/trait.Into.html
