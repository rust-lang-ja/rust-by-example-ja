# エラー型を定義する

<!--
Sometimes it simplifies the code to mask all of the different errors with a
single type of error.  We'll show this with a custom error.
-->
異なるエラー型をマスクし単一のエラー型として扱えるようにすると、コードがシンプルになる場合があります。ここでは自前のエラー型でそれを示してみます。

<!--
Rust allows us to define our own error types. In general, a "good" error type:
-->
Rustはユーザーによる新たなエラー型の定義をサポートします。一般に「良い」エラー型は、

<!--
* Represents different errors with the same type
* Presents nice error messages to the user
* Is easy to compare with other types
    - Good: `Err(EmptyVec)`
    - Bad: `Err("Please use a vector with at least one element".to_owned())`
* Can hold information about the error
    - Good: `Err(BadChar(c, position))`
    - Bad: `Err("+ cannot be used here".to_owned())`
* Composes well with other errors
-->
* 異なるエラーをまとめて同じ型として扱う
* ユーザーに優しいエラーメッセージを提供する
* 他の型との比較を楽にする
    - 良い例：`Err(EmptyVec)`
    - 悪い例：`Err("Please use a vector with at least one element".to_owned())`
* エラーについての情報を保持できる
    - 良い例：`Err(BadChar(c, position))`
    - 悪い例：`Err("+ cannot be used here".to_owned())`
* 他のエラーと問題なく連携できる

```rust,editable
use std::error;
use std::fmt;

type Result<T> = std::result::Result<T, DoubleError>;

// Define our error types. These may be customized for our error handling cases.
// Now we will be able to write our own errors, defer to an underlying error
// implementation, or do something in between.
// 自前のエラー型の定義。エラーハンドリングのケースの応じてカスタマイズされる。
// ここで新たなエラーを書くことができ、根底にあるエラーの実装に処理を延期したり、
// その手前で何らかの処理を挟むことができます。
#[derive(Debug, Clone)]
struct DoubleError;

// Generation of an error is completely separate from how it is displayed.
// There's no need to be concerned about cluttering complex logic with the display style.
// エラーの生成は、それがどのように表示されるかとは別物です。
// そのため、エラーの表示スタイルに関する複雑なロジックを煩雑になるなどと気にする必要はありません。
//
// Note that we don't store any extra info about the errors. This means we can't state
// which string failed to parse without modifying our types to carry that information.
// エラーに関する余分な情報を持たせていないことに注意してください。
// どの文字列がパースに失敗したかなどを出力することは、
// その情報を保持させるようにエラーの定義を修正しない限りできません。
impl fmt::Display for DoubleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid first item to double")
    }
}

// This is important for other errors to wrap this one.
impl error::Error for DoubleError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        // Generic error, underlying cause isn't tracked.
        // 基本となるエラー、原因は記録されていない。
        None
    }
}

fn double_first(vec: Vec<&str>) -> Result<i32> {
    vec.first()
        // Change the error to our new type.
        // エラーを新たな型に変換する。
        .ok_or(DoubleError)
        .and_then(|s| {
            s.parse::<i32>()
                // Update to the new error type here also.
                // ここでも新たなエラー型に更新する。
                .map_err(|_| DoubleError)
                .map(|i| 2 * i)
        })
}

fn print(result: Result<i32>) {
    match result {
        Ok(n) => println!("The first doubled is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

fn main() {
    let numbers = vec!["42", "93", "18"];
    let empty = vec![];
    let strings = vec!["tofu", "93", "18"];

    print(double_first(numbers));
    print(double_first(empty));
    print(double_first(strings));
}
```
