<!-- If you will notice from the previous example, when we call `parse`, the immediate reaction
is to `map` the error from a library error into our new custom error type. -->
前項の例では、`parse`を呼び出した直後に、生じたライブラリのエラー(訳注: `std::num::ParseIntError`)に対して`map`し、自身で定義したカスタムエラー型に変換したのを見ました。

```rust
.and_then(|s| s.parse::<i32>()
    .map_err(DoubleError::Parse)
```

<!-- This is a very simple and also common operation so it would be convenient if eliding it
would work but alas, it does not. `and_then` is not sufficiently flexible that it can handle
this; `try!` is though. -->
これはとてもシンプルで一般的なオペレーションなので、もし取り除くことができたら便利なのですが、悲しいかなそうするとうまく動作しません。`and_then`ではフレキシビリティが足りなくてこのケースには対処できないのです。しかし`try!`ならば可能です。

<!-- `try!` has previously been explained as either `unwrap` or `return Err(err)` which is only
`93%` correct. It actually means `unwrap` or `return Err(From::from(err))`. Since `From::from`
is a conversion utility between different types, this means if you `try!` something where the
error is convertible to the return type, it will convert automatically. This means, if we
rewrite this example with `try!` when `From::from` is implemented for our error type,
the `map_err` will go away: -->
以前、`try!`マクロは「`unwrap`、`return Err(err)`のどちらか」であると説明しました。これは`93%`の正しさでしかありません。正確には「`unwrap`、`return Err(From::from(err))`のどちらか」です。`From::from`は異なる型同士の変換をおこないます。これはつまり、何かを`try!`した際にエラーが生じ、そのエラーが返り値の型へと変換可能な場合、自動的に変換するということを意味します。従って、この例を`try!`マクロを用いて書き直し、`From::from`を我々のエラー型に対して実装すれば、`map_err`をなくすことが可能になるということを意味します。

{rethink.play}

<!-- This is actually fairly clean now. If you compare it with the original `panic`, it is very similar
to replacing the `unwrap` calls with `try!` except that the return types are `Result` and so
they must be destructured at the top level. -->
これで大分綺麗になりました。元々の`panic`していたものと比べると、`unwrap`の呼び出しが`try!`で置き換えられているという点で非常に似たものとなっています。違いは、返り値が`Result`なのでトップレベルで中身を取り出す必要があるという点です。

<!-- However, do not expect error handling of this sort to replace all usage of `unwrap` in
practice. Error handling of this sort tripled our code line count and cannot really be
called simple even if this is heavily biased by the small code size. Indeed, moving a 1000 line
library from `unwrap` to more proper error handling might be feasible in an additional
100 lines of code though the necessary refractoring definitely would not be trivial. -->
しかし実践上は、このようなエラーハンドリングによって`unwrap`の使用が全くなくなるということはないでしょう。というのも、ここではエラーハンドリングの仕方により、コードの行数が3倍になっています。全体のコード量の少なさを考慮に入れてもなお、これはとてもシンプルと呼べるものではありません。とはいえ、1000行のコードからなるライブラリでは、100行追加することにより`unwrap`からより洗練されたエラーハンドリングに移行するのは、十分にリファクタリングの価値があることです。

<!-- This is a very reasonable place to be. Many libraries might get away with only
implementing `Display` and then adding `From` on an as needed basis. A serious library
though will have users with certain expections about how it should implement error handling.
In those cases, the error handling will need to be taken one step further. -->
これは非常に理にかなった対処法です。ほとんどのライブラリでは、`Display`を追加し、必要に応じて`From`を追加するだけで、`unwrap`をなくすことができます。ただ、真剣に作られたライブラリの場合、ユーザーがどのようにエラーを扱うかに対してある程度想定しておくべきです。そのような場合、エラーハンドリングはもう一歩進んだやり方をする必要があります。

### See also:

[`From::from`][from]、[`try!`][try]

[from]: http://doc.rust-lang.org/std/convert/trait.From.html
[try]: http://doc.rust-lang.org/std/macro.try!.html
