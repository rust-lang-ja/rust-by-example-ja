<!--- Rust has two different types of constants which can be declared in any scope --->
<!--- including global. Both require explicit type annotation: --->
Rustには2種類の定数があり、いずれもグローバルスコープを含む任意のスコープで宣言することができます。また、いずれも型を明示しなくてはなりません。

<!--- * `const`: An unchangeable value (the common case). --->
* `const`: 不変の値（通常はこちらを使用する）
<!--- * `static`: A possibly `mut`able variable with [`'static`][static] lifetime. --->
* `static`: [スタティックな][static]ライフタイムを持つミュータブル(`mut`)な値

<!--- One special case is the `"string"` literal. It can be assigned directly to a --->
<!--- `static` variable without modification because its type signature: --->
<!--- `&'static str` has the required lifetime of `'static`. All other reference --->
<!--- types must be specifically annotated so that they fulfill the `'static` --->
<!--- lifetime. This may seem minor though because the required explicit annotation --->
<!--- hides the distinction. --->
特別な例として、文字列(`"string"`)リテラルの場合があります。`&'static str`は`'static`のライフタイムを持つため、文字列は特に注意を払わなくても`static`になります。他の型の場合は必ず`'static`ライフタイムを満たすことを明示しなくてはなりません。いずれにしろ`static`を宣言するときはすべて明示しなくてはならないので、この違いは些細なものと感じられるでしょう。

{constants.play}

### See also:

[`const` 及び `static` の RFC][const vs static]
[`'static` ライフタイム][static]

[static]: ../scope/lifetime/static_lifetime.html
[const vs static]: https://github.com/rust-lang/rfcs/blob/master/text/0246-const-vs-static.md
