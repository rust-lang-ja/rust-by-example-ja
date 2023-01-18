<!--
# Enums
-->
# 列挙型

<!--
The `enum` keyword allows the creation of a type which may be one of a few
different variants. Any variant which is valid as a `struct` is also valid in
an `enum`.
-->
列挙型(`enum`)はいくつかの異なる要素型の中から1つを選ぶような場合に使用します。構造体(`struct`)の定義を満たすものならば何でも`enum` の要素型として使用できます。

```rust,editable
// Create an `enum` to classify a web event. Note how both
// names and type information together specify the variant:
// `PageLoad != PageUnload` and `KeyPress(char) != Paste(String)`.
// Each is different and independent.
// `enum`を作成してwebイベントを分類する。
// 名前と型情報を併せたものが要素型になっていることに注意。
// `PageLoad != PageUnload` であり、
// `KeyPress(char) != Paste(String)` である。
// 要素型は互いに異なり、互いに非依存である。
enum WebEvent {
    // An `enum` variant may either be `unit-like`,
    // `enum`要素型はユニット風でもよい
    PageLoad,
    PageUnload,
    // like tuple structs,
    // タプル風でもよい
    KeyPress(char),
    Paste(String),
    // or c-like structures.
    // C言語スタイルの構造体風でもよい
    Click { x: i64, y: i64 },
}

// A function which takes a `WebEvent` enum as an argument and
// returns nothing.
// 引数として`WebEvent`列挙型をとり、何も返さない関数
fn inspect(event: WebEvent) {
    match event {
        WebEvent::PageLoad => println!("page loaded"),
        WebEvent::PageUnload => println!("page unloaded"),
        // Destructure `c` from inside the `enum` variant.
        WebEvent::KeyPress(c) => println!("pressed '{}'.", c),
        WebEvent::Paste(s) => println!("pasted \"{}\".", s),
        // Destructure `Click` into `x` and `y`.
        WebEvent::Click { x, y } => {
            println!("clicked at x={}, y={}.", x, y);
        },
    }
}

fn main() {
    let pressed = WebEvent::KeyPress('x');
    // `to_owned()` creates an owned `String` from a string slice.
    // `to_owned()`は文字列スライスから所有権のある`String`を作成する
    let pasted  = WebEvent::Paste("my text".to_owned());
    let click   = WebEvent::Click { x: 20, y: 80 };
    let load    = WebEvent::PageLoad;
    let unload  = WebEvent::PageUnload;

    inspect(pressed);
    inspect(pasted);
    inspect(click);
    inspect(load);
    inspect(unload);
}

```

<!--
## Type aliases
-->
## 型エイリアス

<!--
If you use a type alias, you can refer to each enum variant via its alias.
This might be useful if the enum's name is too long or too generic, and you
want to rename it.
-->
型エイリアスを用いると、列挙型の要素型を別名で参照できます。これは列挙型の名前があまりに長かったり、あまりに一般的だったりで改名したい場合に役立ちます。

```rust,editable
enum VeryVerboseEnumOfThingsToDoWithNumbers {
    Add,
    Subtract,
}

// Creates a type alias
// 型エイリアスを作成する
type Operations = VeryVerboseEnumOfThingsToDoWithNumbers;

fn main() {
    // We can refer to each variant via its alias, not its long and inconvenient
    // name.
    // 長くて不便な列挙型の名前ではなく、別名を使って要素型を参照できる
    let x = Operations::Add;
}
```

<!--
The most common place you'll see this is in `impl` blocks using the `Self` alias.
-->
このやり方がもっともよく見られるのは、`impl`ブロックで`Self`という別名を使用する場合です。

```rust,editable
enum VeryVerboseEnumOfThingsToDoWithNumbers {
    Add,
    Subtract,
}

impl VeryVerboseEnumOfThingsToDoWithNumbers {
    fn run(&self, x: i32, y: i32) -> i32 {
        match self {
            Self::Add => x + y,
            Self::Subtract => x - y,
        }
    }
}
```

<!--
To learn more about enums and type aliases, you can read the
[stabilization report][aliasreport] from when this feature was stabilized into
Rust.
-->
列挙型や型エイリアスについて詳しく学びたい人は、この機能が安定してRustに取り込まれた後に[stabilization report][aliasreport]を読んでください。

<!--
### See also:
-->
### 参照

<!--
[`match`][match], [`fn`][fn], and [`String`][str], ["Type alias enum variants" RFC][type_alias_rfc]
-->
[マッチ(`match`)][match], [関数(`fn`)][fn], [文字列(`String`)][str], ["Type alias enum variants" RFC][type_alias_rfc]

[c_struct]: https://en.wikipedia.org/wiki/Struct_(C_programming_language)
[match]: ../flow_control/match.md
[fn]: ../fn.md
[str]: ../std/str.md
[aliasreport]: https://github.com/rust-lang/rust/pull/61682/#issuecomment-502472847
[type_alias_rfc]: https://rust-lang.github.io/rfcs/2338-type-alias-enum-variants.html
