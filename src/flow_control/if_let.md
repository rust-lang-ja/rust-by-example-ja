# if let

<!--
For some use cases, when matching enums, `match` is awkward. For example:
-->
列挙型をマッチさせるとき、場合によっては`match`を使用すると不自然な書き方になってしまう場合があります。例えば...

```rust
// Make `optional` of type `Option<i32>`
// `optional`という変数の型を`Option<i32>`に指定
let optional = Some(7);

match optional {
    Some(i) => {
        println!("This is a really long string and `{:?}`", i);
        // ^ Needed 2 indentations just so we could destructure
        // `i` from the option.
        // ^ `i`をoption型からデストラクトするためだけに
        // インデントが一つ増えてしまっている。
    },
    _ => {},
    // ^ Required because `match` is exhaustive. Doesn't it seem
    // like wasted space?
    // ^ `match`は全ての型に対して網羅的でなくてはならないので必要。
    // 冗長に見えませんか？
};

```

<!--
`if let` is cleaner for this use case and in addition allows various
failure options to be specified:
-->
この場合は`if let`を用いたほうが美しく、失敗時の処理も柔軟に行うことができます。

```rust,editable
fn main() {
    // All have type `Option<i32>`
    // 全て`Option<i32>`型
    let number = Some(7);
    let letter: Option<i32> = None;
    let emoticon: Option<i32> = None;

    // The `if let` construct reads: "if `let` destructures `number` into
    // `Some(i)`, evaluate the block (`{}`).
    // `if let`文は以下と同じ意味.
    //
    // もしletがnumberをデストラクトした結果が`Some(i)`になるならば
    // ブロック内(`{}`)を実行する。
    if let Some(i) = number {
        println!("Matched {:?}!", i);
    }

    // If you need to specify a failure, use an else:
    // デストラクトした結果が`Some()`にならない場合の処理を明示したい場合、
    // `else`を使用する。
    if let Some(i) = letter {
        println!("Matched {:?}!", i);
    } else {
        // Destructure failed. Change to the failure case.
        // デストラクト失敗の場合。このブロック内を実行
        println!("Didn't match a number. Let's go with a letter!");
    }

    // Provide an altered failing condition.
    // デストラクト失敗時の処理を更に分岐させることもできる
    let i_like_letters = false;

    if let Some(i) = emoticon {
        println!("Matched {:?}!", i);
    // Destructure failed. Evaluate an `else if` condition to see if the
    // alternate failure branch should be taken:
    // デストラクト失敗。`else if`を評価し、処理をさらに分岐させる。
    } else if i_like_letters {
        println!("Didn't match a number. Let's go with a letter!");
    } else {
        // The condition evaluated false. This branch is the default:
        // 今回は`else if`の評価がfalseなので、このブロック内がデフォルト
        println!("I don't like letters. Let's go with an emoticon :)!");
    }
}
```

In the same way, `if let` can be used to match any enum value:

```rust,editable
// Our example enum
enum Foo {
    Bar,
    Baz,
    Qux(u32)
}

fn main() {
    // Create example variables
    let a = Foo::Bar;
    let b = Foo::Baz;
    let c = Foo::Qux(100);
    
    // Variable a matches Foo::Bar
    if let Foo::Bar = a {
        println!("a is foobar");
    }
    
    // Variable b does not match Foo::Bar
    // So this will print nothing
    if let Foo::Bar = b {
        println!("b is foobar");
    }
    
    // Variable c matches Foo::Qux which has a value
    // Similar to Some() in the previous example
    if let Foo::Qux(value) = c {
        println!("c is {}", value);
    }

    // Binding also works with `if let`
    if let Foo::Qux(value @ 100) = c {
        println!("c is one hundred");
    }
}
```

Another benefit is that `if let` allows us to match non-parameterized enum variants. This is true even in cases where the enum doesn't implement or derive `PartialEq`. In such cases `if Foo::Bar == a` would fail to compile, because instances of the enum cannot be equated, however `if let` will continue to work.

Would you like a challenge? Fix the following example to use `if let`:

```rust,editable,ignore,mdbook-runnable
// This enum purposely neither implements nor derives PartialEq.
// That is why comparing Foo::Bar == a fails below.
enum Foo {Bar}

fn main() {
    let a = Foo::Bar;

    // Variable a matches Foo::Bar
    if Foo::Bar == a {
    // ^-- this causes a compile-time error. Use `if let` instead.
        println!("a is foobar");
    }
}
```

<!--
### See also:
-->
### 参照

<!--
[`enum`][enum], [`Option`][option], and the [RFC][if_let_rfc]
-->
[列挙型][enum], [オプション][option], [RFC][if_let_rfc]

[enum]: ../custom_types/enum.md
[if_let_rfc]: https://github.com/rust-lang/rfcs/pull/160
[option]: ../std/option.md
