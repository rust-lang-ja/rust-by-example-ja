# 早期リターン

<!--
In the previous example, we explicitly handled the errors using combinators.
Another way to deal with this case analysis is to use a combination of
`match` statements and *early returns*.
-->
前の例では、コンビネータの活用によりエラーを明示的に処理しました。場合分けに対する別の対処法として、`match`文のコンビネータと早期リターンを使うこともできます。

<!--
That is, we can simply stop executing the function and return the error if
one occurs. For some, this form of code can be easier to both read and
write. Consider this version of the previous example, rewritten using early returns:
-->
つまり、エラーが発生した時点で関数の実行を止め、エラーを返してしまうという単純な方法が使えるということです。この方法の方がより読みやすく書きやすい場合があります。早期リターンを使って実装された、前の例の新たなバージョンを考えてみましょう。

```rust,editable
use std::num::ParseIntError;

fn multiply(first_number_str: &str, second_number_str: &str) -> Result<i32, ParseIntError> {
    let first_number = match first_number_str.parse::<i32>() {
        Ok(first_number)  => first_number,
        Err(e) => return Err(e),
    };

    let second_number = match second_number_str.parse::<i32>() {
        Ok(second_number)  => second_number,
        Err(e) => return Err(e),
    };

    Ok(first_number * second_number)
}

fn print(result: Result<i32, ParseIntError>) {
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

<!--
At this point, we've learned to explicitly handle errors using combinators
and early returns. While we generally want to avoid panicking, explicitly
handling all of our errors is cumbersome.
-->
ここまでで、コンビネータと早期リターンによる明示的なエラーハンドリングについて学びました。しかし、パニックは一般に避けたいですが、全てのエラーを明示的に処理するのも厄介でしょう。

<!--
In the next section, we'll introduce `?` for the cases where we simply
need to `unwrap` without possibly inducing `panic`.
-->
次の節では、`panic`を発生させずに`unwrap`する必要があるケースのための`?`について紹介していきます。