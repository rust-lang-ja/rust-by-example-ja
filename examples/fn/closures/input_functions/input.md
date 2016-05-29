<!-- Since closures may be used as arguments, you might wonder if the same can be said
about functions. And indeed they can! However, because a function can
*never* capture variables, closures are strictly more flexible. Therefore, any
function which can take a closure as an argument can also take a function. -->
これまで、クロージャを引数として渡せることを見てきました。すると次の疑問が浮かんできます

「クロージャではない普通の関数を引数として渡すことは可能なのだろうか?」

可能です!とはいえ、通常の関数は*絶対に*周辺の変数を補足することができないので、クロージャの方がより柔軟な使い方ができます。ここから、クロージャを引数としてとる関数は、必ず通常の関数を引数としてとることができます。

{input_functions.play}

<!-- As an additional note, the `Fn`, `FnMut`, and `FnOnce` `traits` dictate how
a closure captures variables from the enclosing scope.  -->
クロージャによる変数の補足がどのように行われているかを詳しく見たいときは`Fn`、`FnMut`、`FnOnce`を参照してください。

### See also:

[`Fn`][fn], [`FnMut`][fn_mut], and [`FnOnce`][fn_once]

[fn]: http://doc.rust-lang.org/std/ops/trait.Fn.html
[fn_mut]: http://doc.rust-lang.org/std/ops/trait.FnMut.html
[fn_once]: http://doc.rust-lang.org/std/ops/trait.FnOnce.html
