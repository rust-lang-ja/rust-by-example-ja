fn main() {
    // `Option<i32>`の`optional`を作成
    let mut optional = Some(0);

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
        // ^ インデントが少なく、デストラクト失敗時の処理を追加で書く必要がない。
    }
    // ^ `if let`の場合は`else`/`else if`句が一つ余分にあったが、
    // `while let`では必要が無い。
}
