<!-- We have been using `Strings` as errors for a while. In fact, this is somewhat limiting as
an error type. Below are the criteria for a good error type. `String` nicely fulfills the first
two but not the second two: -->
これまで、エラーの値として`String`をしばらく使用してきました。実のところ、これはエラーとして用いるには限界があります。以下に良いエラー型であるための条件を挙げます。`String`は1つめと2つめの条件はいい感じに満たしてくれるのですが後の2つは満たしません。

<!-- * Represents different errors with the same type
* Presents nice error messages to the user
* Is easily type comparable. Consider comparing these two types:
    - `Err("Please use a vector with at least one element".to_owned())`
    - `Err(EmptyVec)`
* Can hold information about the error. Compare:
    - `Err("+ cannot be used here".to_owned())`
    - `Err(BadChar(c, position))` -->
* 異なるエラーを一つの型で表現できる。
* ユーザーに綺麗なエラーメッセージを見せてくれる。
* 容易に型比較ができる。例えば以下の二つの型を比較することを考えてみましょう。
    - `Err("少なくとも一つの要素を持つベクタを使用してください".to_owned())`
    - `Err(EmptyVec)`
* エラーの内容に関する情報を持つことができる。例えば以下の二つを比べてください。
    - `Err("+ はここでは使用できません".to_owned())`
    - `Err(BadChar(c, position))`

<!-- This makes `String` errors both difficult to react to and verbose to create. In fact, a nice
looking error message has nothing to do with how the type is structured. It is simply a
consequence of `Display` being implemented for the type. It should not be necessary to
pollute logic heavy code with `String` formatting simply for nice error messages. -->
このことから、`String`をエラーとして用いると、エラーの対処・明示的な作成という点が困難になります。実際のところ、たとえエラーメッセージの見た目がいい感じであったとしても、それはそのエラーの型の構造とは何の関係もありません。単に、その型に対する`Display`の実装のみに依存するからです。単純にいい感じのエラーメッセージを表示したいという目的のために`String`を操作するコードを書くと、どうしてもコードベースが複雑になって汚染されてしまいますので、その必要はありません。

``` rust,editable
use std::num::ParseIntError;
use std::fmt;

type Result<T> = std::result::Result<T, DoubleError>;

#[derive(Debug)]
// 独自のエラー型を定義する。これならば、ここでのエラーハンドリングの仕方に合わせて
// 自由にカスタマイズできる。こうすることで、エラーの実装の裏側にあるツール、
// 独自に定義したエラー、あるいはその両方に従うことが可能となる。
enum DoubleError {
    // このエラーに関しては、エラーの詳細を説明するのに追加の情報は必要ない。
    EmptyVec,
    // パースに失敗した場合はParseIntErrorの挙動に追従する。追加の情報を持たせたい
    // 場合は、この型に、より多くのデータを追加しなくてはならない。
    Parse(ParseIntError),
}

// 型がどのように表示されるかは、そのエラーが生じた場所とは全く関係がない。
// また、ここで用いている複雑なロジックが、エラーのディスプレイスタイルによって
// そのままぶちまけられてしまうことも心配する必要はない。それらは別々の問題であって
// 別々に扱われる。
//
// ここではエラーに関して、追加の情報を保持してはいない。たとえば、パースに失敗したString
// がどれであるかを明示したいとする。その場合、独自に定義したエラー型を変更して、その情報を
// 保持するようにしてやる必要がある。
impl fmt::Display for DoubleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DoubleError::EmptyVec =>
                write!(f, "please use a vector with at least one element"),
            // こちらはラッパーなので、内側の型(この場合は`ParseIntError`)の`fmt`の実装に従う。
            DoubleError::Parse(ref e) => e.fmt(f),
        }
    }
}

fn double_first(vec: Vec<&str>) -> Result<i32> {
    vec.first()
       // エラーを新しい型のものに変換
       .ok_or(DoubleError::EmptyVec)
       .and_then(|s| s.parse::<i32>()
            // ここでも新しいエラー型へとアップデートさせる。
            .map_err(DoubleError::Parse)
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

### See also:

[`Result`][result]、[`io::Result`][io_result]

[result]: http://doc.rust-lang.org/std/result/enum.Result.html
[io_result]: http://doc.rust-lang.org/std/io/type.Result.html
[inplace]: /error/option_with_result/result_string_errors.html
