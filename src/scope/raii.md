# RAII

<!--
Variables in Rust do more than just hold data in the stack: they also *own*
resources, e.g. `Box<T>` owns memory in the heap. Rust enforces [RAII][raii]
(Resource Acquisition Is Initialization), so whenever an object goes out of
scope, its destructor is called and its owned resources are freed.
-->
Rustの変数は単にデータをスタック上に保持するだけのものではありません。例えばヒープメモリを確保する`Box<T>`のように、変数はメモリ上の資源を *保有* する場合もあるのです。Rustは[RAII][raii](Resource Acquisition Is Initialization)を強制するので、オブジェクトがスコープを抜けると、必ずデストラクタが呼び出されてそのオブジェクトが保持していた資源が解放されます。

<!--
This behavior shields against *resource leak* bugs, so you'll never have to
manually free memory or worry about memory leaks again! Here's a quick showcase:
-->
この振る舞いは *リソースリーク* (`resource leak`)バグを防ぐのに役立ちます。手動でメモリを解放したり、メモリリークバグにわずらわされたりすることはなくなるのです！簡単な例で説明しましょう。
```rust,editable
// raii.rs
fn create_box() {
    // Allocate an integer on the heap
    // 整数をヒープ上に確保
    let _box1 = Box::new(3i32);

    // `_box1` is destroyed here, and memory gets freed
    // `_box1`はここで破棄され、メモリは解放される。
}

fn main() {
    // Allocate an integer on the heap
    // 整数をヒープ上に確保
    let _box2 = Box::new(5i32);

    // A nested scope:
    // ネストしたスコープ
    {
        // Allocate an integer on the heap
        // 整数をヒープ上に確保
        let _box3 = Box::new(4i32);

        // `_box3` is destroyed here, and memory gets freed
        // `_box3`はここで破棄され、メモリは解放される。
    }

    // Creating lots of boxes just for fun
    // There's no need to manually free memory!
    // お遊びで大量のボックスを作る。
    // もちろん手動で開放する必要はないよ！
    for _ in 0u32..1_000 {
        create_box();
    }

    // `_box2` is destroyed here, and memory gets freed
    // `_box2`はここで破棄され、メモリは解放される。
}
```

<!--
Of course, we can double check for memory errors using [`valgrind`][valgrind]:
-->
[`valgrind`][valgrind]を用いて、メモリエラーが起きていないか2重チェックすることももちろん可能です。

```shell
$ rustc raii.rs && valgrind ./raii
==26873== Memcheck, a memory error detector
==26873== Copyright (C) 2002-2013, and GNU GPL'd, by Julian Seward et al.
==26873== Using Valgrind-3.9.0 and LibVEX; rerun with -h for copyright info
==26873== Command: ./raii
==26873==
==26873==
==26873== HEAP SUMMARY:
==26873==     in use at exit: 0 bytes in 0 blocks
==26873==   total heap usage: 1,013 allocs, 1,013 frees, 8,696 bytes allocated
==26873==
==26873== All heap blocks were freed -- no leaks are possible
==26873==
==26873== For counts of detected and suppressed errors, rerun with: -v
==26873== ERROR SUMMARY: 0 errors from 0 contexts (suppressed: 2 from 2)
```

<!--
No leaks here!
-->
リークはないみたいですね！

## Destructor

The notion of a destructor in Rust is provided through the [`Drop`] trait. The
destructor is called when the resource goes out of scope. This trait is not
required to be implemented for every type, only implement it for your type if
you require its own destructor logic.

Run the below example to see how the [`Drop`] trait works. When the variable in
the `main` function goes out of scope the custom destructor will be invoked.

```rust,editable
struct ToDrop;

impl Drop for ToDrop {
    fn drop(&mut self) {
        println!("ToDrop is being dropped");
    }
}

fn main() {
    let x = ToDrop;
    println!("Made a ToDrop!");
}
```

<!--
### See also:
-->
### 参照

<!--
[Box][box]
-->
[ボックス][box]

[raii]: https://en.wikipedia.org/wiki/Resource_Acquisition_Is_Initialization
[box]: ../std/box.md
[valgrind]: http://valgrind.org/info/
[`Drop`]: https://doc.rust-lang.org/std/ops/trait.Drop.html
