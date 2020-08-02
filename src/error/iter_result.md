<!--
# Iterating over `Result`s
-->
# `Result`をイテレートする

<!--
An `Iter::map` operation might fail, for example:
-->
`Iter::map`オペレーションは失敗することもあります。例えば、

```rust,editable
fn main() {
    let strings = vec!["tofu", "93", "18"];
    let numbers: Vec<_> = strings
        .into_iter()
        .map(|s| s.parse::<i32>())
        .collect();
    println!("Results: {:?}", numbers);
}
```

<!--
Let's step through strategies for handling this.
-->
ここでは、この対処法についてみてみましょう。

<!--
## Ignore the failed items with `filter_map()`
-->
## `filter_map()`を使って失敗した要素のみを無視する

<!--
`filter_map` calls a function and filters out the results that are `None`.
-->
`filter_map`は関数を呼び出し、結果が`None`になるものだけ取り除きます。

```rust,editable
fn main() {
    let strings = vec!["tofu", "93", "18"];
    let numbers: Vec<_> = strings
        .into_iter()
        .map(|s| s.parse::<i32>())
        .filter_map(Result::ok)
        .collect();
    println!("Results: {:?}", numbers);
}
```

<!--
## Fail the entire operation with `collect()`
-->
## `collect()`で全ての処理を失敗させる

<!--
`Result` implements `FromIter` so that a vector of results (`Vec<Result<T, E>>`)
can be turned into a result with a vector (`Result<Vec<T>, E>`). Once an
`Result::Err` is found, the iteration will terminate.
-->
`Result`は、それらのベクトル(`Vec<Result<T, E>>`)がベクトルのそれ(`Result<Vec<T>, E>`)へと変換できるようにするため、`FromIter`を実装します。`Result::Err`が見つかり次第、イテレーションは終了します。

```rust,editable
fn main() {
    let strings = vec!["tofu", "93", "18"];
    let numbers: Result<Vec<_>, _> = strings
        .into_iter()
        .map(|s| s.parse::<i32>())
        .collect();
    println!("Results: {:?}", numbers);
}
```

<!--
This same technique can be used with `Option`.
-->
同じテクニックは、`Option`を用いて行うこともできます。

<!--
## Collect all valid values and failures with `partition()`
-->
## `partition()`を使って全ての正常な値と失敗をまとめる

```rust,editable
fn main() {
    let strings = vec!["tofu", "93", "18"];
    let (numbers, errors): (Vec<_>, Vec<_>) = strings
        .into_iter()
        .map(|s| s.parse::<i32>())
        .partition(Result::is_ok);
    println!("Numbers: {:?}", numbers);
    println!("Errors: {:?}", errors);
}
```

<!--
When you look at the results, you'll note that everything is still wrapped in
`Result`.  A little more boilerplate is needed for this.
-->
結果を見てみると、まだ全て`Result`にラップされていることに気づくでしょう。もう少しのボイラープレートが必要です。

```rust,editable
fn main() {
    let strings = vec!["tofu", "93", "18"];
    let (numbers, errors): (Vec<_>, Vec<_>) = strings
        .into_iter()
        .map(|s| s.parse::<i32>())
        .partition(Result::is_ok);
    let numbers: Vec<_> = numbers.into_iter().map(Result::unwrap).collect();
    let errors: Vec<_> = errors.into_iter().map(Result::unwrap_err).collect();
    println!("Numbers: {:?}", numbers);
    println!("Errors: {:?}", errors);
}
```
