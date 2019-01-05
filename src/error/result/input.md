<!-- Previously, we have used the type `Option` to annotate that absence is a possibility. This
absence sometimes appears as an error, for example when `None` is unwrapped. In the more
general case where there may be multiple failure points for a multitude of different reasons,
an `Option` can be replaced by the more general `Result` type. A `Result<T, E>` has these
variants: -->
ここまでで、値が欠如している可能性を扱うために`Option`を使用する例を見てきました。このように値が存在しないことは、時としてエラーを生じさせることがあります。たとえば`None`をアンラップしようとした場合などがそうです。より一般的に、様々な場所・理由で失敗しうる場合には`Option`はより包括的な`Result`型で置き換えることができます。`Result<T, E>`には以下の様な変種が存在します。

<!-- * `Ok<T>`: An element `T` was found
* `Err<E>`: An error was found with element `E` -->
* `Ok<T>`: 要素`T`が見つかった場合
* `Err<E>`: 要素`E`とともにエラーが見つかった場合

<!-- Similar to `Option`, `Result` also contains the `unwrap()` method which yields the element
`T` or calls `panic!()`. So far, this should seem similar to `Option`: -->
`Option`と同様、`Result`もまた`unwrap()`メソッドを持っており、これは要素`T`を産出するか`panic!()`します。今のところは、これは`Option`とほぼ同じに見えるでしょう。

{result.play}

<!-- Clearly, panicking on an `Err` leaves an unhelpful error message. Do we even know anything
about libcore that the error is telling us all about? There must be a better way. -->
明らかに、`Err`でパニックした結果、役に立たないエラーメッセージをあとに残すのみとなってしまっています。このエラーの内容から、libcoreで何が起こっているかを推測することができるでしょうか？もっといい方法があるはずです。


### See also:

[`Result`][result]

[result]: http://doc.rust-lang.org/std/result/enum.Result.html
