<!--
# Inference
-->
# 型推論

<!--
The type inference engine is pretty smart. It does more than looking at the
type of the value expression
during an initialization. It also looks at how the variable is used afterwards 
to infer its type. Here's an advanced example of type inference:
-->
FIXME_EN: The type inference engine is pretty smart. It does more than looking at the
FIXME_EN: type of the
FIXME_EN: [r-value][rvalue]
FIXME_EN: during an initialization. It also looks at how the variable is used afterwards
FIXME_EN: to infer its type. Here's an advanced example of type inference:
FIXME_JA: Rustの型推論エンジンはなかなか賢くできています。初期化の際に右辺値([`r-value`][rvalue])の型をチェックするだけでなく、その後にどのような使われ方をしているかを見て推論します。以下がその例です。

```rust,editable
fn main() {
    // Because of the annotation, the compiler knows that `elem` has type u8.
    // アノテーションのおかげで、コンパイラは`elem`がu8型であることがわかる。
    let elem = 5u8;

    // Create an empty vector (a growable array).
    // 空のベクトル(可変長の配列)を生成
    let mut vec = Vec::new();
    // At this point the compiler doesn't know the exact type of `vec`, it
    // just knows that it's a vector of something (`Vec<_>`).
    // この時点でコンパイラは`vec`の型を知らず、
    // 単に何らかの値のベクトル(`Vec<_>`)であるということだけを把握している。

    // Insert `elem` in the vector.
    // `elem`をベクトルに挿入
    vec.push(elem);
    // Aha! Now the compiler knows that `vec` is a vector of `u8`s (`Vec<u8>`)
    // TODO ^ Try commenting out the `vec.push(elem)` line
    // よし！ これでコンパイラは`vec`が`u8`のベクトル(`Vec<u8>`)であることを把握する。
    // TODO ^ 上の `vec.push(elem)` をコメントアウトしてみましょう。

    println!("{:?}", vec);
}
```

<!--
No type annotation of variables was needed, the compiler is happy and so is the
programmer!
-->
このように、変数の型アノテーションは必要ありません。これでコンパイラもプログラマもハッピーですね！
