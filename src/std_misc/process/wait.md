<!-- When a `Process` goes out of scope, its `drop` method will *wait* until the
child process finishes before releasing the resource. -->
`Process`がスコープから抜けても、その子プロセスが実行を終了するまで、`drop`メソッドはリソースの開放を**行いません**

``` rust
use std::process::Command;

fn main() {
    let _process = Command::new("sleep").arg("5").spawn();

    println!("reached end of main");
}

```

``` bash
$ rustc wait.rs && ./wait
reached end of main
# `wait` keeps running for 5 seconds
# `sleep 5` command ends, and then our `wait` program finishes
```
