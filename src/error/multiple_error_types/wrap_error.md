<!--
# Wrapping errors
-->
# エラーをラップする

<!--
An alternative to boxing errors is to wrap them in your own error type.
-->
Boxする方法の代替として、エラーを自前のエラー型としてラップする方法もあります。

```rust,editable
use std::error;
use std::error::Error;
use std::num::ParseIntError;
use std::fmt;

type Result<T> = std::result::Result<T, DoubleError>;

#[derive(Debug)]
enum DoubleError {
    EmptyVec,
    // We will defer to the parse error implementation for their error.
    // Supplying extra info requires adding more data to the type.
    // パースエラーの実装まで処理を委譲します。
    // 追加の情報を提供するには、型により多くのデータを追加する必要があります。
    Parse(ParseIntError),
}

impl fmt::Display for DoubleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DoubleError::EmptyVec =>
                write!(f, "please use a vector with at least one element"),
            // The wrapped error contains additional information and is available
            // via the source() method.
            // This is a wrapper, so defer to the underlying types' implementation of `fmt`.
            // ラップされたエラーは追加情報を含み、source メソッドから取り出すことができます。
            // これはラッパーなので、`fmt`での元となる型の実装に処理を任せます。
            DoubleError::Parse(..) =>
                write!(f, "the provided string could not be parsed as int"),
        }
    }
}

impl error::Error for DoubleError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            DoubleError::EmptyVec => None,
            // The cause is the underlying implementation error type. Is implicitly
            // cast to the trait object `&error::Error`. This works because the
            // underlying type already implements the `Error` trait.
            // 元の実装のエラー型が原因。
            // `&error::Error`トレートオブジェクトに暗にキャストされる。
            // 元となる型が`Error`トレートをすでに実装しているため問題なく動く。
            DoubleError::Parse(ref e) => Some(e),
        }
    }
}

// Implement the conversion from `ParseIntError` to `DoubleError`.
// This will be automatically called by `?` if a `ParseIntError`
// needs to be converted into a `DoubleError`.
// `ParseIntError`から`DoubleError`への変換の実装。
// `ParseIntError`が`DoubleError`に変換される必要がある時、自動的に`?`から呼び出される。
impl From<ParseIntError> for DoubleError {
    fn from(err: ParseIntError) -> DoubleError {
        DoubleError::Parse(err)
    }
}

fn double_first(vec: Vec<&str>) -> Result<i32> {
    let first = vec.first().ok_or(DoubleError::EmptyVec)?;
    // Here we implicitly use the `ParseIntError` implementation of `From` (which
    // we defined above) in order to create a `DoubleError`.
    let parsed = first.parse::<i32>()?;

    Ok(2 * parsed)
}

fn print(result: Result<i32>) {
    match result {
        Ok(n)  => println!("The first doubled is {}", n),
        Err(e) => {
            println!("Error: {}", e);
            if let Some(source) = e.source() {
                println!("  Caused by: {}", source);
            }
        },
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
This adds a bit more boilerplate for handling errors and might not be needed in
all applications. There are some libraries that can take care of the boilerplate
for you.
-->
これはエラーの処理のボイラープレートを増やしてしまい、全てのアプリケーションで必要になる訳では無いでしょう。これらのボイラープレートの処理を代わりにやってくれるようなライブラリもあります。

<!--
### See also:
-->
### 参照

[`From::from`][from] and [`Enums`][enums]

[from]: https://doc.rust-lang.org/std/convert/trait.From.html
[enums]: ../../custom_types/enum.md
