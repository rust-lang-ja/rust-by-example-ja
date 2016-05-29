<!-- It has been noted that Rust chooses how to capture variables on the fly
without annotation. This is all very convenient in normal usage however when
writing functions, this ambiguity is not allowed. The closure's complete
type, including which capturing type, must be annotated. The manner of capture
a closure uses is annotated as one of the following `traits`: -->
ここまでで、アノテーションのない変数を、Rustが実行時に捕捉する例を何度か見てきました。通常はこれは大変便利なのですが、関数を書く際には、このように曖昧にしておくことは許されません。クロージャが捕捉する型を含むクロージャの完全型をはっきりさせておく必要があります。クロージャが外部変数を捕捉する方法は以下のトレイトで明示されています。

<!-- * `Fn`: takes captures by reference (`&T`)
* `FnMut`: takes captures by mutable reference (`&mut T`)
* `FnOnce`: takes captures by value (`T`) -->
* `Fn`: は変数をリファレンス(`&T`)で捕捉する。
* `FnMut`: は変数をミュータブルなリファレンス(`&mut T`)で捕捉する。
* `FnOnce`: は変数を値(`T`)で捕捉する。

<!-- Even annotated, these are very flexible: a parameter of `FnOnce` specifies
the closure *may* capture by `T` or `&mut T` or `&T` at will (if a move is
possible, any type of borrow should also be possible). The reverse is not
true: if the parameter is `Fn`, then nothing lower is allowed. Therefore,
the rule is: -->
このように明示されていても、クロージャの柔軟性は損なわれません。というのも`FnOnce`は、クロージャが`T`だけではなく、`&mut T`、`&T`のいずれかをとることが可能であるということを示しているにすぎないからです。(値の移動(`move`)が可能ならば、いかなるタイプの借用も可能です。)逆に`Fn`は`&mut T`や`T`をとることはできません。ゆえに捕捉のルールを一言でいうと

<!-- * any annotated parameter restricts capture to itself and above -->
* それぞれのトレイトは自分、あるいは上位のトレイトの型に補足対象を限定する。

となります。

> 訳注: わかりにくいですが、要するに`Fn` =< `FnMut` =< `FnOnce`という包含関係が成り立つということを言っています。

<!-- In addition, Rust will preferentially capture variables in the least
restrictive manner possible on a variable-by-variable basis: -->
付け加えると、Rustは変数を出来る限り制限の少ない方法(訳注: つまり`&T`)で補足します。


{input_parameters.play}

### See also:

[`std::mem::drop`][drop], [`Fn`][fn], [`FnMut`][fnmut], and [`FnOnce`][fnonce]

[drop]: http://doc.rust-lang.org/std/mem/fn.drop.html
[fn]: http://doc.rust-lang.org/std/ops/trait.Fn.html
[fnmut]: http://doc.rust-lang.org/std/ops/trait.FnMut.html
[fnonce]: http://doc.rust-lang.org/std/ops/trait.FnOnce.html
