<!-- The previous problem was awkward because avoiding `unwrap` forced us to nest deeper and
deeper when what we really wanted was to get the variable *out*. So, is there any way
to accomodate this approach without `panic`? Well, what is a valid action to take when
an `Err` is found? It turns out there are two: -->
前項の例では、`unwrap`の使用を避けようとしたところ、ネストがどんどん深くなっていってしまいましたが、本当の目的は変数を**取り出す**ことだったはずです。では、`panic`せずにこのアプローチをよりコンパクトにする方法はあるのでしょうか？えーと、`Err`が見つかった時に取れる手法として有効なものには何があるでしょう？結論としては２つです。

<!-- 1. `panic!` which we already decided to try to avoid if possible
2. `return` because an `Err` means it cannot be handled -->
1. `panic!`、これは可能ならば避けるとすでに決めました。
2. `return`、`Err`とは処理が不可能であることを意味するためです。

<!-- This is exactly the purpose of `try!`; it is *almost*[^1] exactly equivalent to an
`unwrap` which `returns` instead of `panics` on `Errs`. -->
これこそまさに`try!`が必要となる理由です。これは、値が`Err`の際、`panic`の代わりに`return`することを除けば、`unwrap`と**ほぼ**[^1]同一です。

{try.play}

<!-- This really is a *huge* improvement but there is still the nagging issue of `map_err`. There is
actually a way to avoid it (we are using it everywhere it seems) but we are still missing some
details. First, we have to learn how to make better errors. -->
これは本当に**大きな**改善ですが、`map_err`がうるさいという問題はまだ解決していません。使わずに済ませる方法はあるのですが(今は必要そうなところ全てで使用しています)、そのためにはまだ触れていない詳細に立ち入る必要があります。まずはより良いエラーの作り方を学んでいきましょう。

[^1]: より詳しくは[try!再入門][re_enter_try]の項を見ましょう。

### See also:

[`Result`][result]、[`io::Result`][io_result]

[result]: http://doc.rust-lang.org/std/result/enum.Result.html
[io_result]: http://doc.rust-lang.org/std/io/type.Result.html
[re_enter_try]: ../../error/reenter_try.html
