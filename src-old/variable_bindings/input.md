<!-- Rust provides type safety via static typing. Variable bindings can be type
annotated when declared. However, in most cases, the compiler will be able
to infer the type of the variable from the context, heavily reducing the
annotation burden. -->
Rustは静的(`static`)な型付けゆえに型安全です。変数は宣言時に型を明示できます。とはいえたいていの場合は、コンパイラは変数の型をコンテキストから推測することができますので、常に苦労して明示する必要があるわけではありません。


<!-- Values (like literals) can be bound to variables, using the `let` binding.
-->
値は（リテラルに似て）`let`を用いて変数に束縛されることができる。

{variable_bindings.play}
