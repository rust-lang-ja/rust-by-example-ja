<!-- Where vectors store values by an integer index, `HashMap`s store values by key.
`HashMap` keys can be booleans, integers, strings,
or any other type that implements the `Eq` and `Hash` traits.
More on this in the next section. -->
ベクタ型が値を整数のインデックスで保持するのに対し、`HashMap`ではキーで保持します。`HashMap`のキーはブーリアン、整数、文字列等の`Eq`あるいは`Hash`トレイトを保持する型なら何でもOKです。後でより詳しく見ていきます。

<!-- Like vectors, `HashMap`s are growable, but HashMaps can also shrink themselves
when they have excess space.
You can create a HashMap with a certain starting capacity using
`HashMap::with_capacity(uint)`, or use `HashMap::new()` to get a HashMap
with a default initial capacity (recommended). -->
ベクタ型と同様、伸長可能ですが、`HashMap`の場合さらに、スペースが余っているときには小さくすることも可能です。`HashMap`を一定の容量のエリアに作成するときは`HashMap::with_capacity(uint)`を、デフォルトの容量で作成するときは`HashMap::new()`を用います。後者が推奨されています。

``` rust,editable
use std::collections::HashMap;

fn call(number: &str) -> &str {
    match number {
        "798-1364" => "We're sorry, the call cannot be completed as dialed.
            Please hang up and try again.",
        "645-7689" => "Hello, this is Mr. Awesome's Pizza. My name is Fred.
            What can I get for you today?",
        _ => "Hi! Who is this again?"
    }
}

fn main() {
    let mut contacts = HashMap::new();

    contacts.insert("Daniel", "798-1364");
    contacts.insert("Ashley", "645-7689");
    contacts.insert("Katie", "435-8291");
    contacts.insert("Robert", "956-1745");

    // 参照をとり、Option<&V>を返す。
    match contacts.get(&"Daniel") {
        Some(&number) => println!("Calling Daniel: {}", call(number)),
        _ => println!("Don't have Daniel's number."),
    }

    // `HashMap::insert()`は
    // insertされた値が新しい場合は`None`を
    // そうでなければ`Some(value)`を返す。

    contacts.insert("Daniel", "164-6743");

    match contacts.get(&"Ashley") {
        Some(&number) => println!("Calling Ashley: {}", call(number)),
        _ => println!("Don't have Ashley's number."),
    }

    contacts.remove(&("Ashley"));

    // `HashMap::iter()`は(&'a key, &'a value)
    // のペアを順不同で産出するイテレータを返す
    for (contact, &number) in contacts.iter() {
        println!("Calling {}: {}", contact, call(number));
    }
}

```

<!-- For more information on how hashing and hash maps
(sometimes called hash tables) work, have a look at
[Hash Table Wikipedia][wiki-hash] -->
ハッシングやハッシュマップ(ハッシュテーブルと呼ばれることもあります)の仕組みについて、より詳しく知りたい場合は[Wikipediaのハッシュテーブルのページ][wiki-hash]を見てください。

[wiki-hash]: http://en.wikipedia.org/wiki/Hash_table
