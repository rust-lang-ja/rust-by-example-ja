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
