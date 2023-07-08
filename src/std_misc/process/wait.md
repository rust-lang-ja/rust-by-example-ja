<!--
# Wait
-->
# dropの延期

<!--
If you'd like to wait for a `process::Child` to finish, you must call
`Child::wait`, which will return a `process::ExitStatus`.
-->
`process::Child`が終了するのを待ちたい場合は、
`process::ExitStatus`を返す`Child::wait`を呼び出さなくてはなりません。

```rust,ignore
use std::process::Command;

fn main() {
    let mut child = Command::new("sleep").arg("5").spawn().unwrap();
    let _result = child.wait().unwrap();

    println!("reached end of main");
}
```

```bash
$ rustc wait.rs && ./wait
# `wait` keeps running for 5 seconds until the `sleep 5` command finishes
# `wait`は`sleep 5`コマンドが終了するまで５秒間実行され続ける。
reached end of main
```
