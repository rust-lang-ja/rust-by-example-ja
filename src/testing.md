<!--
# Testing
-->
# テスト

<!--
Rust is a programming language that cares a lot about correctness and it
includes support for writing software tests within the language itself.
-->
Rustはとても正確性を配慮したプログラミング言語であり、ソフトウェアテストを書くためのサポートを言語自身が含んでいます。

<!--
Testing comes in three styles:
-->
テストには3つの種類があります。

<!--
* [Unit][unit] testing.
* [Doc][doc] testing.
* [Integration][integration] testing.
-->
* [単体テスト][unit]
* [ドキュメンテーションテスト][doc]
* [結合テスト][integration]

<!--
Also Rust has support for specifying additional dependencies for tests:
-->
またRustではテストのために追加の依存パッケージを指定することもできます。

* [Dev-dependencies][dev-dependencies]

<!--
## See Also
-->
## 参照

<!--
* [The Book][doc-testing] chapter on testing
* [API Guidelines][doc-nursery] on doc-testing
-->
* [The Book][doc-testing] の自動テストの章
* [API ガイドライン][doc-nursery] のドキュメンテーション

[unit]: testing/unit_testing.md
[doc]: testing/doc_testing.md
[integration]: testing/integration_testing.md
[dev-dependencies]: testing/dev_dependencies.md
[doc-testing]: https://doc.rust-lang.org/book/ch11-00-testing.html
[doc-nursery]: https://rust-lang-nursery.github.io/api-guidelines/documentation.html
