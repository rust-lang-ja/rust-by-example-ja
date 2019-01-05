<!--- For some use cases, `match` is awkward. For example: --->
場合によっては`match`を使用すると不自然な書き方になってしまう場合があります。例えば...

``` rust
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

``` rust,editable
fn main() {
    // 全て`Option<i32>`型
    let number = Some(7);
    let letter: Option<i32> = None;
    let emoticon: Option<i32> = None;

    // `if let`文は以下と同じ意味.
    //
    // もしletがnumberをデストラクトした結果が`Some(i)`になるならば
    // ブロック内(`{}`)を実行する。
    if let Some(i) = number {
        println!("Matched {:?}!", i);
    }

    // デストラクトした結果が`Some()`にならない場合の処理を明示したい場合、
    // `else`を使用する。
    if let Some(i) = letter {
        println!("Matched {:?}!", i);
    } else {
        // デストラクト失敗の場合。このブロック内を実行
        println!("Didn't match a number. Let's go with a letter!");
    };

    // デストラクト失敗時の処理を更に分岐させることもできる
    let i_like_letters = false;

    if let Some(i) = emoticon {
        println!("Matched {:?}!", i);
    // デストラクト失敗。`else if`を評価し、処理をさらに分岐させる。
    } else if i_like_letters {
        println!("Didn't match a number. Let's go with a letter!");
    } else {
        // 今回は`else if`の評価がfalseなので、このブロック内がデフォルト
        println!("I don't like letters. Let's go with an emoticon :)!");
    };
}

```

### See also:

[列挙型][enum], [オプション][option], [RFC][if_let_rfc]

[enum]: ../custom_types/enum.html
[if_let_rfc]: https://github.com/rust-lang/rfcs/pull/160
[option]: ../std/option.html
