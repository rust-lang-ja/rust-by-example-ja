# while let

<!--
Similar to `if let`, `while let` can make awkward `match` sequences
more tolerable. Consider the following sequence that increments `i`:
-->
`if let`と同様に、`while let`も不格好な`match`処理を多少マシにしてくれます。例えば、以下の`i`をインクリメントする処理を見てください。

```rust
// Make `optional` of type `Option<i32>`
// `Option<i32>`の`optional`を作成
let mut optional = Some(0);

// Repeatedly try this test.
// 変数の照合を繰り返し行う。
loop {
    match optional {
        // If `optional` destructures, evaluate the block.
        // もし`optional`のデストラクトに成功した場合、値に応じて処理を分岐
        Some(i) => {
            if i > 9 {
                println!("Greater than 9, quit!");
                optional = None;
            } else {
                println!("`i` is `{:?}`. Try again.", i);
                optional = Some(i + 1);
            }
            // ^ Requires 3 indentations!
            // ^ 3つものインデントが必要。
        },
        // Quit the loop when the destructure fails:
        // デストラクトに失敗した場合、ループを脱出
        _ => { break; }
        // ^ Why should this be required? There must be a better way!
        // どうしてこんな行を書く必要が?もっと良い方法があるはずです!
    }
}
```

<!--
Using `while let` makes this sequence much nicer:
-->
`while let`の使用によってベターになります。

```rust,editable
fn main() {
    // Make `optional` of type `Option<i32>`
    // `Option<i32>`の`optional`を作成
    let mut optional = Some(0);

    // This reads: "while `let` destructures `optional` into
    // `Some(i)`, evaluate the block (`{}`). Else `break`.
    // これは次のように読める。「`let`が`optional`を`Some(i)`にデストラクトしている間は
    // ブロック内(`{}`)を評価せよ。さもなくば`break`せよ。」
    while let Some(i) = optional {
        if i > 9 {
            println!("Greater than 9, quit!");
            optional = None;
        } else {
            println!("`i` is `{:?}`. Try again.", i);
            optional = Some(i + 1);
        }
        // ^ Less rightward drift and doesn't require
        // explicitly handling the failing case.
        // ^ インデントが少なく、デストラクト失敗時の処理を追加で書く必要がない。
    }
    // ^ `if let` had additional optional `else`/`else if`
    // clauses. `while let` does not have these.
    // ^ `if let`の場合は`else`/`else if`句が一つ余分にあったが、
    // `while let`では必要が無い。
}
```

<!--
### See also:
-->
### 参照

<!--
[`enum`][enum], [`Option`][option], and the [RFC][while_let_rfc]
-->
[列挙型(`enum`)][enum], [`Option`][option], [RFC][while_let_rfc]

[enum]: ../custom_types/enum.md
[option]: ../std/option.md
[while_let_rfc]: https://github.com/rust-lang/rfcs/pull/214
