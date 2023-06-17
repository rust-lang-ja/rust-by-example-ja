<!--
# Returning Traits with `dyn`
-->
# `dyn`を利用してトレイトをリターンする

<!--
The Rust compiler needs to know how much space every function's return type requires. This means all your functions have to return a concrete type. Unlike other languages, if you have a trait like `Animal`, you can't write a function that returns `Animal`, because its different implementations will need different amounts of memory. 
-->
Rustのコンパイラはあらゆる関数のリターン型に必要なスペースを知っておく必要があります。
つまり、すべての関数は具体的な型をリターンする必要があるのです。
他の言語と違って、`Animal`のようなトレイトがある場合に、`Animal`をリターンする関数を書くことはできません。
なぜなら、そのトレイトの異なる実装はそれぞれ別の量のメモリを必要とするからです。

<!--
However, there's an easy workaround. Instead of returning a trait object directly, our functions return a `Box` which _contains_ some `Animal`. A `box` is just a reference to some memory in the heap. Because a reference has a statically-known size, and the compiler can guarantee it points to a heap-allocated `Animal`, we can return a trait from our function!
-->
しかし、簡単な回避策があります。
直接トレイトオブジェクトをリターンする代わりに、`Animal`を _含む_ `Box`をリターンするのです。
`Box`はヒープ中のメモリへの単なる参照です。
参照のサイズは静的に知ることができ、コンパイラは参照がヒープに割り当てられた`Animal`を指していると保証できるので、私たちは関数からトレイトをリターンできます。

<!--
Rust tries to be as explicit as possible whenever it allocates memory on the heap. So if your function returns a pointer-to-trait-on-heap in this way, you need to write the return type with the `dyn` keyword, e.g. `Box<dyn Animal>`.
-->
ヒープにメモリを割り当てる際、Rustは可能な限り明示的であろうとします。
なので、もしあなたの関数がヒープ上のトレイトへのポインタをリターンする場合、例えば`Box<dyn Animal>`のように、リターン型に`dyn`キーワードをつける必要があります。

```rust,editable
struct Sheep {}
struct Cow {}

trait Animal {
    // Instance method signature
    // インスタンスのメソッドシグネチャ
    fn noise(&self) -> &'static str;
}

// Implement the `Animal` trait for `Sheep`.
// `Sheep`に`Animal`トレイトを実装する。
impl Animal for Sheep {
    fn noise(&self) -> &'static str {
        "baaaaah!"
    }
}

// Implement the `Animal` trait for `Cow`.
// `Cow`に`Animal`トレイトを実装する。
impl Animal for Cow {
    fn noise(&self) -> &'static str {
        "moooooo!"
    }
}

// Returns some struct that implements Animal, but we don't know which one at compile time.
// Animalを実装した何らかの構造体をリターンする。
// ただし、コンパイル時にはどの実装か分からない。
fn random_animal(random_number: f64) -> Box<dyn Animal> {
    if random_number < 0.5 {
        Box::new(Sheep {})
    } else {
        Box::new(Cow {})
    }
}

fn main() {
    let random_number = 0.234;
    let animal = random_animal(random_number);
    println!("You've randomly chosen an animal, and it says {}", animal.noise());
}

```