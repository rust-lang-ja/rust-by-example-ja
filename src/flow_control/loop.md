<!--- Rust provides a `loop` keyword to indicate an infinite loop. --->
Rustには`loop`というキーワードが存在します。これは無限ループを作成するのに使用します。

> 訳注: `while True`と同じですが、ループのたびに条件を確認しないため、若干高速になります。

<!--- The `break` statement can be used to exit a loop at anytime, whereas the --->
<!--- `continue` statement can be used to skip the rest of the iteration and start a --->
<!--- new one. --->
ループから抜けだす時は`break`, 即座に次のループに移るときは`continue`が使用できます。

``` rust,editable
fn main() {
    let mut count = 0u32;

    println!("Let's count until infinity!");

    // 無限ループ
    loop {
        count += 1;

        if count == 3 {
            println!("three");

            // 残りの処理をスキップ
            continue;
        }

        println!("{}", count);

        if count == 5 {
            println!("OK, that's enough");

            // Exit this loop
            // ループを抜ける。
            break;
        }
    }
}

```
