<!--- For some use cases, `match` is awkward. For example: --->
場合によっては`match`を使用すると不自然な書き方になってしまう場合があります。例えば...

```rust
// `optional`という変数の型を`Option<i32>`に指定
let optional = Some(7);

match optional {
    Some(i) => {
        println!("This is a really long string and `{:?}`", i);
        // ^ `i`をoption型からデストラクトするためだけに
        // インデントが一つ増えてしまっている。
    },
    _ => {},
    // ^ `match`は全ての型に対して網羅的でなくてはならないので必要。
    // 冗長に見えませんか？
};

```

<!--- `if let` is cleaner for this use case and in addition allows various --->
<!--- failure options to be specified: --->
この場合は`if let`を用いたほうが美しく、失敗時の処理も柔軟に行うことができます。

{if_let.play}

### See also:

[列挙型][enum], [オプション][option], [RFC][if_let_rfc]

[enum]: ../custom_types/enum.html
[if_let_rfc]: https://github.com/rust-lang/rfcs/pull/160
[option]: ../std/option.html
