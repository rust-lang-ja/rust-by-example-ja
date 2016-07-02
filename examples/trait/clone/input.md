<!-- When dealing with resources, the default behavior is to transfer them during
assignments or function calls. However, sometimes we need to make a
copy of the resource as well. -->
メモリ上の資源を扱う際、変数束縛や関数呼び出しを介して移動させるのがデフォルトの挙動です。しかしながら、場合によっては資源のコピーを作るのが適切なこともあります。

<!-- The [`Clone`][clone] trait helps us do exactly this. Most commonly, we can
use the `.clone()` method defined by the `Clone` trait. -->
[`Clone`][clone]トレイトはまさにこのためにあります。普通は`Clone`トレイトで定義されている`.clone()`を用います。

{clone.play}

[clone]: https://doc.rust-lang.org/std/clone/trait.Clone.html
