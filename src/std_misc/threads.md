<!-- Rust provides a mechanism for spawning native OS threads via the `spawn`
function, the argument of this function is a moving closure. -->
Rustは`spawn`関数を用いてOSのネイティブスレッドを開始することができます。この関数の引数はmoveクロージャ(訳注: 参照ではなく値を取るクロージャ。　詳しくは[クロージャを返す関数][fn_output]を参照)です。

``` rust,editable
use std::thread;

static NTHREADS: i32 = 10;

// この関数は`main`スレッドで実行される。
fn main() {
    // spawnされるクロージャを保持するためのベクタ
    let mut children = vec![];

    for i in 0..NTHREADS {
        // 新しいスレッドを起動
        children.push(thread::spawn(move || {
            println!("this is thread number {}", i)
        }));
    }

    for child in children {
        // 子スレッドが終了するのを待ち、結果を返す。
        let _ = child.join();
    }
}

```

<!-- These threads will be scheduled by the OS. -->
これらのスレッドのスケジューリングはOSによって行われる。


[fn_output]: ../fn/closures/output_parameters.html
