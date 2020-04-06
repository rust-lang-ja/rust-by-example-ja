<!-- Similar to functions, implementations require care to remain generic. -->
関数と同様、`impl`でメソッドを実装する際にもジェネリック型特有の記法が必要です。


```rust
struct S; // 具象型`S`
struct GenericVal<T>(T,); // ジェネリック型`GenericVal`

// 型パラメータを指定したうえで、GenericValにメソッドを実装
impl GenericVal<f32> {} // `f32`の場合のメソッド
impl GenericVal<S> {} // 上で定義した`S`への実装

// ジェネリック型のまま扱うには`<T>`が先に来る必要がある。
impl <T> GenericVal<T> {}
```

{impl.play}

### See also:

[参照を返す関数][fn], [`impl`][methods], [`struct`][structs]


[fn]: /scope/lifetime/fn.html
[methods]: /fn/methods.html
[specialization_plans]: http://blog.rust-lang.org/2015/05/11/traits.html#the-future
[structs]: /custom_types/structs.html
