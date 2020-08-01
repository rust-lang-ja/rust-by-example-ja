# `Result`に対するエイリアス

<!--
How about when we want to reuse a specific `Result` type many times?
Recall that Rust allows us to create [aliases][typealias]. Conveniently,
we can define one for the specific `Result` in question.
-->
特定の`Result`型を何度も使いたくなるのはどんな時でしょう？Rustは[エイリアス][typealias]の作成をサポートしていたことを思い出してください。便利なことに、議題である特定の`Result`型に対しても定義することができます。

<!--
At a module level, creating aliases can be particularly helpful. Errors
found in a specific module often have the same `Err` type, so a single alias
can succinctly define *all* associated `Results`. This is so useful that the
`std` library even supplies one: [`io::Result`][io_result]!
-->
モジュールレベルでは、エイリアスの作成は非常に役に立ちます。特定のモジュールで見られるエラーは同じ`Err`型を持つため、単一のエイリアスで簡潔に`Results`に関わる*全て*を定義できます。`std`ライブラリが提供するもの（[`io::Result`][io_result]）もあるほど有益なのです！

<!--
Here's a quick example to show off the syntax:
-->
簡単な例で構文を見てみましょう。

```rust,editable
use std::num::ParseIntError;

// Define a generic alias for a `Result` with the error type `ParseIntError`.
// `ParseIntError`を`Err`の型として持つ全ての`Result`のジェネリックエイリアス
type AliasedResult<T> = Result<T, ParseIntError>;

// Use the above alias to refer to our specific `Result` type.
// 上で定義したエイリアス（この場所特有の`Result`型）を使用
fn multiply(first_number_str: &str, second_number_str: &str) -> AliasedResult<i32> {
    first_number_str.parse::<i32>().and_then(|first_number| {
        second_number_str.parse::<i32>().map(|second_number| first_number * second_number)
    })
}

// Here, the alias again allows us to save some space.
// もう一度使用。エイリアスによって再度明記する必要性がない。
fn print(result: AliasedResult<i32>) {
    match result {
        Ok(n)  => println!("n is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

fn main() {
    print(multiply("10", "2"));
    print(multiply("t", "2"));
}
```

### See also:

[`io::Result`][io_result]

[typealias]: ../../types/alias.md
[io_result]: https://doc.rust-lang.org/std/io/type.Result.html
