<!-- Eliminating `unwrap` from the previous example requires more care. The two types in play
being `Option` and `Result`, one valid approach would be to convert both into a `Result`
with a common `Err` type. We will try it with `Err(String)` which seems like a nice first
approximation: -->
前項の例から`unwrap`を取り除くには(`Result`のみの場合よりも)より多くのケアを必要とします。ここで扱っている型は`Option`と`Result`の２つなので、有効な方法の一つとして、どちらも同じ`Err`型を持つ`Result`にしてしまう、ということが挙げられます。とりあえずは`Err(String)`から始めるのが良さそうに見えますのでこれを用いていきましょう。

{result_string.play}

<!-- This is not too bad but it is hardly as nice as the original (it can still be nicer but
we are not there yet). The question is, does this approach scale well. Consider the next
example. -->
これはそこまで悪くはないように見えますが、当初のコードに比べるとやはり難があります。問題となるのはこのアプローチの「スケーラビリティ」です。例えば次の例を見てください。

### See also:

[`Result`][result]、[`io::Result`][io_result]

[result]: http://doc.rust-lang.org/std/result/enum.Result.html
[io_result]: http://doc.rust-lang.org/std/io/type.Result.html
