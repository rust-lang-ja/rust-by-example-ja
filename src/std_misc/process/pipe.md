<!--
# Pipes
-->
# パイプ

<!--
The `std::Child` struct represents a running child process, and exposes the
`stdin`, `stdout` and `stderr` handles for interaction with the underlying
process via pipes.
-->
`std::Child`構造体は実行中の子プロセスを表します。`stdin`、`stdout`、`stderr`を介して表面化のプロセスとのやり取りを仲介します。

```rust,ignore
use std::error::Error;
use std::io::prelude::*;
use std::process::{Command, Stdio};

static PANGRAM: &'static str =
"the quick brown fox jumped over the lazy dog\n";

fn main() {
    // Spawn the `wc` command
    // `wc`コマンドを起動する。
    let process = match Command::new("wc")
                                .stdin(Stdio::piped())
                                .stdout(Stdio::piped())
                                .spawn() {
        Err(why) => panic!("couldn't spawn wc: {}", why.description()),
        Ok(process) => process,
    };

    // Write a string to the `stdin` of `wc`.
    // `wc`の`stdin`に文字列を書き込む。
    //
    // `stdin` has type `Option<ChildStdin>`, but since we know this instance
    // must have one, we can directly `unwrap` it.
    // `stdin`は`Option<ChildStdin>`型を持つが、今回は値を持っていることが
    // 確かなので、いきなり`unwrap`してしまってよい。
    match process.stdin.unwrap().write_all(PANGRAM.as_bytes()) {
        Err(why) => panic!("couldn't write to wc stdin: {}",
                           why.description()),
        Ok(_) => println!("sent pangram to wc"),
    }

    // Because `stdin` does not live after the above calls, it is `drop`ed,
    // and the pipe is closed.
    //
    // This is very important, otherwise `wc` wouldn't start processing the
    // input we just sent.
    // `stdin`は上のプロセスコールのあとには有効でないので、`drop`され、
    // パイプはcloseされる。
    // (これは非常に重要です。というのもcloseしないと`wc`は
    // 送った値の処理を開始しないからです。)

    // The `stdout` field also has type `Option<ChildStdout>` so must be unwrapped.
    // `stdout`フィールドも`Option<ChildStdout>`型なのでアンラップする必要がある
    let mut s = String::new();
    match process.stdout.unwrap().read_to_string(&mut s) {
        Err(why) => panic!("couldn't read wc stdout: {}",
                           why.description()),
        Ok(_) => print!("wc responded with:\n{}", s),
    }
}
```
