use std::error;
use std::fmt;
use std::num::ParseIntError;

// `Box<error::Error>`へのエイリアスに変更
type Result<T> = std::result::Result<T, Box<error::Error>>;

#[derive(Debug)]
enum DoubleError {
    EmptyVec,
    Parse(ParseIntError),
}

impl From<ParseIntError> for DoubleError {
    fn from(err: ParseIntError) -> DoubleError {
        DoubleError::Parse(err)
    }
}

impl fmt::Display for DoubleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DoubleError::EmptyVec =>
                write!(f, "please use a vector with at least one element"),
            DoubleError::Parse(ref e) => e.fmt(f),
        }
    }
}

impl error::Error for DoubleError {
    fn description(&self) -> &str {
        match *self {
            // エラー内容の非常に簡潔な説明。
            // `Display`と同じである必要はない。
            DoubleError::EmptyVec => "empty vectors not allowed",
            // これはすでに`Error`を実装しているので、その実装を用いる。
            DoubleError::Parse(ref e) => e.description(),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            // 隠れた原因があるわけではないので`None`を返す。
            DoubleError::EmptyVec => None,
            // 原因はParseIntError。`&error::Error`というトレイトオブジェクトに
            // 暗黙的に変換される。これが可能なのはParseIntErrorがすでに`Error`
            // トレイトを実装しているため。
            DoubleError::Parse(ref e) => Some(e),
        }
    }
}

fn double_first(vec: Vec<&str>) -> Result<i32> {
    let first = try!(vec.first().ok_or(DoubleError::EmptyVec));
    let parsed = try!(first.parse::<i32>());

    Ok(2 * parsed)
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
