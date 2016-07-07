/// あらゆる人物はここに代表されます。
pub struct Person {
    /// ジュリエットがどんなに名前というものを嫌っていようと、
    /// 人物には名前が必要です。
    name: String,
}

impl Person {
    /// 与えられた名前を持つpersonをを返します。
    ///
    /// # Arguments
    ///
    /// * `name` - `person`の名前を表す文字列のスライス
    ///
    /// # Example
    ///
    /// ```
    /// // バッククォートによってRustのコードをコメント中に挟むこと
    /// // もできます。Rustdocに--testを渡せば、テストも行えます！
    /// // (訳注: pythonのdoctestと同じです。)
    /// let person = Person::new("name");
    /// ```
    pub fn new(name: &str) -> Person {
        Person {
            name: name.to_string(),
        }
    }

    /// フレンドリーに挨拶しましょう！
    ///
    /// このメソッドを呼び出した`Person`に対して"Hello, [name]"
    /// と話しかけます。
    pub fn hello(& self) {
        println!("Hello, {}!", self.name);
    }
}

fn main() {
    let john = Person::new("John");

    john.hello();
}
