<!--
# constants
-->
# 定数

<!--
Rust has two different types of constants which can be declared in any scope
including global. Both require explicit type annotation:
-->
Rustには2種類の定数があり、いずれもグローバルスコープを含む任意のスコープで宣言することができます。また、いずれも型を明示しなくてはなりません。

<!--
* `const`: An unchangeable value (the common case).
* `static`: A possibly `mut`able variable with [`'static`][static] lifetime.
  The static lifetime is inferred and does not have to be specified.
  Accessing or modifying a mutable static variable is [`unsafe`][unsafe].
-->
* `const`: 不変の値（通常はこちらを使用する）
* `static`: [スタティックな][static]ライフタイムを持つミュータブル(`mut`)な値
  The static lifetime is inferred and does not have to be specified.
  Accessing or modifying a mutable static variable is [`unsafe`][unsafe].

```rust,editable,ignore,mdbook-runnable
// Globals are declared outside all other scopes.
// グローバル変数はあらゆるスコープの外で宣言します
static LANGUAGE: &str = "Rust";
const THRESHOLD: i32 = 10;

fn is_big(n: i32) -> bool {
    // Access constant in some function
    // 関数内から定数を参照
    n > THRESHOLD
}

fn main() {
    let n = 16;

    // Access constant in the main thread
    // main 関数の中から定数を参照
    println!("This is {}", LANGUAGE);
    println!("The threshold is {}", THRESHOLD);
    println!("{} is {}", n, if is_big(n) { "big" } else { "small" });

    // Error! Cannot modify a `const`.
    // エラー!`const`は変更できません。
    THRESHOLD = 5;
    // FIXME ^ Comment out this line
    // FIXME ^ この行をコメントアウトしましょう
}
```

<!--
### See also:
-->
### 参照

<!--
[The `const`/`static` RFC](
https://github.com/rust-lang/rfcs/blob/master/text/0246-const-vs-static.md),
[`'static` lifetime][static]
-->
[`const` 及び `static` の RFC](
https://github.com/rust-lang/rfcs/blob/master/text/0246-const-vs-static.md),
[`'static` ライフタイム][static]

[static]: ../scope/lifetime/static_lifetime.md
[unsafe]: ../unsafe.md
