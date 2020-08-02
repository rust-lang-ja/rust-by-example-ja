# `Result`

<!--
[`Result`][result] is a richer version of the [`Option`][option] type that
describes possible *error* instead of possible *absence*.
-->
[`Result`][result]は、リッチなバージョンの[`Option`][option]型で,
*値の不在*の可能性の代わりに*エラー*の可能性を示します。

<!--
That is, `Result<T, E>` could have one of two outcomes:
-->
つまり、`Result<T, E>`は以下の２つの結果を持ちます。

<!--
* `Ok(T)`: An element `T` was found
* `Err(E)`: An error was found with element `E`
-->
* `Ok<T>`: 要素`T`が見つかった場合
* `Err<E>`: 要素`E`とともにエラーが見つかった場合

<!--
By convention, the expected outcome is `Ok` while the unexpected outcome is `Err`.
-->
慣例により、`Ok`が期待される結果であり、`Err`は期待されない結果です。

<!--
Like `Option`, `Result` has many methods associated with it. `unwrap()`, for
example, either yields the element `T` or `panic`s. For case handling,
there are many combinators between `Result` and `Option` that overlap.
-->
`Option`と同様、`Result`は多くのメソッドを持ちます。例えば`unwrap()`は、`T`もしくは`panic`を受け渡します。エラーハンドリングでは、`Result`と`Option`で重複するコンビネータが多くあります。

<!--
In working with Rust, you will likely encounter methods that return the
`Result` type, such as the [`parse()`][parse] method. It might not always
be possible to parse a string into the other type, so `parse()` returns a
`Result` indicating possible failure.
-->
Rustを書いていく中で、[`parse()`][parse]メソッドなど、`Result`型を返すメソッドを目にするでしょう。文字列を他の型にパースすることは必ずしも成功する訳ではないため、`Result`を返すことで失敗するケースについてもカバーできるのです。

<!--
Let's see what happens when we successfully and unsuccessfully `parse()` a string:
-->
早速、`parse()`による文字列の成功例と失敗例を共に見てみましょう。

```rust,editable,ignore,mdbook-runnable
fn multiply(first_number_str: &str, second_number_str: &str) -> i32 {
    // Let's try using `unwrap()` to get the number out. Will it bite us?
    // `unwrap()`で数字を取り出しましてみましょう。痛い目を見るでしょうか？
    let first_number = first_number_str.parse::<i32>().unwrap();
    let second_number = second_number_str.parse::<i32>().unwrap();
    first_number * second_number
}

fn main() {
    let twenty = multiply("10", "2");
    println!("double is {}", twenty);

    let tt = multiply("t", "2");
    println!("double is {}", tt);
}
```

<!--
In the unsuccessful case, `parse()` leaves us with an error for `unwrap()`
to `panic` on. Additionally, the `panic` exits our program and provides an
unpleasant error message.
-->
失敗例では、`parse()`は`unwrap()`がパニックするためのエラーを残します。そして、`panic`はプログラムを終了させて不快なエラーメッセージを出力します。

<!--
To improve the quality of our error message, we should be more specific
about the return type and consider explicitly handling the error.
-->
エラーメッセージを改善するために、リターン型に対してもっと明確になるべきで、またエラーを明示的に処理することを考えるべきです。

<!--
## Using `Result` in `main`
-->
## `main`内で使う`Result`

<!--
The `Result` type can also be the return type of the `main` function if
specified explicitly. Typically the `main` function will be of the form:
-->
`Result`型は、明示的な指定により`main`関数のリターン型にもなります。一般に、`main`関数は以下のような形になるでしょう。

```rust
fn main() {
    println!("Hello World!");
}
```

<!--
However `main` is also able to have a return type of `Result`. If an error
occurs within the `main` function it will return an error code and print a debug
representation of the error (using the [`Debug`] trait). The following example
shows such a scenario and touches on aspects covered in [the following section].
-->
一方`main`で`Result`をリターン型とする場合、エラーが`main`関数内で発生した時、エラーコードを返し、エラーに関するデバッグ表記を（[`Debug`]トレートを使って）出力します。以下の例ではそのようなシナリオを示し、[この先の節]でカバーする内容に触れていきます。

```rust,editable
use std::num::ParseIntError;

fn main() -> Result<(), ParseIntError> {
    let number_str = "10";
    let number = match number_str.parse::<i32>() {
        Ok(number)  => number,
        Err(e) => return Err(e),
    };
    println!("{}", number);
    Ok(())
}
```


[option]: https://doc.rust-lang.org/std/option/enum.Option.html
[result]: https://doc.rust-lang.org/std/result/enum.Result.html
[parse]: https://doc.rust-lang.org/std/primitive.str.html#method.parse
[`Debug`]: https://doc.rust-lang.org/std/fmt/trait.Debug.html
[この先の節]: result/early_returns.md
