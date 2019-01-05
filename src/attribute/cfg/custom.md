<!-- Some conditionals like `target_os` are implicitly provided by `rustc`, but
custom conditionals must be passed to `rustc` using the `--cfg` flag. -->
`target_os`のように、いくつかの条件分岐は`rustc`が暗黙のうちに提供しています。条件を独自に追加する場合には`--cfg`フラグを用いて`rustc`に伝える必要があります。

``` rust,ignore
#[cfg(some_condition)]
fn conditional_function() {
    println!("condition met!")
}

fn main() {
    conditional_function();
}

```

<!-- Without the custom `cfg` flag: -->
`cfg`フラグがない場合:

``` bash
$ rustc custom.rs && ./custom
No such file or directory (os error 2)
```


<!-- With the custom `cfg` flag: -->
`cfg`フラグがある場合:

``` bash
$ rustc --cfg some_condition custom.rs && ./custom
condition met!
```
