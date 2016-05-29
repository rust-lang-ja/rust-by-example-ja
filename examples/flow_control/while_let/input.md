<!-- Similar to `if let`, `while let` can make awkward `match` sequences
more tolerable. Consider the following sequence that increments `i`: -->
`if let`と同様に、`while let`も不格好な`match`処理を多少マシにしてくれます。例えば、以下の`i`をインクリメントする処理を見てください。

```rust
// `Option<i32>`の`optional`を作成
let mut optional = Some(0);

// 変数の照合を繰り返し行う。
loop {
    match optional {
        // もし`optional`のデストラクトに成功した場合、値に応じて処理を分岐
        Some(i) => {
            if i > 9 {
                println!("Greater than 9, quit!");
                optional = None;
            } else {
                println!("`i` is `{:?}`. Try again.", i);
                optional = Some(i + 1);
            }
            // ^ 3つものインデントが必要。
        },
        // デストラクトに失敗した場合、ループを脱出
        _ => { break; }
        // どうしてこんな行を書く必要が?もっと良い方法があるはずです!
    }
}
```

<!-- Using `while let` makes this sequence much nicer: -->
`while let`の使用によってベターになります。

{while_let.play}

### See also:

[列挙型(`enum`)][enum], [`Option`][option], [RFC][while_let_rfc]

[enum]: ../custom_types/enum.html
[option]: ../std/option.html
[while_let_rfc]: https://github.com/rust-lang/rfcs/pull/214
