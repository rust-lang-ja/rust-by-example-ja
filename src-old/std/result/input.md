<!-- We've seen that the `Option` enum can be used as a return value from functions
that may fail, where `None` can be returned to indicate failure. However,
sometimes it is important to express *why* an operation failed. To do this we
have the `Result` enum. -->
これまでの例で、失敗する可能性のある関数の返り値として、列挙型`Option`が使用でき、失敗時の返り値には`None`を用いることを見てきました。しかし、時には**なぜ**そのオペレーションが失敗したのかを明示することが重要な場合があります。そのためには`Result`列挙型を使用します。

<!-- The `Result<T, E>` enum has two variants: -->
列挙型`Result<T, E>`は2つの値をとりえます。

<!-- * `Ok(value)` which indicates that the operation succeeded, and wraps the
  `value` returned by the operation. (`value` has type `T`)
* `Err(why)`, which indicates that the operation failed, and wraps `why`,
  which (hopefully) explains the cause of the failure. (`why` has type `E`) -->
* `Ok(value)` ... これはオペレーションが成功したことを意味し、返り値`value`をラップします。(`value`は型`T`を持ちます。)
* `Err(why)` ... これはオペレーションの失敗を意味します。`why`をラップしており、ここには失敗した理由が(必ずではありませんが)書かれています。(`why`の型は`E`です。)

{result.play}
