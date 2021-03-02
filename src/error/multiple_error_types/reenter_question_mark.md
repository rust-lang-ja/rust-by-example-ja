<!--
# Other uses of `?`
-->
# `?`の他の活用法

<!--
Notice in the previous example that our immediate reaction to calling
`parse` is to `map` the error from a library error into a boxed
error:
-->
以前の例では`parse`の呼び出しに対するその場での対応として、エラーをライブラリのエラーからboxされたエラーへと`map`していました。

```rust,ignore
.and_then(|s| s.parse::<i32>()
    .map_err(|e| e.into())
```

<!--
Since this is a simple and common operation, it would be convenient if it
could be elided. Alas, because `and_then` is not sufficiently flexible, it
cannot. However, we can instead use `?`.
-->
簡単でよくあるオペレーションのため、可能なら省略してしまえると便利だったでしょう。でも残念、`and_then`が十分にフレキシブルでないため、それはできません。ただその代わり、`?`なら使えます。

<!--
`?` was previously explained as either `unwrap` or `return Err(err)`.
This is only mostly true. It actually means `unwrap` or
`return Err(From::from(err))`. Since `From::from` is a conversion utility
between different types, this means that if you `?` where the error is
convertible to the return type, it will convert automatically.
-->
`?`の挙動は、`unwrap`または`return Err(err)`として説明されていました。これはほぼ正解で、本当は`unwrap`、もしくは`return Err(From::from(err))`という意味があります。`From::from`は異なる型の間での変換ユーティリティであることから、エラーがリターン型に変換可能な場合に`?`を使うことで、その変換を自動的に行ってくれます。

<!--
Here, we rewrite the previous example using `?`. As a result, the
`map_err` will go away when `From::from` is implemented for our error type:
-->
前の例を`?`を使ったものに書き換えてみましょう。その結果、`From::from`がエラー型に実装されている時`map_err`は消えてなくなります。

```rust,editable
use std::error;
use std::fmt;

// Change the alias to `Box<dyn error::Error>`.
// エイリアスを`Box<error::Error>`に変更する。
type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug)]
struct EmptyVec;

impl fmt::Display for EmptyVec {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid first item to double")
    }
}

impl error::Error for EmptyVec {
    fn description(&self) -> &str {
        "invalid first item to double"
    }

    fn cause(&self) -> Option<&error::Error> {
        // Generic error, underlying cause isn't tracked.
        // 基本となるエラー、原因は記録されていない。
        None
    }
}

// The same structure as before but rather than chain all `Results`
// and `Options` along, we `?` to get the inner value out immediately.
// 前と同じ構造だが、`Results`と`Option`を繋げていく代わりに、
// `?`で内部の値をその場で取得します。
fn double_first(vec: Vec<&str>) -> Result<i32> {
    let first = vec.first().ok_or(EmptyVec)?;
    let parsed = first.parse::<i32>()?;
    Ok(2 * parsed)
}

fn print(result: Result<i32>) {
    match result {
        Ok(n)  => println!("The first doubled is {}", n),
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
This is actually fairly clean now. Compared with the original `panic`, it
is very similar to replacing the `unwrap` calls with `?` except that the
return types are `Result`. As a result, they must be destructured at the
top level.
-->
これでかなり綺麗になりました。元の`panic`と比べ、リターン型が`Result`であることを除けば、`unwrap`の呼び出しを`?`で置き換えたものに非常に似ています。結果、その`Result`は上のレベルで分解されなければなりません。

<!--
### See also:
-->
### 参照

[`From::from`][from] and [`?`][q_mark]

[from]: https://doc.rust-lang.org/std/convert/trait.From.html
[q_mark]: https://doc.rust-lang.org/reference/expressions/operator-expr.html#the-question-mark-operator
