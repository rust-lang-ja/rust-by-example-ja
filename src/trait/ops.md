<!-- In Rust, many of the operators can be overloaded via traits. That is, some operators can
be used to accomplish different tasks based on their input arguments. This is possible
because operators are syntactic sugar for method calls. For example, the `+` operator in
`a + b` calls the `add` method (as in `a.add(b)`). This `add` method is part of the `Add`
trait. Hence, the `+` operator can be used by any implementor of the `Add` trait. -->
Rustでは、多くの演算子はトレイトによってオーバーロードすることができます。つまり、一部の演算子は引数となる値の型に応じて異なる役割を果たすことができるということです。これが可能なのは、演算子が実際にはメソッド呼び出しの糖衣構文にすぎないからです。例えば`a + b`における`+`演算子は`add`メソッドを(`a.add(b)`の形で)呼び出します。この`add`メソッドは`Add`トレイトの一部です。それ故、`+`は`Add`トレイトを実装している全ての型に対して有効なのです。

<!-- A list of the traits, such as `Add`, that overload operators are available [here][ops]. -->
`Add`などの、演算子をオーバーロードするトレイトの一覧は[ここ][ops]にあります。

``` rust,editable
use std::ops;

struct Foo;
struct Bar;

#[derive(Debug)]
struct FooBar;

#[derive(Debug)]
struct BarFoo;

// `std::ops::Add`トレイトは`+`の振る舞いを規定するために使用される
// ここでは`Foo`に対して`Add<Bar>`を実装する。これは加算時の右辺が`Bar`型
// の時に呼び出されるトレイト。つまり以下は`Foo + Bar = FooBar`という振る舞いを
// もたらす。
impl ops::Add<Bar> for Foo {
    type Output = FooBar;

    fn add(self, _rhs: Bar) -> FooBar {
        println!("> Foo.add(Bar) was called");

        FooBar
    }
}

// 型を反転することで、非可換の加算を実装できる。ここでは`Bar`に対して
// `Add<Foo>`を実装する。これは加算時の右辺が`Foo`型の時に呼び出されるメソッド。
// つまり以下は`Bar + Foo = BarFoo`という結果をもたらす。
impl ops::Add<Foo> for Bar {
    type Output = BarFoo;

    fn add(self, _rhs: Foo) -> BarFoo {
        println!("> Bar.add(Foo) was called");

        BarFoo
    }
}

fn main() {
    println!("Foo + Bar = {:?}", Foo + Bar);
    println!("Bar + Foo = {:?}", Bar + Foo);
}

```

###See Also

[Add][add], [構文の索引][syntax]

[add]: http://doc.rust-lang.org/core/ops/trait.Add.html
[ops]: http://doc.rust-lang.org/core/ops/
[syntax]: https://rust-lang-ja.github.io/the-rust-programming-language-ja/1.6/book/syntax-index.html
