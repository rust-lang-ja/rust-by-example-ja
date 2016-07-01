<!-- Chaining results using match can get pretty untidy; luckily, the `try!` macro
can be used to make things pretty again. The `try!` macro expands to a match
expression, where the `Err(err)` branch expands to an early `return Err(err)`,
and the `Ok(ok)` branch expands to an `ok` expression. -->
マッチを使用して結果をチェインするのは中々面倒です。幸いなことに、`try!`マクロを使用すればイケてるコードに戻すことができます。`try!`マクロはマッチ構文に展開され、`Err(err)`を返す分岐は早い段階で`return Err(err)`し、`Ok(ok)`を返す分岐は`ok`に展開されます。

{try.play}

<!-- Be sure to check the [documentation][docs],
as there are many methods to map/compose `Result`. -->
[公式ドキュメント][docs]をチェックすることをオススメします。`Result`型を扱う関数や`Result`型のメソッドが多く挙げられています。

[docs]: http://doc.rust-lang.org/std/result/index.html
