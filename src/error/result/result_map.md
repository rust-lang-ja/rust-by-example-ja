<!--
# `map` for `Result`
-->
# `Result`の`map`

<!--
Panicking in the previous example's `multiply` does not make for robust code.
Generally, we want to return the error to the caller so it can decide what is
the right way to respond to errors.
-->
前の例で見た`multiply`でのパニックは、コードを強固にするためには書きません。一般に、呼び出した側がエラーをどのように対処するべきかを自由に決められるように、エラーを呼び出した場所に返すのが好ましいです。

<!--
We first need to know what kind of error type we are dealing with. To determine
the `Err` type, we look to [`parse()`][parse], which is implemented with the
[`FromStr`][from_str] trait for [`i32`][i32]. As a result, the `Err` type is
specified as [`ParseIntError`][parse_int_error].
-->
まずは、どのようなエラー型を扱っているのかを知る必要があります。`Err`型を定めるために、[`i32`][i32]に対し[`FromStr`][from_str]トレートを使って実装された[`parse()`][parse]を見てみましょう。結果、`Err`型は[`ParseIntError`][parse_int_error]というものであることが分かります。

<!--
In the example below, the straightforward `match` statement leads to code
that is overall more cumbersome.
-->
以下の例では、単純な`match`文が全体として扱いづらいコードにしています。

```rust,editable
use std::num::ParseIntError;

// With the return type rewritten, we use pattern matching without `unwrap()`.
// リターン型を再度記述することで、パターンマッチングを`unwrap()`なしで行います。
fn multiply(first_number_str: &str, second_number_str: &str) -> Result<i32, ParseIntError> {
    match first_number_str.parse::<i32>() {
        Ok(first_number)  => {
            match second_number_str.parse::<i32>() {
                Ok(second_number)  => {
                    Ok(first_number * second_number)
                },
                Err(e) => Err(e),
            }
        },
        Err(e) => Err(e),
    }
}

fn print(result: Result<i32, ParseIntError>) {
    match result {
        Ok(n)  => println!("n is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

fn main() {
    // This still presents a reasonable answer.
    // ここは以前と変わらず、妥当な解を与えます。
    let twenty = multiply("10", "2");
    print(twenty);

    // The following now provides a much more helpful error message.
    // こちらは今度は有益なエラーメッセージを与えます。
    let tt = multiply("t", "2");
    print(tt);
}
```

<!--
Luckily, `Option`'s `map`, `and_then`, and many other combinators are also
implemented for `Result`. [`Result`][result] contains a complete listing.
-->
幸運にも、`Option`の`map`、`and_then`、その他多くのコンビネータも`Result`のために実装されています。[`Result`][result]に全てのリストが記載されています。

```rust,editable
use std::num::ParseIntError;

// As with `Option`, we can use combinators such as `map()`.
// This function is otherwise identical to the one above and reads:
// Multiply if both values can be parsed from str, otherwise pass on the error.
// `Option`と同様、`map()`などのコンビネータを使うことができます。
// この関数は`map()`を使っている点以外は上記の関数と同じで、
// 両方の値がstrからパース可能であればそれらを乗算し、無効であればエラーをそのまま見送ります。
fn multiply(first_number_str: &str, second_number_str: &str) -> Result<i32, ParseIntError> {
    first_number_str.parse::<i32>().and_then(|first_number| {
        second_number_str.parse::<i32>().map(|second_number| first_number * second_number)
    })
}

fn print(result: Result<i32, ParseIntError>) {
    match result {
        Ok(n)  => println!("n is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

fn main() {
    // This still presents a reasonable answer.
    // ここは以前と変わらず、妥当な解を与えます。
    let twenty = multiply("10", "2");
    print(twenty);

    // The following now provides a much more helpful error message.
    // こちらは今度は有益なエラーメッセージを与えます。
    let tt = multiply("t", "2");
    print(tt);
}
```

[parse]: https://doc.rust-lang.org/std/primitive.str.html#method.parse
[from_str]: https://doc.rust-lang.org/std/str/trait.FromStr.html
[i32]: https://doc.rust-lang.org/std/primitive.i32.html
[parse_int_error]: https://doc.rust-lang.org/std/num/struct.ParseIntError.html
[result]: https://doc.rust-lang.org/std/result/enum.Result.html
