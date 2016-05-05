// 使用されていないコードによる警告を隠すアトリビュート
#![allow(dead_code)]

// 人間を分類するための`enum`を作成。要素の名前と型の情報の両方が
// 変数の内容を同定するために、使用されていることに注目、つまり
// `Skinny != Fat`であり、`Height(i32) != Weight(i32)`である。
enum Person {
    // `enum`の要素は以下の用に様々な型を取れる
    // ユニットライクな型
    Skinny,
    Fat,
    // タプル
    Height(i32),
    Weight(i32),
    // 構造体
    Info { name: String, height: i32 }
}

// `Person`型のEnumを引数にとり、返り値のない関数。
fn inspect(p: Person) {
    // `match`が`enum`型を精査する場合、その型について網羅的
    // でなくてはならない。
    // （訳注: `enum`の内容が定かでない場合は`_`を使用する）
    match p {
        Person::Skinny    => println!("Is skinny!"),
        Person::Fat       => println!("Is fat!"),
        // `Person`の中から`i`をデストラクトする。
        Person::Height(i) => println!("Has a height of {}.", i),
        Person::Weight(i) => println!("Has a weight of {}.", i),
        // `info`を`height`と`name`にデストラクトする。
        Person::Info { name, height } => {
            println!("{} is {} tall!", name, height);
        },
    }
}

fn main() {
    let person = Person::Height(18);
    let danny  = Person::Weight(10);
    // `to_owned()`は文字列のスライスから、新しい所有権の`String`を作成する。
    let dave   = Person::Info { name: "Dave".to_owned(), height: 72 };
    let john   = Person::Fat;
    let larry  = Person::Skinny;

    inspect(person);
    inspect(danny);
    inspect(dave);
    inspect(john);
    inspect(larry);
}
