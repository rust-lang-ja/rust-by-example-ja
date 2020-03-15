<!--- All types which want to use `std::fmt` formatting `traits` require an --->
<!--- implementation to be printable. Automatic implementations are only provided --->
<!--- for types such as in the `std` library. All others *must* be manually --->
<!--- implemented somehow. --->
`std::fmt`のフォーマット用`トレイト`を使用したい型は、プリント可能である用に実装されている必要があります。`std`ライブラリの型のように自動でプリント可能なものもありますが、他はすべて*手動で実装する必要があります。*

<!--- The `fmt::Debug` `trait` makes this very straightforward. *All* types can --->
<!--- `derive` (automatically create) the `fmt::Debug` implementation. This is --->
<!--- not true for `fmt::Display` which must be manually implemented. --->
`fmt::Debug`という`トレイト`はこれを簡略化します。*すべての*型は`fmt::Debug`の実装を`derive`、（すなわち自動で作成）することができるためです。
`fmt::Display`の場合はやはり手動で実装しなくてはなりません。


```rust
// この構造体は`fmt::Display`、`fmt::Debug`のいずれによっても
// プリントすることができません。
struct UnPrintable(i32);

// `derive`アトリビュートは、
// この構造体を`fmt::Debug`でプリントするための実装を自動で提供します。
#[derive(Debug)]
struct DebugPrintable(i32);
```

<!--- All `std` library types automatically are printable with `{:?}` too: --->
`std`ライブラリの型の場合は、自動的に`{:?}`によりプリント可能になっています。

{debug.play}

<!--- So `fmt::Debug` definitely makes this printable but sacrifices some --->
<!--- elegance. Manually implementing `fmt::Display` will fix that. --->
`fmt::Debug`は確実にプリント可能にしてくれるのですが、一方である種の美しさを犠牲にしています。
`fmt::Display`を手動で実装すればその美しさを取り戻す事ができるでしょう。

### See also

[アトリビュート][attributes], [`derive`][derive], [`std::fmt`][fmt],
and [構造体][structs]

[attributes]: http://doc.rust-lang.org/reference.html#attributes
[derive]: ../../trait/derive.html
[fmt]: http://doc.rust-lang.org/std/fmt/
[structs]: ../../custom_types/structs.html

