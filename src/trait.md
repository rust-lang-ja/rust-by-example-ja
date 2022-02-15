<!--
# Traits
-->
# トレイト

<!--
A `trait` is a collection of methods defined for an unknown type:
`Self`. They can access other methods declared in the same trait.
-->
トレイト(`trait`)とは任意の型となりうる`Self`に対して定義されたメソッドの集合のことです。同じトレイト内で宣言されたメソッド同士はお互いにアクセスすることができます。

<!--
Traits can be implemented for any data type. In the example below,
we define `Animal`, a group of methods. The `Animal` `trait` is 
then implemented for the `Sheep` data type, allowing the use of 
methods from `Animal` with a `Sheep`.
-->
トレイトはあらゆるデータ型に実装することができます。以下の例ではまず`Animal`というメソッドの集合を定義し、その後`Animal`トレイトを`Sheep`というデータ型に対して実装します。これにより`Animal`のメソッドを`Sheep`が使用することが可能になります。

```rust,editable
struct Sheep { naked: bool, name: &'static str }

trait Animal {
    // Associated function signature; `Self` refers to the implementor type.
    // 関連関数のシグネチャ。
    // `Self` はこのトレイトを実装している型になる。
    fn new(name: &'static str) -> Self;

    // Method signatures; these will return a string.
    // メソッドのシグネチャ。
    // これらの関数は文字列を返す。
    fn name(&self) -> &'static str;
    fn noise(&self) -> &'static str;

    // Traits can provide default method definitions.
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
            // Implementor methods can use the implementor's trait methods.
            // メソッドをある型に実装する際に、その型のトレイトメソッドを
            // 使用することができる。
            println!("{} is already naked...", self.name());
        } else {
            println!("{} gets a haircut!", self.name);

            self.naked = true;
        }
    }
}

// Implement the `Animal` trait for `Sheep`.
// `Animal`というトレイトを`Sheep`に実装する。
impl Animal for Sheep {
    // `Self` is the implementor type: `Sheep`.
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
    
    // Default trait methods can be overridden.
    // デフォルトのトレイトメソッドはオーバーライドすることができる。
    fn talk(&self) {
        // For example, we can add some quiet contemplation.
        // 例えば、静かに熟考させてみる。
        println!("{} pauses briefly... {}", self.name, self.noise());
    }
}

fn main() {
    // Type annotation is necessary in this case.
    // この場合、型アノテーションが必須。
    let mut dolly: Sheep = Animal::new("Dolly");
    // TODO ^ Try removing the type annotations.
    // TODO ^ ここの型アノテーションを消してみましょう。

    dolly.talk();
    dolly.shear();
    dolly.talk();
}
```
