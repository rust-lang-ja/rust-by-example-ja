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
