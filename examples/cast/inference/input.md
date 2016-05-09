<!-- The type inference engine is pretty smart. It does more than looking at the
type of the
[r-value][rvalue]
during an initialization. It also looks at how the variable is used afterwards
to infer its type. Here's an advanced example of type inference: -->
Rustの型推論エンジンはなかなか賢くできています。初期化の際に右辺値([`r-value`][rvalue])の型をチェックするだけでなく、その後にどのような使われ方をしているかを見て推論します。以下がその例です。

{inference.play}

<!-- No type annotation of variables was needed, the compiler is happy and so is the
programmer! -->
このように、変数の型アノテーションは必要ありません。これでコンパイラもプログラマもハッピーですね！

[rvalue]: https://en.wikipedia.org/wiki/Value_%28computer_science%29#lrvalue
