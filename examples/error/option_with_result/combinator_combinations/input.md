<!-- What if multiple `Results` needed to interact together? Is it still reasonably convenient?
It turns out, not really. -->
複数の`Result`がお互いに関係しあう必要がある場合はどうでしょう？扱いやすさに変わりはないでしょうか？実際に見てみるとわかりますが、そうでもありません。

{result_try.play}

<!-- What is happening is this approach tries to work with the data without ever removing the `Ok`
wrapper on it. Sometimes it is a good approach but in this case it is really awkward. What if
we could `unwrap` it without possibly inducing `panic`? That is where we are headed next. -->
このアプローチでは、値をラップする`Ok`を除外せずにそのまま扱おうとしています。時にはこれが良いアプローチである場合もありますが。今回は非常にぎこちないものとなってしまっています。`panic`を引き起こす可能性なしに、`unwrap`することができたらどうでしょう？次のステップではその方法を見ていきます。

### See also:

[`Result`][result]、[`io::Result`][io_result]

[result]: http://doc.rust-lang.org/std/result/enum.Result.html
[io_result]: http://doc.rust-lang.org/std/io/type.Result.html
