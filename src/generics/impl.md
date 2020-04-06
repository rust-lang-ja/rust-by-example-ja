<!--
# Implementation
-->
# メソッド

<!--
Similar to functions, implementations require care to remain generic.
-->
関数と同様、`impl`でメソッドを実装する際にもジェネリック型特有の記法が必要です。

```rust
struct S; // Concrete type `S`
          // 具象型`S`
struct GenericVal<T>(T); // Generic type `GenericVal`
                         // ジェネリック型`GenericVal`

// impl of GenericVal where we explicitly specify type parameters:
// 型パラメータを指定したうえで、GenericValにメソッドを実装
impl GenericVal<f32> {} // Specify `f32`
                        // `f32`の場合のメソッド
impl GenericVal<S> {} // Specify `S` as defined above
                      // 上で定義した`S`への実装

// `<T>` Must precede the type to remain generic
// ジェネリック型のまま扱うには`<T>`が先に来る必要がある。
impl<T> GenericVal<T> {}
```

```rust,editable
struct Val {
    val: f64,
}

struct GenVal<T> {
    gen_val: T,
}

// impl of Val
// Valに対してimpl
impl Val {
    fn value(&self) -> &f64 {
        &self.val
    }
}

// impl of GenVal for a generic type `T`
// ジェネリック型`T`の場合のメソッドをGenValに対して実装
impl<T> GenVal<T> {
    fn value(&self) -> &T {
        &self.gen_val
    }
}

fn main() {
    let x = Val { val: 3.0 };
    let y = GenVal { gen_val: 3i32 };

    println!("{}, {}", x.value(), y.value());
}
```

### See also:

<!--
[functions returning references][fn], [`impl`][methods], and [`struct`][structs]
-->
[参照を返す関数][fn], [`impl`][methods], [`struct`][structs]


[fn]: ../scope/lifetime/fn.md
[methods]: ../fn/methods.md
[specialization_plans]: https://blog.rust-lang.org/2015/05/11/traits.html#the-future
[structs]: ../custom_types/structs.md
