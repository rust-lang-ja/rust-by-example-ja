struct Sheep { naked: bool, name: &'static str }

trait Animal {
    // スタティックメソッドのシグネチャ。
    // `Self` はこのトレイトを実装している型になる。
    fn new(name: &'static str) -> Self;

    // インスタンスメソッドのシグネチャ。
    // これらの関数は文字列を返す。
    fn name(&self) -> &'static str;
    fn noise(&self) -> &'static str;

    // メソッドのデフォルトの挙動を定義することもできる。
    fn talk(&self) {
        println!("{} says {}", self.name(), self.noise());
    }
}

impl Sheep {
    fn is_naked(&self) -> bool {
        self.naked
    }

    fn shear(&mut self) {
        if self.is_naked() {
            // メソッドをある型に実装する際に、その型のトレイトメソッドを
            // 使用することができる。
            println!("{} is already naked...", self.name());
        } else {
            println!("{} gets a haircut!", self.name);

            self.naked = true;
        }
    }
}

// `Animal`というトレイトを`Sheep`に実装する。
impl Animal for Sheep {
    // `Self`は実装対象の型: ここでは`Sheep`
    fn new(name: &'static str) -> Sheep {
        Sheep { name: name, naked: false }
    }

    fn name(&self) -> &'static str {
        self.name
    }

    fn noise(&self) -> &'static str {
        if self.is_naked() {
            "baaaaah?"
        } else {
            "baaaaah!"
        }
    }

    // デフォルトのトレイトメソッドはオーバーライドすることができる。
    fn talk(&self) {
        // 例えば、静かに熟考させてみる。
        println!("{} pauses briefly... {}", self.name, self.noise());
    }
}

fn main() {
    // この場合、型アノテーションが必須。
    let mut dolly: Sheep = Animal::new("Dolly");
    // TODO ^ ここの型アノテーションを消してみましょう。

    dolly.talk();
    dolly.shear();
    dolly.talk();
}
