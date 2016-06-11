struct Val (f64,);
struct GenVal<T>(T,);

// Valに対してimpl
impl Val {
    fn value(&self) -> &f64 { &self.0 }
}

// ジェネリック型`T`の場合のメソッドをGenValに対して実装
impl <T> GenVal<T> {
    fn value(&self) -> &T { &self.0 }
}

fn main() {
    let x = Val(3.0);
    let y = GenVal(3i32);

    println!("{}, {}", x.value(), y.value());
}

