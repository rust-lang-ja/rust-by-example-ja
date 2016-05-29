<!-- The `type` statement can be used to give a new name to an existing type. Types
must have `CamelCase` names, or the compiler will raise a warning. The
exception to this rule are the primitive types: `usize`, `f32`, etc. -->
`type`文を使用することで既存の型に新しい名前(`alias`)を付けることができます。その場合、名前は`CamelCase`でなくてはなりません。さもなくばコンパイラがエラーを出します。唯一の例外は`usize`や`f32`のようなプリミティブ型です。

{alias.play}

<!-- The main use of aliases is to reduce typing; for example the `IoResult<T>` type
is an alias for the `Result<T, IoError>` type. -->
このようにエイリアスを付ける一番の理由はタイプ量を減らすことです。例えば`IoResult<T>`型は`Result<T, IoError>`の別名です。


### See also:

[アトリビュート](../attribute.html)
