# `cfg`

<!--
Configuration conditional checks are possible through two different operators:
-->
環境に応じたコンパイルをするには2種類の方法があります。

<!--
* the `cfg` attribute: `#[cfg(...)]` in attribute position
* the `cfg!` macro: `cfg!(...)` in boolean expressions
-->
* `cfg`アトリビュート: `#[cfg(...)]`をアトリビュートとして使用する。
* `cfg!`マクロ: `cfg!(...)`をブーリアンとして評価する。

<!--
While the former enables conditional compilation, the latter conditionally
evaluates to `true` or `false` literals allowing for checks at run-time. Both
utilize identical argument syntax.
-->
前者は条件付きコンパイルを行いますが、後者は`true`または`false`リテラルに評価され実行時にチェックすることが可能です。
いずれの場合も適切なシンタックスで記述する必要があります。

```rust,editable
// This function only gets compiled if the target OS is linux
// この関数はターゲットOSがLinuxの時のみコンパイルされる。
#[cfg(target_os = "linux")]
fn are_you_on_linux() {
    println!("You are running linux!");
}

// And this function only gets compiled if the target OS is *not* linux
// そしてこの関数はターゲットOSがLinux *ではない* ときのみコンパイルされる。
#[cfg(not(target_os = "linux"))]
fn are_you_on_linux() {
    println!("You are *not* running linux!");
}

fn main() {
    are_you_on_linux();

    println!("Are you sure?");
    if cfg!(target_os = "linux") {
        println!("Yes. It's definitely linux!");
    } else {
        println!("Yes. It's definitely *not* linux!");
    }
}
```

<!--
### See also:
-->
### 参照

<!--
[the reference][ref], [`cfg!`][cfg], and [macros][macros].
-->
[参照(`reference`)][ref], [`cfg!`][cfg], [マクロ][macros].

[cfg]: https://doc.rust-lang.org/std/macro.cfg!.html
[macros]: ../macros.md
[ref]: https://doc.rust-lang.org/reference/attributes.html#conditional-compilation
