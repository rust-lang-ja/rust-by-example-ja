<!--
# Pulling `Result`s out of `Option`s
-->
# `Option`から`Result`を取り出す

<!--
The most basic way of handling mixed error types is to just embed them in each
other.
-->
混在するエラー型に対する最も基本的な対処法は、単にお互いを埋め込んでしまうことです。

```rust,editable
use std::num::ParseIntError;

fn double_first(vec: Vec<&str>) -> Option<Result<i32, ParseIntError>> {
    vec.first().map(|first| {
        first.parse::<i32>().map(|n| 2 * n)
    })
}

fn main() {
    let numbers = vec!["42", "93", "18"];
    let empty = vec![];
    let strings = vec!["tofu", "93", "18"];

    println!("The first doubled is {:?}", double_first(numbers));

    println!("The first doubled is {:?}", double_first(empty));
    // Error 1: the input vector is empty
    // エラー１：入力が空ベクトル

    println!("The first doubled is {:?}", double_first(strings));
    // Error 2: the element doesn't parse to a number
    // エラー２：要素が数字としてパースできない
}
```

<!--
There are times when we'll want to stop processing on errors (like with
[`?`][enter_question_mark]) but keep going when the `Option` is `None`. A
couple of combinators come in handy to swap the `Result` and `Option`.
-->
中には、`Option`の中身が`None`の場合はそのまま処理を進め、エラーの検出に限り実行を止めたいという場合もあるでしょう（[`?`][enter_question_mark]を使った時のように）。いくつかのコンビネータによって簡単に`Result`と`Option`をスワップすることができます。

```rust,editable
use std::num::ParseIntError;

fn double_first(vec: Vec<&str>) -> Result<Option<i32>, ParseIntError> {
    let opt = vec.first().map(|first| {
        first.parse::<i32>().map(|n| 2 * n)
    });

    opt.map_or(Ok(None), |r| r.map(Some))
}

fn main() {
    let numbers = vec!["42", "93", "18"];
    let empty = vec![];
    let strings = vec!["tofu", "93", "18"];

    println!("The first doubled is {:?}", double_first(numbers));
    println!("The first doubled is {:?}", double_first(empty));
    println!("The first doubled is {:?}", double_first(strings));
}
```

[enter_question_mark]: ../result/enter_question_mark.md
