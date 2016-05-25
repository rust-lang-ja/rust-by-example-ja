<!-- Macros can use `+` in the argument list to indicate that an argument may
repeat at least once, or `*`, to indicate that the argument may repeat zero or
more times. -->
マクロは引数のリストの中で`+`を使うことができ、そうすることによって、引数が少なくとも1回以上繰り返されるということを示すことができます。同様に`*`の場合は、0以上を示します。

<!-- In the following example, surrounding the matcher with `$(...),+` will
match one or more expression, separated by commas.
Also note that the semicolon is optional on the last case. -->
以下の例では、マッチ対象を `$(...),+`で囲むことにより、カンマで区切られた1つ以上の式文とマッチします。最後のセミコロンは必須ではないことに注目しましょう。

{repeat.play}
