<!-- When a `Process` goes out of scope, its `drop` method will *wait* until the
child process finishes before releasing the resource. -->
`Process`がスコープから抜けても、その子プロセスが実行を終了するまで、`drop`メソッドはリソースの開放を**行いません**

{wait.rs}

```
$ rustc wait.rs && ./wait
reached end of main
# `wait` keeps running for 5 seconds
# `sleep 5` command ends, and then our `wait` program finishes
```
