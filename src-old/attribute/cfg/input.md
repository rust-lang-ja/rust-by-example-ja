<!-- Conditional compilation is possible through two different operators:

* the `cfg` attribute: `#[cfg(...)]` in attribute position
* the `cfg!` macro: `cfg!(...)` in boolean expressions

Both utilize identical argument syntax. -->
環境に応じたコンパイルをするには2種類の方法があります。

* `cfg`アトリビュート: `#[cfg(...)]`をアトリビュートとして使用する。
* `cfg!`マクロ: `cfg!(...)`をブーリアンとして評価する。

いずれの場合も適切なシンタックスで記述する必要があります。

{cfg.play}

### See also:

[参照(`reference`)][ref], [`cfg!`][cfg], [マクロ][macros].

[cfg]: http://doc.rust-lang.org/std/macro.cfg!.html
[macros]: ./macros.html
[ref]: http://doc.rust-lang.org/reference.html#conditional-compilation
