<!--
# Error handling
-->
# エラーハンドリング

<!--
Error handling is the process of handling the possibility of failure. For
example, failing to read a file and then continuing to use that *bad* input
would clearly be problematic. Noticing and explicitly managing those errors
saves the rest of the program from various pitfalls.
-->
FIXME_EN: Error handling is the process of handling the possibility of failure. For
FIXME_EN: example, failing to read a file and then continuing to use that *bad* input
FIXME_EN: would clearly be problematic. Error handling allows us to notice and handle
FIXME_EN: those errors in an explicit fashion, saving the rest of the program from
FIXME_EN: potential issues.
FIXME_JA: エラーハンドリングとは失敗の起きる可能性を扱うプロセスのことです。例えば、ファイルを読み込むのに失敗した際、その*誤った*インプットを使い続けるのは明らかに問題です。エラーハンドリングによって、そのようなエラーに気づき、よりきれいな方法で扱い、残りのプログラムに問題が波及することを防ぐことができるようになります。

There are various ways to deal with errors in Rust, which are described in the
following subchapters. They all have more or less subtle differences and different
use cases. As a rule of thumb:

An explicit `panic` is mainly useful for tests and dealing with unrecoverable errors.
For prototyping it can be useful, for example when dealing with functions that
haven't been implemented yet, but in those cases the more descriptive `unimplemented`
is better. In tests `panic` is a reasonable way to explicitly fail.

The `Option` type is for when a value is optional or when the lack of a value is
not an error condition. For example the parent of a directory - `/` and `C:` don't
have one. When dealing with `Option`s, `unwrap` is fine for prototyping and cases
where it's absolutely certain that there is guaranteed to be a value. However `expect`
is more useful since it lets you specify an error message in case something goes
wrong anyway.

When there is a chance that things do go wrong and the caller has to deal with the
problem, use `Result`. You can `unwrap` and `expect` them as well (please don't
do that unless it's a test or quick prototype).

For a more rigorous discussion of error handling, refer to the error
handling section in the [official book][book].

> 訳注: こちらのQiitaの日本語記事も参考になります。[「RustでOption値やResult値を上手に扱う」][qiita]

[book]: https://doc.rust-lang.org/book/ch09-00-error-handling.html
[qiita]: http://qiita.com/tatsuya6502/items/cd41599291e2e5f38a4a
