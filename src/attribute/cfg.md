<!-- Conditional compilation is possible through two different operators:

* the `cfg` attribute: `#[cfg(...)]` in attribute position
* the `cfg!` macro: `cfg!(...)` in boolean expressions

Both utilize identical argument syntax. -->
環境に応じたコンパイルをするには2種類の方法があります。

* `cfg`アトリビュート: `#[cfg(...)]`をアトリビュートとして使用する。
* `cfg!`マクロ: `cfg!(...)`をブーリアンとして評価する。

いずれの場合も適切なシンタックスで記述する必要があります。

``` rust,editable
// この関数はターゲットOSがLinuxの時のみコンパイルされる。
#[cfg(target_os = "linux")]
fn are_you_on_linux() {
    println!("You are running linux!")
}

// そしてこの関数はターゲットOSがLinux*ではない*ときのみコンパイルされる。
#[cfg(not(target_os = "linux"))]
fn are_you_on_linux() {
    println!("You are *not* running linux!")
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

### See also:

[参照(`reference`)][ref], [`cfg!`][cfg], [マクロ][macros].

[cfg]: http://doc.rust-lang.org/std/macro.cfg!.html
[macros]: ./macros.html
[ref]: http://doc.rust-lang.org/reference.html#conditional-compilation
