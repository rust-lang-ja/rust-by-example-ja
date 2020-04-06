<!-- Using closures as input parameters are possible, so returning closures as
output parameters should also be possible. However, returning closure types
are problematic because Rust currently only supports returning concrete
(non-generic) types. Anonymous closure types are, by definition, unknown
and so returning a closure is only possible by making it concrete. This
can be done via boxing. -->
クロージャを引数として使用することができるのと同様、返り値をクロージャにすることもできます。ただし、現在の仕様ではRustは返り値をジェネリック型にすることはできません。無名クロージャは、定義上型が不明なので、返り値として渡すには、具体的な(`concrete`)型にする必要があります。これはbox化することで可能です。


<!-- The valid traits for returns are slightly different than before: -->
返り値となるクロージャが持つトレイトには若干の制限があります。

<!-- * `Fn`: normal
* `FnMut`: normal
* `FnOnce`: There are some unusual things at play here, so the [`FnBox`][fnbox]
  type is currently needed, and is unstable. This is expected to change in
  the future. -->
* `Fn`: 通常どおりです。
* `FnMut`: 通常どおりです。
* `FnOnce`: ほぼ同じ機能を持つ`FnBox`が必要になります。この仕様は将来的には廃止される可能性があります。

<!-- Beyond this, the `move` keyword must be used, which signals that all captures
occur by value. This is required because any captures by reference would be
dropped as soon as the function exited, leaving invalid references in the
closure. -->
さらに、リファレンスではなく値による捕捉が起きることを保証するため、`move`キーワードを使用する必要があります。リファレンスでの捕捉の場合、関数の実行が終わった段階で、リファレンスを開放する必要があるため、クロージャの中に不当なリファレンスが残ってしまうためです。

{output_parameters.play}

### See also:

[BOX化][box], [`Fn`][fn], [`FnMut`][fnmut], and [ジェネリクス][generics].

[box]: ../../std/box.html
[fn]: http://doc.rust-lang.org/std/ops/trait.Fn.html
[fnmut]: http://doc.rust-lang.org/std/ops/trait.FnMut.html
[fnbox]: http://doc.rust-lang.org/std/boxed/trait.FnBox.html
[generics]: ../../generics.html
