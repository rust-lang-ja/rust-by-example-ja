<!--
# Raw identifiers
-->
# 生識別子

<!--
Rust, like many programming languages, has the concept of "keywords".
These identifiers mean something to the language, and so you cannot use them in
places like variable names, function names, and other places.
Raw identifiers let you use keywords where they would not normally be allowed.
This is particularly useful when Rust introduces new keywords, and a library
using an older edition of Rust has a variable or function with the same name
as a keyword introduced in a newer edition.
-->
Rustは多くのプログラミング言語と同様に、「キーワード」の概念を持っています。
これらの識別子は言語にとって何かしらの意味を持ちますので、変数名や関数名、その他の場所で使用することはできません。
生識別子は、通常は許されない状況でキーワードを使用することを可能にします。
これは、新しいキーワードを導入したRustと、古いエディションのRustを使用したライブラリが同じ名前の変数や関数を持つ場合に特に便利です。

<!--
For example, consider a crate `foo` compiled with the 2015 edition of Rust that
exports a function named `try`. This keyword is reserved for a new feature in
the 2018 edition, so without raw identifiers, we would have no way to name the
function.
-->
例えば、2015年エディションのRustでコンパイルされたクレート`foo`が`try`という名前の関数をエクスポートしているとします。
このキーワードは2018年エディションで新機能として予約されていますので、生識別子がなければ、この関数を名付ける方法がありません。


```rust,ignore
extern crate foo;

fn main() {
    foo::try();
}
```

<!--
You'll get this error:
-->
このエラーが出ます:

```text
error: expected identifier, found keyword `try`
 --> src/main.rs:4:4
  |
4 | foo::try();
  |      ^^^ expected identifier, found keyword
```

<!--
You can write this with a raw identifier:
-->
これは生識別子を使って書くことができます:

```rust,ignore
extern crate foo;

fn main() {
    foo::r#try();
}
```
