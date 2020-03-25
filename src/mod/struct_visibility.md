<!--
# Struct visibility
-->
# 構造体の場合

<!--
Structs have an extra level of visibility with their fields. The visibility 
defaults to private, and can be overridden with the `pub` modifier. This 
visibility only matters when a struct is accessed from outside the module 
where it is defined, and has the goal of hiding information (encapsulation).
-->
構造体はそれ自身に加え、フィールドごとにもパブリック・プライベートを設定することができます。デフォルトではプライベートですが、`pub`宣言をすることで、フィールドをパブリックにすることができます。これは、構造体がモジュールの外から参照される時に限り意味のあるもので、情報の隠蔽（カプセル化）を達成するための機能です。

```rust,editable
mod my {
    // A public struct with a public field of generic type `T`
    // パブリックなフィールド`T`(ジェネリック型)を持つパブリックな構造体
    pub struct OpenBox<T> {
        pub contents: T,
    }

    // A public struct with a private field of generic type `T`
    // プライベートなフィールド`T`（ジェネリック型）を持つパブリックな構造体
    #[allow(dead_code)]
    pub struct ClosedBox<T> {
        contents: T,
    }

    impl<T> ClosedBox<T> {
        // A public constructor method
        // パブリックなコンストラクタメソッドを持つ構造体
        pub fn new(contents: T) -> ClosedBox<T> {
            ClosedBox {
                contents: contents,
            }
        }
    }
}

fn main() {
    // Public structs with public fields can be constructed as usual
    // パブリックなフィールドを持つパブリックな構造体は、通常通り
    // インスタンス化できる。
    let open_box = my::OpenBox { contents: "public information" };

    // and their fields can be normally accessed.
    // フィールドにも普通にアクセスできる。
    println!("The open box contains: {}", open_box.contents);

    // Public structs with private fields cannot be constructed using field names.
    // Error! `ClosedBox` has private fields
    // プライベートなフィールドを持つ構造体は、インスタンス化する際に
    // フィールド名を指定することができない。
    // エラー!`ClosedBox`にはプライベートな属性が存在します。
    //let closed_box = my::ClosedBox { contents: "classified information" };
    // TODO ^ Try uncommenting this line
    // TODO ^ 試しにここをアンコメントしてみましょう。

    // However, structs with private fields can be created using
    // public constructors
    // そのような場合でも、パブリックなコンストラクタを介して作成
    // することは可能。
    let _closed_box = my::ClosedBox::new("classified information");

    // and the private fields of a public struct cannot be accessed.
    // Error! The `contents` field is private
    // たとえパブリックな構造体でも、プライベートなフィールドには
    // アクセス出来ない。
    // エラー!`contents`フィールドはプライベートです。
    //println!("The closed box contains: {}", _closed_box.contents);
    // TODO ^ Try uncommenting this line
    // TODO ^ ここをアンコメントしてみましょう。
}
```

### See also:

<!--
[generics][generics] and [methods][methods]
-->
[ジェネリック型][generics], [メソッド][methods]

[generics]: ../generics.md
[methods]: ../fn/methods.md
