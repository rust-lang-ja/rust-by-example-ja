fn main() {
    // この変数はmain関数内が生息域です。
    let long_lived_binding = 1;

    // ここから下が`main`より小さいスコープを持つブロックとなります。
    {
        // この変数はこのブロック内のみに存在します。
        let short_lived_binding = 2;

        println!("inner short: {}", short_lived_binding);

        // この変数はスコープ外の同名の変数を*シャドーイング*します。
        let long_lived_binding = 5_f32;

        println!("inner long: {}", long_lived_binding);
    }
    // ブロックの終わり

    // `short_lived_binding`はこのスコープ内には存在しませんのでエラーとなります。
    println!("outer short: {}", short_lived_binding);
    // FIXME ^ コメントアウトしましょう

    println!("outer long: {}", long_lived_binding);

    // この変数バインディングも以前に定義した変数を*シャドーイング*します
    let long_lived_binding = 'a';

    println!("outer long: {}", long_lived_binding);
}
