<!-- For pointers, a distinction needs to be made between destructuring
and dereferencing as they are different concepts which are used
differently from a language like `C`. -->
Rustのポインタは、`C`のポインタとは異なる概念なので、デストラクトとデリファレンスを同じようなやり方で扱うことはできない

<!-- * Dereferencing uses `*`
 * Destructuring uses `&`, `ref`, and `ref mut` -->
* デリファレンスには`*`を用いる。
* デストラクトには`&`, `ref`, `ref mut`を用いる。

{pointers.play}
