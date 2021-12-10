<!--
# Conversion
-->
# 型変換

Primitive types can be converted to each other through [casting].

<!--
Rust addresses conversion between custom types (i.e., `struct` and `enum`)
by the use of [traits]. The generic
conversions will use the [`From`] and [`Into`] traits. However there are more
specific ones for the more common cases, in particular when converting to and
from `String`s.
-->
Rustはカスタム型（例えば`struct`や`enum`）間の変換を[トレイト][traits]を用いて行います。ジェネリックな型変換には[`From`]および[`Into`]トレイトを使用します。しかし、よくあるケースにおいて、特に`String`との相互の型変換では、特殊なトレイトが使用されます。

[casting]: types/cast.md
[traits]: trait.md
[`From`]: https://doc.rust-lang.org/std/convert/trait.From.html
[`Into`]: https://doc.rust-lang.org/std/convert/trait.Into.html
