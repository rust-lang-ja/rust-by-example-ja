<!-- Of course `trait`s can also be generic. Here we define one which reimplements
the `Drop` `trait` as a generic method to `drop` itself and an input. -->
もちろんトレイトもジェネリクスを活用することができます。ここでは`Drop`トレイトをジェネリックメソッドとして再実装し、自身と引数として受け取った値の両方を`drop`するようなメソッドにします。

{trait.play}

### See also:

[`Drop`][Drop], [構造体(`struct`)][structs], [トレイト(`trait`)][traits]

[Drop]: http://doc.rust-lang.org/std/ops/trait.Drop.html
[structs]: /custom_types/structs.html
[traits]: /trait.html
