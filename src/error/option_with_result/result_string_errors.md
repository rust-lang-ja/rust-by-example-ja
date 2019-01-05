<!-- Eliminating `unwrap` from the previous example requires more care. The two types in play
being `Option` and `Result`, one valid approach would be to convert both into a `Result`
with a common `Err` type. We will try it with `Err(String)` which seems like a nice first
approximation: -->
前項の例から`unwrap`を取り除くには(`Result`のみの場合よりも)より多くのケアを必要とします。ここで扱っている型は`Option`と`Result`の２つなので、有効な方法の一つとして、どちらも同じ`Err`型を持つ`Result`にしてしまう、ということが挙げられます。とりあえずは`Err(String)`から始めるのが良さそうに見えますのでこれを用いていきましょう。

``` rust,editable
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

```

<!-- This is not too bad but it is hardly as nice as the original (it can still be nicer but
we are not there yet). The question is, does this approach scale well. Consider the next
example. -->
これはそこまで悪くはないように見えますが、当初のコードに比べるとやはり難があります。問題となるのはこのアプローチの「スケーラビリティ」です。例えば次の例を見てください。

### See also:

[`Result`][result]、[`io::Result`][io_result]

[result]: http://doc.rust-lang.org/std/result/enum.Result.html
[io_result]: http://doc.rust-lang.org/std/io/type.Result.html
