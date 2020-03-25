<!--
# Testcase: linked-list
-->
# テストケース: 連結リスト

<!--
A common use for `enums` is to create a linked-list:
-->
`enum`を使用が適切なパターンのひとつに、連結リスト(`linked-list`)を作成する場合があります。

```rust,editable
use crate::List::*;

enum List {
    // Cons: Tuple struct that wraps an element and a pointer to the next node
    // Cons: これは、要素をラップし、次の要素へのポインタを保持するタプル。
    Cons(u32, Box<List>),
    // Nil: A node that signifies the end of the linked list
    // Nil: 連結リストの終端であることを示すノード
    Nil,
}

// Methods can be attached to an enum
// 列挙型にはメソッドを付与することができる。
impl List {
    // Create an empty list
    // 空リストの作成。
    fn new() -> List {
        // `Nil` has type `List`
        // `Nil` は `List`型を持つ。
        Nil
    }

    // Consume a list, and return the same list with a new element at its front
    // リストを受け取り、その始端に新しい要素を付加したものを返す関数。
    fn prepend(self, elem: u32) -> List {
        // `Cons` also has type List
        // この`Cons` 自体も、その第2要素もどちらもlist型である。
        Cons(elem, Box::new(self))
    }

    // Return the length of the list
    // list の長さを返すメソッド
    fn len(&self) -> u32 {
        // `self` has to be matched, because the behavior of this method
        // depends on the variant of `self`
        // `self` has type `&List`, and `*self` has type `List`, matching on a
        // concrete type `T` is preferred over a match on a reference `&T`
        // このメソッドは、`self`の状態によって振る舞いが
        // 変化するため、matchをする必要がある。
        // `self`の型は`&List`であるので、`*self`は`List`になる。マッチングは
        // リファレンス（`&T`）ではなく実体（`T`）に対して行うのが好ましい。
        match *self {
            // Can't take ownership of the tail, because `self` is borrowed;
            // instead take a reference to the tail
            // `self`をすでに借用しているので、tailの所有権を取ることができない。
            // 代わりに参照を使用する。
            Cons(_, ref tail) => 1 + tail.len(),
            // Base Case: An empty list has zero length
            // 空リストならば長さは0
            Nil => 0
        }
    }

    // Return representation of the list as a (heap allocated) string
    // Listをheap上の文字列として表したものを返すメソッド。
    fn stringify(&self) -> String {
        match *self {
            Cons(head, ref tail) => {
                // `format!` is similar to `print!`, but returns a heap
                // allocated string instead of printing to the console
                // `format!`は`print!`に似ているが、コンソール上に出力
                // する代わりに、heap上の文字列を返す。
                format!("{}, {}", head, tail.stringify())
            },
            Nil => {
                format!("Nil")
            },
        }
    }
}

fn main() {
    // Create an empty linked list
    // 空の連結リストを作成
    let mut list = List::new();

    // Prepend some elements
    // 要素を追加
    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);

    // Show the final state of the list
    // 追加後の状態を表示
    println!("linked list has length: {}", list.len());
    println!("{}", list.stringify());
}
```

### See also:

<!--
[`Box`][box] and [methods][methods]
-->
[ボックス(`Box`)][box], [メソッド][methods]

[box]: ../../std/box.md
[methods]: ../../fn/methods.md
