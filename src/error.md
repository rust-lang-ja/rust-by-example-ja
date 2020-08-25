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
エラーハンドリングとは失敗の起きる可能性を扱うプロセスのことです。例えば、ファイルを読み込むのに失敗した際、その *誤った* インプットを使い続けるのは明らかに問題です。そのようなエラーを通知して明示的に扱うことで、残りのプログラムに問題が波及することを防ぐことができるようになります。

<!--
There are various ways to deal with errors in Rust, which are described in the
following subchapters. They all have more or less subtle differences and different
use cases. As a rule of thumb:
-->
Rustには、これからこの章で見ていく通り、エラーを処理するための様々な方法が存在します。それらは全て僅かに異なり、ユースケースも異なります。経験則として：

<!--
An explicit `panic` is mainly useful for tests and dealing with unrecoverable errors.
For prototyping it can be useful, for example when dealing with functions that
haven't been implemented yet, but in those cases the more descriptive `unimplemented`
is better. In tests `panic` is a reasonable way to explicitly fail.
-->
明示的な`panic`はテストや復旧不可能なエラーに対して効果的です。プロトタイプにも便利で、例えば未実装の関数を扱う時などに有効ですが、このような場合にはより叙述的な`unimplemented`の方が良いでしょう。テストにおいては`panic`は明示的にテストを失敗させるための良い手法になるでしょう。

<!--
The `Option` type is for when a value is optional or when the lack of a value is
not an error condition. For example the parent of a directory - `/` and `C:` don't
have one. When dealing with `Option`s, `unwrap` is fine for prototyping and cases
where it's absolutely certain that there is guaranteed to be a value. However `expect`
is more useful since it lets you specify an error message in case something goes
wrong anyway.
-->
`Option`型は値があるとは限らない場合や、値が無いことがエラーの条件とならない場合に有効です。例えば親ディレクトリ（`/`や`C:`はそれを持ちません）などです。`Option`を扱う際は、`unwrap`がプロトタイプや値が確実に存在することが約束されるケースに使えます。しかし、`expect`の方が何かが上手くいかなかった際にエラーメッセージを指定することができるため、より便利でしょう。

<!--
When there is a chance that things do go wrong and the caller has to deal with the
problem, use `Result`. You can `unwrap` and `expect` them as well (please don't
do that unless it's a test or quick prototype).
-->
何かが上手くいかない可能性があったり、呼び出し元が問題を処理しなければならない時は、`Result`を使いましょう。`unwrap`や`expect`を実行することもできます（テストや短期的なプロトタイプ以外では使わないでください）。

<!--
For a more rigorous discussion of error handling, refer to the error
handling section in the [official book][book].
-->
より詳細なエラーハンドリングに関する議論については、[オフィシャルブック][book]の該当の章を参考にしてください。

> 訳注: こちらのQiitaの日本語記事も参考になります。[「RustでOption値やResult値を上手に扱う」][qiita]

[book]: https://doc.rust-lang.org/book/ch09-00-error-handling.html
[qiita]: http://qiita.com/tatsuya6502/items/cd41599291e2e5f38a4a
