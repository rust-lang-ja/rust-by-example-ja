<!-- We have been using `Strings` as errors for a while. In fact, this is somewhat limiting as
an error type. Below are the criteria for a good error type. `String` nicely fulfills the first
two but not the second two: -->
これまで、エラーの値として`String`をしばらく使用してきました。実のところ、これはエラーとして用いるには限界があります。以下に良いエラー型であるための条件を挙げます。`String`は1つめと2つめの条件はいい感じに満たしてくれるのですが後の2つは満たしません。

<!-- * Represents different errors with the same type
* Presents nice error messages to the user
* Is easily type comparable. Consider comparing these two types:
    - `Err("Please use a vector with at least one element".to_owned())`
    - `Err(EmptyVec)`
* Can hold information about the error. Compare:
    - `Err("+ cannot be used here".to_owned())`
    - `Err(BadChar(c, position))` -->
* 異なるエラーを一つの型で表現できる。
* ユーザーに綺麗なエラーメッセージを見せてくれる。
* 容易に型比較ができる。例えば以下の二つの型を比較することを考えてみましょう。
    - `Err("少なくとも一つの要素を持つベクタを使用してください".to_owned())`
    - `Err(EmptyVec)`
* エラーの内容に関する情報を持つことができる。例えば以下の二つを比べてください。
    - `Err("+ はここでは使用できません".to_owned())`
    - `Err(BadChar(c, position))`

<!-- This makes `String` errors both difficult to react to and verbose to create. In fact, a nice
looking error message has nothing to do with how the type is structured. It is simply a
consequence of `Display` being implemented for the type. It should not be necessary to
pollute logic heavy code with `String` formatting simply for nice error messages. -->
このことから、`String`をエラーとして用いると、エラーの対処・明示的な作成という点が困難になります。実際のところ、たとえエラーメッセージの見た目がいい感じであったとしても、それはそのエラーの型の構造とは何の関係もありません。単に、その型に対する`Display`の実装のみに依存するからです。単純にいい感じのエラーメッセージを表示したいという目的のために`String`を操作するコードを書くと、どうしてもコードベースが複雑になって汚染されてしまいますので、その必要はありません。

{rethink.play}

### See also:

[`Result`][result]、[`io::Result`][io_result]

[result]: http://doc.rust-lang.org/std/result/enum.Result.html
[io_result]: http://doc.rust-lang.org/std/io/type.Result.html
[inplace]: /error/option_with_result/result_string_errors.html
