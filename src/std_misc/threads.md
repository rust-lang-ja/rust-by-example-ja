<!--
# Threads
-->
# スレッド

<!--
Rust provides a mechanism for spawning native OS threads via the `spawn`
function, the argument of this function is a moving closure.
-->
Rustは`spawn`関数を用いてOSのネイティブスレッドを開始することができます。この関数の引数はmoveクロージャ（訳注: 参照ではなく値を取るクロージャ。　詳しくは[クロージャを返す関数][fn_output]を参照）です。

```rust,editable
use std::thread;

const NTHREADS: u32 = 10;

// This is the `main` thread
// この関数は`main`スレッドで実行される。
fn main() {
    // Make a vector to hold the children which are spawned.
    // spawnされるクロージャを保持するためのベクタ
    let mut children = vec![];

    for i in 0..NTHREADS {
        // Spin up another thread
        // 新しいスレッドを起動
        children.push(thread::spawn(move || {
            println!("this is thread number {}", i);
        }));
    }

    for child in children {
        // Wait for the thread to finish. Returns a result.
        // 子スレッドが終了するのを待ち、結果を返す。
        let _ = child.join();
    }
}
```

<!--
These threads will be scheduled by the OS.
-->
これらのスレッドのスケジューリングはOSによって行われる。

[fn_output]: ../fn/closures/output_parameters.md
