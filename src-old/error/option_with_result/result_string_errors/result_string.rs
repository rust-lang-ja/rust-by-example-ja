type Result<T> = std::result::Result<T, String>;

fn double_first(vec: Vec<&str>) -> Result<i32> {
    vec.first()
       // `Option`が値を持つ場合`Result`に変換する。
       // `None`の場合、引数として与えた以下の文字列を持つ`Err`となる。
       .ok_or("Please use a vector with at least one element.".to_owned())
       // `parse`は`Result<T, ParseIntError>`を返す。
       .and_then(|s| s.parse::<i32>()
                      //  返り値の型は`Result<T, String>`なので、`parse`
                      //  により生じたエラーにのみmapを行い、`String`に変換する
                      .map_err(|e| e.to_string())
                      // 中の値を２倍する
                      .map(|i| 2 * i))
}

fn print(result: Result<i32>) {
    match result {
        Ok(n)  => println!("The first doubled is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

fn main() {
    let numbers = vec!["93", "18"];
    let empty = vec![];
    let strings = vec!["tofu", "93", "18"];

    print(double_first(numbers));
    print(double_first(empty));
    print(double_first(strings));
}
