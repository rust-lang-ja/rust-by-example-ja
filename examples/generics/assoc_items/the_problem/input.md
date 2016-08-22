<!-- A `trait` that is generic over its container type has type specification
requirements - users of the `trait` *must* specify all of its generic types. -->
コンテナ型に、その要素に対してジェネリックなトレイトを実装した場合、そのトレイトを使用する者は全てのジェネリック型を明記*しなくてはなりません*。

<!-- In the example below, the `Contains` `trait` allows the use of the generic
types `A` and `B`. The trait is then implemented for the `Container` type,
specifying `i32` for `A` and `B` so that it can be used with `fn difference()`. -->
以下の例では`Contains`トレイトはジェネリック型`A`と`B`の使用を許しています。その後、`Container`型に対して`Contains`を実装していますが、その際後に`fn difference()`が使用できるように、`A`、`B`はそれぞれ`i32`と明記されています。

<!-- Because `Contains` is generic, we are forced to explicitly state *all* of the
generic types for `fn difference()`. In practice, we want a way to express that
`A` and `B` are determined by the *input* `C`. As you will see in the next
section, associated types provide exactly that capability. -->
`Contains`はジェネリックトレイトなので、`fn difference()`では**全ての**ジェネリック型を宣言しなくてはなりません。実際のところ、`A`と`B`は**引数**である`C`によって決定されていて欲しいにも関わらず、です。これは次のページで紹介する関連型と呼ばれる機能によって可能です。

{problem.play}

### See also:

[構造体][structs], [トレイト][traits]

[structs]: /custom_types/structs.html
[traits]: /trait.html
