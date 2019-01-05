<!-- The previous examples have always been very convenient; a `Result` interacts with the same
`Results` and an `Option` with the same `Option`. Sometimes it is not this easy though;
`Options` and `Results` may have to interact or even `Result<T, Error1>` with
`Result<T, Error2>`. -->
前項までの例では、`Result`は`Result`と、`Option`は`Option`としか関わらなかったため、非常に扱いやすいものでした。とはいえ、常にこのような扱いやすい場合ばかりではありません。`Option`と`Result`が、あるいは`Result<T, Error1>`と`Result<T, Error2>`とが関わる場合もあります。

<!-- Here is an example where one returns an `Option` and the other returns an `Result`. Aside
from messy errors provided by `unwrap`, this looks reasonable: -->
以下は、一方が`Option`を返し、もう一方が`Result`を返すような場合です。`unwrap`が汚いエラーメッセージを返すことを除けば、これは筋の通ったやり方に見えます。

``` rust,editable,ignore,mdbook-runnable
// まずは単純に`unwrap`を用いてみましょう。これは前述の好ましくない
// エラーメッセージを引き起こします。
fn double_first(vec: Vec<&str>) -> i32 {
    // ベクタが空の場合どうなる？
    let first = vec.first().unwrap();

    // 要素が数値にパースできない場合、どうなる？
    2 * first.parse::<i32>().unwrap()
}

fn main() {
    let numbers = vec!["93", "18"];
    let empty = vec![];
    let strings = vec!["tofu", "93", "18"];

    println!("The first doubled is {}", double_first(numbers));
    println!("The first doubled is {}", double_first(empty));
    // ^ ２つ目のエラーを見たければこの行をコメントアウトしましょう。
    println!("The first doubled is {}", double_first(strings));
}

```

### See also:

[`Result`][result] 、[`io::Result`][io_result]

[result]: http://doc.rust-lang.org/std/result/enum.Result.html
[io_result]: http://doc.rust-lang.org/std/io/type.Result.html
