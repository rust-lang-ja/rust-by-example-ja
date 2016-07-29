<!-- The same set of rules can be applied to functions: a type `T` becomes
generic when preceded by `<T>`. -->
「型`T`はその前に`<T>`があるとジェネリック型になる」というルールは関数に対しても当てはまります。

<!-- Using generic functions sometimes requires explicitly specifying type
parameters. This may be if the function is called where the return type
is generic, or if the compiler doesn't have enough information to infer
the necessary type parameters. -->
ジェネリック関数を使用する際、以下の様な場合には型パラメータを明示する必要があります。

* 返り値がジェネリック型である場合。
* コンパイラが型パラメータを推論するのに十分な情報がない場合

<!-- A function call with explicitly specified type parameters looks like:
`fun::<A, B, ...>()`. -->
型パラメータを明示したうえでの関数呼び出しの構文は`fun::<A, B, ...>()`のようになります。

{fn.play}

### See also:

[関数][fn] and [構造体][structs]

[fn]:../fn.html
[structs]: ../custom_types/structs.html
