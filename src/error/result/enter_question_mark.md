<!--
# Introducing `?`
-->
# `?`の導入

<!--
Sometimes we just want the simplicity of `unwrap` without the possibility of
a `panic`. Until now, `unwrap` has forced us to nest deeper and deeper when
what we really wanted was to get the variable *out*. This is exactly the purpose of `?`.
-->
時には`panic`の可能性を無視して、`unwrap`のシンプルさを活用したいこともあるでしょう。今までの`unwrap`は、値を*取り出す*ためだけであろうとも、ネストを深く書くことを要求しました。そして、これがまさに`?`の目的です。

<!--
Upon finding an `Err`, there are two valid actions to take:
-->
`Err`を見つけるにあたり、２つのとるべき行動があります。

<!--
1. `panic!` which we already decided to try to avoid if possible
2. `return` because an `Err` means it cannot be handled
-->
1. 可能な限り避けたいと決めた`panic!`
2. `Err`は処理できないことを意味するため`return`

<!--
`?` is *almost*[^†] exactly equivalent to an `unwrap` which `return`s
instead of `panic`king on `Err`s. Let's see how we can simplify the earlier
example that used combinators:
-->
`?`は*ほぼ*[^†]まさしく、`Err`に対して`panic`するより`return`するという点で`unwrap`と同等です。コンビネータを使った以前の例をどれだけ簡潔に書けるか見てみましょう。

```rust,editable
use std::num::ParseIntError;

fn multiply(first_number_str: &str, second_number_str: &str) -> Result<i32, ParseIntError> {
    let first_number = first_number_str.parse::<i32>()?;
    let second_number = second_number_str.parse::<i32>()?;

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
## The `try!` macro
-->
## `try!`マクロ

<!--
Before there was `?`, the same functionality was achieved with the `try!` macro.
The `?` operator is now recommended, but you may still find `try!` when looking
at older code. The same `multiply` function from the previous example
would look like this using `try!`:
-->
`?`ができる前、同様の動作を`try!`マクロによって行うことができました。現在は`?`オペレータが推奨されていますが、古いコードでは`try!`に出会うこともあります。`try!`を使って前の例と同じ`multiply`関数を実装すると、以下のようになるでしょう。

```rust,editable
// To compile and run this example without errors, while using Cargo, change the value 
// of the `edition` field, in the `[package]` section of the `Cargo.toml` file, to "2015".
// Cargoを使いながらこの例をエラーなくコンパイル、及び実行する場合、
// `Cargo.toml`ファイル内、`[package]`セクションの`edition`の値を"2015"に変更してください。

use std::num::ParseIntError;

fn multiply(first_number_str: &str, second_number_str: &str) -> Result<i32, ParseIntError> {
    let first_number = try!(first_number_str.parse::<i32>());
    let second_number = try!(second_number_str.parse::<i32>());

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


[^†]: 詳細は[re-enter ?][re_enter_?]を参照。

[re_enter_?]: ../multiple_error_types/reenter_question_mark.md
