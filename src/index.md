<!--
# Rust by Example
-->
# Rust by Example 日本語版

<!--
[Rust][rust] is a modern systems programming language focusing on safety, speed,
and concurrency. It accomplishes these goals by being memory safe without using 
garbage collection.
-->
[Rust][rust] は安全性、速度、並列性にフォーカスした現代的なシステムプログラミング
用のプログラミング言語です。
ガベージコレクション無しでメモリ安全であることが、これを可能にしています。

<!--
Rust by Example (RBE) is a collection of runnable examples that illustrate various Rust
concepts and standard libraries. To get even more out of these examples, don't forget
to [install Rust locally][install] and check out the [official docs][std]. 
Additionally for the curious, you can also [check out the source code for this site][home].
-->
Rust by Example(RBE)はRustの実行可能なサンプルスクリプト集で、ここではRustの様々な
コンセプトと標準ライブラリを紹介していきます。
この例をより活用するためには[Rustをローカルにインストール][install]し、[公式ドキュメント][std]をチェックすることをおすすめします。
興味がある方は[このサイト自体のソース][home]のチェックもどうぞ。

> 訳注:
> 日本語版のソースコードは[こちら][home-ja]にあります。

<!--
Now let's begin!
-->
それでははじめましょう!

<!--
- [Hello World](hello.md) - Start with a traditional Hello World program.
-->
- [Hello World](hello.md) - お決まりのHello Worldプログラムから始めましょう。


<!--
- [Primitives](primitives.md) - Learn about signed integers, unsigned integers and other primitives.
-->
- [基本データ型](primitives.md) - 符号付き整数や符号無し整数、その他の基本データ型について学びましょう。

<!--
- [Custom Types](custom_types.md) - `struct` and `enum`.
-->
- [カスタム型](custom_types.md) - `struct` と `enum`について。

<!--
- [Variable Bindings](variable_bindings.md) - mutable bindings, scope, shadowing.
-->
- [変数バインディング](variable_bindings.md) - ミュータブルな束縛、スコープ、シャドーイングについて。

<!--
- [Types](types.md) - Learn about changing and defining types.
-->
- [Types](types.md) - 型を変更したり定義したりすることを学びましょう。

<!--
- [Conversion](conversion.md)
-->
- [Conversion](conversion.md)

<!--
- [Expressions](expression.md)
-->
- [式](expression.md)

<!--
- [Flow of Control](flow_control.md) - `if`/`else`, `for`, and others.
-->
- [Flow of Control](flow_control.md) - `if`や`else`、`for`など。

<!--
- [Functions](fn.md) - Learn about Methods, Closures and High Order Functions.
-->
- [関数](fn.md) - メソッド、クロージャ、高階関数について。

<!--
- [Modules](mod.md) - Organize code using modules
-->
- [モジュール](mod.md) - プログラムをモジュールを使って整理する

<!--
- [Crates](crates.md) - A crate is a compilation unit in Rust. Learn to create a library.
-->
- [クレート](crates.md) - クレートは、Rustにおいてコンパイルされる単位です。ライブラリの作り方について学びます。

<!--
- [Cargo](cargo.md) - Go through some basic features of the official Rust package management tool.
-->
- [Cargo](cargo.md) - Rustの公式パッケージマネージャの基本的な機能を学びます。

<!--
- [Attributes](attribute.md) - An attribute is metadata applied to some module, crate or item.
-->
- [アトリビュート](attribute.md) - アトリビュートは、モジュールやクレート、要素に適用されるメタデータです。

<!--
- [Generics](generics.md) - Learn about writing a function or data type which can work for multiple types of arguments.
-->
- [ジェネリクス](generics.md) - 様々な型の引数を取れる関数やデータ型を書く方法を学びましょう。

<!--
- [Scoping rules](scope.md) - Scopes play an important part in ownership, borrowing, and lifetimes.
-->
- [スコーピングの規則](scope.md) - スコープは所有権、借用、ライフタイムにおいて重要な役割を果たします。

<!--
- [Traits](trait.md) - A trait is a collection of methods defined for an unknown type: `Self`
-->
- [トレイト](trait.md) - トレイトとは、未知の型`Self`に対して定義された一連のメソッドです。

- [Macros](macros.md)

<!--
- [Error handling](error.md) - Learn Rust way of handling failures.
-->
- [エラーハンドリング](error.md) - 失敗に対処するRust流のやり方を学びましょう。

<!--
- [Std library types](std.md) - Learn about some custom types provided by `std` library.
-->
- [標準ライブラリの型](std.md) - `std`ライブラリによって提供されるいくつかのカスタム型について学びます。

<!--
- [Std misc](std_misc.md) - More custom types for file handling, threads.
-->
- [標準ライブラリのその他](std_misc.md) - ファイルハンドリングのためのその他のカスタム型と、スレッドについて。

<!--
- [Testing](testing.md) - All sorts of testing in Rust.
-->
- [Testing](testing.md) - Rustにおけるテストのすべて。

<!--
- [Unsafe Operations](unsafe.md)
-->
- [安全でない操作](unsafe.md)

<!--
- [Compatibility](compatibility.md)
-->
- [Compatibility](compatibility.md)

<!--
- [Meta](meta.md) - Documentation, Benchmarking.
-->
- [周辺情報](meta.md) - ドキュメント、ベンチマークの方法。


[rust]: https://www.rust-lang.org/
[install]: https://www.rust-lang.org/tools/install
[std]: https://doc.rust-lang.org/std/
[home]: https://github.com/rust-lang/rust-by-example
[home-ja]: https://github.com/rust-lang-ja/rust-by-example-ja
