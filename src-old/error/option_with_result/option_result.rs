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
