<!-- Rust provides no implicit type conversion (coercion) between primitive types.
But, explicit type conversion (casting) can be performed using the `as` keyword. -->
Rustはプリミティブ型における強制的な型変換を暗黙に行うことはありません。しかし明示的に行うこと（`casting`）は可能です。その場合`as`キーワードを使用します。

<!-- Rules for converting between integral types follow C conventions generally,
except in cases where C has undefined behavior. The behavior of all casts
between integral types is well defined in Rust. -->
整数型から整数型へ型変換する場合、C言語で可能なケースの場合はC言語と同じです。
C言語で未定義の場合の挙動も、Rustでは完全に定義されています。


{cast.play}
