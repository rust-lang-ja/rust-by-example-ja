use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::thread;

static NTHREADS: i32 = 3;

fn main() {
    // チャネルには`Sender<T>`と`Receiver<T>`という2つのエンドポイントがある。
    // ここで、`T`は送信されるメッセージの型である。
    // (型アノテーションは必須ではない。)
    let (tx, rx): (Sender<i32>, Receiver<i32>) = mpsc::channel();

    for id in 0..NTHREADS {
        // 送信者エンドポイントはコピーすることができる。
        let thread_tx = tx.clone();

        // ここでは、それぞれのスレッドが自身のIDを送信している。
        thread::spawn(move || {
            // スレッドは`thread_tx`の所有権をとり、それぞれのスレッドは
            // メッセージをチャネルにキューイングする。
            thread_tx.send(id).unwrap();

            // 送信はノンブロッキングなオペレーションなので、
            // メッセージを送信した後もすぐに実行を継続する。
            println!("thread {} finished", id);
        });
    }

    // ここで、全てのメッセージが収集される。
    let mut ids = Vec::with_capacity(NTHREADS as usize);
    for _ in 0..NTHREADS {
        // `recv`メソッドはチャネルからメッセージを取り出す。
        // もし取り出せるメッセージが存在しない場合、`recv`は
        // 現在のスレッドをブロックする。
        ids.push(rx.recv());
    }

    // メッセージが送信された順番を表示
    println!("{:?}", ids);
}
