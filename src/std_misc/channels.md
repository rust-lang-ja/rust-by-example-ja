<!--
# Channels
-->
# チャネル

<!--
Rust provides asynchronous `channels` for communication between threads. Channels
allow a unidirectional flow of information between two end-points: the
`Sender` and the `Receiver`.
-->
Rustは、スレッド間のコミュニケーションのために、非同期のチャネル(`channels`)を提供しています。チャネル2つのエンドポイント、すなわち送信者(`Sender`)と受信者(`Receiver`)を介して、情報の一方向への流れを作り出すことを可能にしています。

```rust,editable
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::thread;

static NTHREADS: i32 = 3;

fn main() {
    // Channels have two endpoints: the `Sender<T>` and the `Receiver<T>`,
    // where `T` is the type of the message to be transferred
    // (type annotation is superfluous)
    // チャネルには`Sender<T>`と`Receiver<T>`という2つのエンドポイントがある。
    // ここで、`T`は送信されるメッセージの型である。
    // (型アノテーションは必須ではない。)
    let (tx, rx): (Sender<i32>, Receiver<i32>) = mpsc::channel();
    let mut children = Vec::new();

    for id in 0..NTHREADS {
        // The sender endpoint can be copied
        // 送信者エンドポイントはコピーすることができる。
        let thread_tx = tx.clone();

        // Each thread will send its id via the channel
        // ここでは、それぞれのスレッドが自身のIDを送信している。
        let child = thread::spawn(move || {
            // The thread takes ownership over `thread_tx`
            // Each thread queues a message in the channel
            // スレッドは`thread_tx`の所有権をとり、それぞれのスレッドは
            // メッセージをチャネルにキューイングする。
            thread_tx.send(id).unwrap();

            // Sending is a non-blocking operation, the thread will continue
            // immediately after sending its message
            // 送信はノンブロッキングなオペレーションなので、
            // メッセージを送信した後もすぐに実行を継続する。
            println!("thread {} finished", id);
        });

        children.push(child);
    }

    // Here, all the messages are collected
    // ここで、全てのメッセージが収集される。
    let mut ids = Vec::with_capacity(NTHREADS as usize);
    for _ in 0..NTHREADS {
        // The `recv` method picks a message from the channel
        // `recv` will block the current thread if there are no messages available
        // `recv`メソッドはチャネルからメッセージを取り出す。
        // もし取り出せるメッセージが存在しない場合、`recv`は
        // 現在のスレッドをブロックする。
        ids.push(rx.recv());
    }
    
    // Wait for the threads to complete any remaining work
    for child in children {
        child.join().expect("oops! the child thread panicked");
    }

    // Show the order in which the messages were sent
    // メッセージが送信された順番を表示
    println!("{:?}", ids);
}
```
