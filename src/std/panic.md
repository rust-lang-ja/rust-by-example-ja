# `panic!`

<!--
The `panic!` macro can be used to generate a panic and start unwinding
its stack. While unwinding, the runtime will take care of freeing all the
resources *owned* by the thread by calling the destructor of all its objects.
-->
`panic!`マクロはパニックを生成し、スタックの巻き戻しを開始するために使用することができます。巻き戻しの間、ランタイムは、（訳注: panicを起こした）スレッドが **所有権を持つ** 全ての資源のデストラクタを呼び出し、メモリ上から解放します。

<!--
Since we are dealing with programs with only one thread, `panic!` will cause the
program to report the panic message and exit.
-->
今回はシングルスレッドのプログラムを実行しているので、`panic!`はプログラムにパニックメッセージを表示させ、exitします。

```rust,editable,ignore,mdbook-runnable
// Re-implementation of integer division (/)
// 整数の除法(/)の再実装
fn division(dividend: i32, divisor: i32) -> i32 {
    if divisor == 0 {
        // Division by zero triggers a panic
        // ゼロによる除算はパニックを引き起こす
        panic!("division by zero");
    } else {
        dividend / divisor
    }
}

// The `main` task
// `main`のタスク
fn main() {
    // Heap allocated integer
    // ヒープ上の整数
    let _x = Box::new(0i32);

    // This operation will trigger a task failure
    // このオペレーションはタスクの失敗を引き起こす
    division(3, 0);

    println!("This point won't be reached!");

    // `_x` should get destroyed at this point
    // `_x`はここに到達する前に破棄される。
}
```

<!--
Let's check that `panic!` doesn't leak memory.
-->
`panic!`がメモリリークを引き起こさないことを確認しましょう。



```shell
$ rustc panic.rs && valgrind ./panic
==4401== Memcheck, a memory error detector
==4401== Copyright (C) 2002-2013, and GNU GPL'd, by Julian Seward et al.
==4401== Using Valgrind-3.10.0.SVN and LibVEX; rerun with -h for copyright info
==4401== Command: ./panic
==4401== 
thread '<main>' panicked at 'division by zero', panic.rs:5
==4401== 
==4401== HEAP SUMMARY:
==4401==     in use at exit: 0 bytes in 0 blocks
==4401==   total heap usage: 18 allocs, 18 frees, 1,648 bytes allocated
==4401== 
==4401== All heap blocks were freed -- no leaks are possible
==4401== 
==4401== For counts of detected and suppressed errors, rerun with: -v
==4401== ERROR SUMMARY: 0 errors from 0 contexts (suppressed: 0 from 0)
```
