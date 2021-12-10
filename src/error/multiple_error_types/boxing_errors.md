<!--
# `Box`ing errors
-->
# エラーを`Box`する

<!--
A way to write simple code while preserving the original errors is to [`Box`][box]
them.  The drawback is that the underlying error type is only known at runtime and not
[statically determined][dynamic_dispatch].
-->
元のエラーを維持しながらシンプルなコードを書くには、[`Box`][box]してしまうと良いでしょう。欠点として、元のエラー型はランタイムまで判明せず、[静的に決定][dynamic_dispatch]されないことが挙げられます。

<!--
The stdlib helps in boxing our errors by having `Box` implement conversion from
any type that implements the `Error` trait into the trait object `Box<Error>`,
via [`From`][from].
-->
標準ライブラリは`Box`に、[`From`][from]を介してあらゆる`Error`トレートを実装した型から`Box<Error>`トレートオブジェクトへの変換を実装させることで、エラーをboxしやすくしてくれます。

```rust,editable
use std::error;
use std::fmt;

// Change the alias to `Box<error::Error>`.
// エイリアスを`Box<error::Error>`に変更する。
type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug, Clone)]
struct EmptyVec;

impl fmt::Display for EmptyVec {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid first item to double")
    }
}

impl error::Error for EmptyVec {}

fn double_first(vec: Vec<&str>) -> Result<i32> {
    vec.first()
        .ok_or_else(|| EmptyVec.into()) // Converts to Box
                                        // Boxに変換
        .and_then(|s| {
            s.parse::<i32>()
                .map_err(|e| e.into()) // Converts to Box
                                       // Boxに変換
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

<!--
### See also:
-->
### 参照

[Dynamic dispatch][dynamic_dispatch] and [`Error` trait][error]

[box]: https://doc.rust-lang.org/std/boxed/struct.Box.html
[dynamic_dispatch]: https://doc.rust-lang.org/book/ch17-02-trait-objects.html#trait-objects-perform-dynamic-dispatch
[error]: https://doc.rust-lang.org/std/error/trait.Error.html
[from]: https://doc.rust-lang.org/std/convert/trait.From.html
