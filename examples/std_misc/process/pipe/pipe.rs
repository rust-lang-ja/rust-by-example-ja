use std::error::Error;
use std::io::prelude::*;
use std::process::{Command, Stdio};

static PANGRAM: &'static str =
"the quick brown fox jumped over the lazy dog\n";

fn main() {
    // `wc`コマンドを起動する。
    let process = match Command::new("wc")
                                .stdin(Stdio::piped())
                                .stdout(Stdio::piped())
                                .spawn() {
        Err(why) => panic!("couldn't spawn wc: {}", Error::description(&why)),
        Ok(process) => process,
    };

    // `wc`の`stdin`に文字列を書き込む。
    //
    // `stdin`は`Option<ChildStdin>`型を持つが、今回は値を持っていることが
    // 確かなので、いきなり`unwrap`してしまってよい。
    match process.stdin.unwrap().write_all(PANGRAM.as_bytes()) {
        Err(why) => panic!("couldn't write to wc stdin: {}",
                           Error::description(&why)),
        Ok(_) => println!("sent pangram to wc"),
    }

    // `stdin`は上のプロセスコールのあとには有効でないので、`drop`され、
    // パイプはcloseされる。
    // (これは非常に重要です。というのもcloseしないと`wc`は
    // 送った値の処理を開始しないからです。)

    // `stdout`フィールドも`Option<ChildStdout>`型なのでアンラップする必要がある
    let mut s = String::new();
    match process.stdout.unwrap().read_to_string(&mut s) {
        Err(why) => panic!("couldn't read wc stdout: {}",
                           Error::description(&why)),
        Ok(_) => print!("wc responded with:\n{}", s),
    }
}
