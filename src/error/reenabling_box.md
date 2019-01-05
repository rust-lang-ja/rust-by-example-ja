<!-- We have seen that by implementing `Display` and `From` for our error type, we have enabled
usage of almost all of the std library error handling tools. That is, we missed one
capability: the ability to easily `box` our error type. -->
独自のエラー型に対して`Display`と`From`を実装することで、標準ライブラリのエラーハンドリング機能のほぼすべてが使用できるようになりました。「ほぼ」というのはつまり、エラー型を`box`化する機能がまだ備わっていないためです。

<!-- Namely, the std library will automatically convert from any type which implements the
`Error` trait into the trait object `Box<Error>` via `From`. To a library user, this
conveniently allows the following: -->
すなわち、標準ライブラリは`From`によって、`Error`トレイトを実装しているあらゆる型を`Box<Error>`というトレイトオブジェクトへと変換します。ライブラリのユーザにとっては、これは以下のような書き方ができて便利です。

``` rust,ignore
// 自動的に`Box<Error>`に変換できるエラー型ならばどんなものでもここで使用できる。
fn foo(...) -> Result<T, Box<Error>> { ... }
```

<!-- For example, a user may use a variety of libraries which each provide their own error
types. In order to define a valid `Result<T, E>` type, the user has a few choices: -->
例えば、ユーザーはそれぞれが独自のエラー型を提供している種々のライブラリを使用することがあるでしょう。有効な`Result<T, E>`を定義する方法として、ユーザーにはいくつかの選択肢があります。

<!-- * define a new wrapper error type around the external libraries error types
* convert it to `String` or some other intermediate choice
* box it up into `Box<Error>` via type erasure -->
* 外部ライブラリのエラー型をラップする新しいエラー型を定義する。
* `String`などの別の型にいったん変換して仲介させる。
* 型消去によって`Box<Error>`に変換する。

<!-- Boxing it is a common choice. The only penalty is that the underlying error type is only known
at runtime and not [statically determined][dynamic_dispatch]. All that needs to be done to enable
this is implement the `Error` trait: -->
普通はボックス化する方法を用います。唯一のペナルティは実際の(訳注: Box内の`Error`トレイトを実装している)エラー型が[静的に決定されているわけではない][dynamic_dispatch]ため、ランタイムまでわからないという点です。これを可能にするために唯一必要なのは、`Error`トレイトを実装することです。

``` rust,ignore
trait Error: Debug + Display {
    fn description(&self) -> &str;
    fn cause(&self) -> Option<&Error>;
}
```

<!-- By implementing this, our previous example would be just as valid when the error type
is `Box<Error>` as it was before with `DoubleError`. -->
これを実装することで、前項の例は`DoubleError`を用いる以前と同様に、`Box<Error>`に対して有効になります。

``` rust,editable
use std::error;
use std::fmt;
use std::num::ParseIntError;

// `Box<error::Error>`へのエイリアスに変更
type Result<T> = std::result::Result<T, Box<error::Error>>;

#[derive(Debug)]
enum DoubleError {
    EmptyVec,
    Parse(ParseIntError),
}

impl From<ParseIntError> for DoubleError {
    fn from(err: ParseIntError) -> DoubleError {
        DoubleError::Parse(err)
    }
}

impl fmt::Display for DoubleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DoubleError::EmptyVec =>
                write!(f, "please use a vector with at least one element"),
            DoubleError::Parse(ref e) => e.fmt(f),
        }
    }
}

impl error::Error for DoubleError {
    fn description(&self) -> &str {
        match *self {
            // エラー内容の非常に簡潔な説明。
            // `Display`と同じである必要はない。
            DoubleError::EmptyVec => "empty vectors not allowed",
            // これはすでに`Error`を実装しているので、その実装を用いる。
            DoubleError::Parse(ref e) => e.description(),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            // 隠れた原因があるわけではないので`None`を返す。
            DoubleError::EmptyVec => None,
            // 原因はParseIntError。`&error::Error`というトレイトオブジェクトに
            // 暗黙的に変換される。これが可能なのはParseIntErrorがすでに`Error`
            // トレイトを実装しているため。
            DoubleError::Parse(ref e) => Some(e),
        }
    }
}

fn double_first(vec: Vec<&str>) -> Result<i32> {
    let first = try!(vec.first().ok_or(DoubleError::EmptyVec));
    let parsed = try!(first.parse::<i32>());

    Ok(2 * parsed)
}

fn print(result: Result<i32>) {
    match result {
        Ok(n)  => println!("The first doubled is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

fn main() {
    let numbers = vec!["93", "18"];
    let empty = vec![];
    let strings = vec!["tofu", "93", "18"];

    print(double_first(numbers));
    print(double_first(empty));
    print(double_first(strings));
}

```

### See also:

[動的ディスパッチ][dynamic_dispatch]、[`Error`トレイト][error]

[dynamic_dispatch]: http://doc.rust-lang.org/book/trait-objects.html#dynamic-dispatch
[error]: http://doc.rust-lang.org/std/error/trait.Error.html
