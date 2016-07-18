<!-- What if the specific `Result` type is reused many many times? Then quickly it becomes tedious
to write out the full type name. Instead, a generic alias for the specific `Result` may be
defined. -->
特定の`Result`型が何度も何度も使用されると何が起きるでしょう？正解は「型名を書き下すのがすぐに面倒になる」です。このような場合は特定の`Result`のジェネリックエイリアスを定義すると吉です。

{alias.play}

<!-- This is particularly helpful at a module level because all errors found in a specific module
may have the same `Err` type; a single alias succinctly defines *all* module `Results`. This
is so useful that the std library even supplies one: `io::Result` which refers to IO errors. -->
これはモジュールのレベルでは特に役立ちます。というのも特定のモジュールで見つかるエラーは同じ`Err`型(そのモジュール内の**全ての**`Result`を完結に表現する唯一のエイリアス)を持つ可能性があるからです。これは本当に有用なので標準ライブラリ内にもその例があります。IOエラー全般を代表する`io::Result`です。

> 訳注: ピンと来ない方は[Python プログラマーのための Rust 入門](http://qiita.com/t2y/items/434854fab16159a7c0f7)が参考になるかもしれません。

### See also:

[`Result`][result]、[`io::Result`][io_result]

[result]: http://doc.rust-lang.org/std/result/enum.Result.html
[io_result]: http://doc.rust-lang.org/std/io/type.Result.html
