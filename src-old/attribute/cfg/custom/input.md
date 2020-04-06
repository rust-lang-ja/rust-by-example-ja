<!-- Some conditionals like `target_os` are implicitly provided by `rustc`, but
custom conditionals must be passed to `rustc` using the `--cfg` flag. -->
`target_os`のように、いくつかの条件分岐は`rustc`が暗黙のうちに提供しています。条件を独自に追加する場合には`--cfg`フラグを用いて`rustc`に伝える必要があります。

{custom.rs}

<!-- Without the custom `cfg` flag: -->
`cfg`フラグがない場合:

{custom.out}

<!-- With the custom `cfg` flag: -->
`cfg`フラグがある場合:

```
$ rustc --cfg some_condition custom.rs && ./custom
condition met!
```
