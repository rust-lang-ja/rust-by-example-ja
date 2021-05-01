<!--
# Conversion
-->
# 型変換

<!--
Rust addresses conversion between types by the use of [traits]. The generic
conversions will use the [`From`] and [`Into`] traits. However there are more
specific ones for the more common cases, in particular when converting to and
from `String`s.
-->
Rustは型の変換を[トレイト][traits]を用いて行います。一般的な型変換には[`From`]および[`Into`]トレイトを使用します。しかし、よくあるケースにおいて、特に`String`との相互の型変換では、特殊なトレイトが使用されます。

[traits]: trait.md
[`From`]: https://doc.rust-lang.org/std/convert/trait.From.html
[`Into`]: https://doc.rust-lang.org/std/convert/trait.Into.html
