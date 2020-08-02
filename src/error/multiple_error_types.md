<!--
# Multiple error types
-->
# 複数のエラー型

<!--
The previous examples have always been very convenient; `Result`s interact
with other `Result`s and `Option`s interact with other `Option`s.
-->
`Result`が他の`Result`と連携したり、`Option`が他の`Option`と連携するなど、今までの例はとても便利な物でした。

<!--
Sometimes an `Option` needs to interact with a `Result`, or a
`Result<T, Error1>` needs to interact with a `Result<T, Error2>`. In those
cases, we want to manage our different error types in a way that makes them
composable and easy to interact with.
-->
時には`Option`が`Result`と連携したり、`Result<T, Error1>`が`Result<T, Error2>`と連携する必要もあるでしょう。そのような場面では、異なるエラー型を構成しやすく、かつ連携しやすく管理したいです。

<!--
In the following code, two instances of `unwrap` generate different error
types. `Vec::first` returns an `Option`, while `parse::<i32>` returns a
`Result<i32, ParseIntError>`:
-->
以下のコードは`unwrap`の２つのインスタンスが異なるエラー型を生成します。`Vec::first`は`Option`を返し、一方で`parse::<i32>`は`Result<i32, ParseIntError>`を返しています。

```rust,editable,ignore,mdbook-runnable
fn double_first(vec: Vec<&str>) -> i32 {
    let first = vec.first().unwrap(); // Generate error 1
                                      // エラー１の生成
    2 * first.parse::<i32>().unwrap() // Generate error 2
                                      // エラー２の生成
}

fn main() {
    let numbers = vec!["42", "93", "18"];
    let empty = vec![];
    let strings = vec!["tofu", "93", "18"];

    println!("The first doubled is {}", double_first(numbers));

    println!("The first doubled is {}", double_first(empty));
    // Error 1: the input vector is empty
    // エラー１：入力が空ベクトル

    println!("The first doubled is {}", double_first(strings));
    // Error 2: the element doesn't parse to a number
    // エラー２：要素が数字としてパースできない
}
```

<!--
Over the next sections, we'll see several strategies for handling these kind of problems.
-->
この先の節では、これらの問題を処理する方法について見ていきます。