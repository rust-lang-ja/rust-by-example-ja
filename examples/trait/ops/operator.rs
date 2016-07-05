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
