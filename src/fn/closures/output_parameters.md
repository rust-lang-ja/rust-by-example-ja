<!--
# As output parameters
-->
# クロージャを返す関数

<!--
Closures as input parameters are possible, so returning closures as
output parameters should also be possible. However, anonymous
closure types are, by definition, unknown, so we have to use
`impl Trait` to return them.
-->

クロージャを引数のパラメータとして用いることができるのと同様に、クロージャを戻り値として返すことも可能です。しかし無名のクロージャの型はその定義上、不明であるため、クロージャを返すためには`impl Trait`を使用する必要があります。

<!--
The valid traits for returning a closure are:
-->

クロージャを返すために有効なトレイトは下記の通りです。

* `Fn`
* `FnMut`
* `FnOnce`

<!--
Beyond this, the `move` keyword must be used, which signals that all captures
occur by value. This is required because any captures by reference would be
dropped as soon as the function exited, leaving invalid references in the
closure.
-->

更に、`move`というキーワードを使用し、全ての捕捉が値でおこなわれることを明示しなければなりません。
これは、関数を抜けると同時に参照による捕捉がドロップされ、無効な参照がクロージャに残ってしまうのを防ぐためです。

```rust,editable
fn create_fn() -> impl Fn() {
    let text = "Fn".to_owned();

    move || println!("This is a: {}", text)
}

fn create_fnmut() -> impl FnMut() {
    let text = "FnMut".to_owned();

    move || println!("This is a: {}", text)
}

fn create_fnonce() -> impl FnOnce() {
    let text = "FnOnce".to_owned();

    move || println!("This is a: {}", text)
}

fn main() {
    let fn_plain = create_fn();
    let mut fn_mut = create_fnmut();
    let fn_once = create_fnonce();

    fn_plain();
    fn_mut();
    fn_once();
}
```

<!--
### See also:
-->
### 参照

<!--
[`Fn`][fn], [`FnMut`][fnmut], [Generics][generics] and [impl Trait][impltrait].
-->
[`Fn`][fn], [`FnMut`][fnmut], [ジェネリクス][generics], [impl Trait][impltrait].

[fn]: https://doc.rust-lang.org/std/ops/trait.Fn.html
[fnmut]: https://doc.rust-lang.org/std/ops/trait.FnMut.html
[generics]: ../../generics.md
[impltrait]: ../../trait/impl_trait.md
