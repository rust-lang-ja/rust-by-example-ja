<!--
# Comments
-->
# コメント

<!--
Any program requires comments, and Rust supports
a few different varieties:
-->
あらゆるプログラムにはコメントが必要です。Rustには何種類かのコメントがあります

<!--
* *Regular comments* which are ignored by the compiler:
  * `// Line comments which go to the end of the line.`
  * `/* Block comments which go to the closing delimiter. */`
* *Doc comments* which are parsed into HTML library [documentation][docs]:
  * `/// Generate library docs for the following item.`
  * `//! Generate library docs for the enclosing item.`
-->
* *通常のコメント* これはコンパイラによって完全に無視されます。
  * `// 行末までコメントアウト`
  * `/* ブロックによって囲まれた部分をコメントアウト */`
* *ドキュメンテーションコメント* ライブラリのドキュメンテーションとしてhtmlにパースされます。
  * `/// このコメントの下の内容に関するドキュメントとなります`
  * `//! このコメントを含むソースのドキュメントになります`

```rust,editable
fn main() {
    // This is an example of a line comment.
    // There are two slashes at the beginning of the line.
    // And nothing written inside these will be read by the compiler.
    // こちらはラインコメントです
    // 一番左にスラッシュが2つある行と、何も書かれていない行は
    // どちらもコンパイラによって無視されます。試しに実行してみてください

    // println!("Hello, world!");

    // Run it. See? Now try deleting the two slashes, and run it again.
    // でしょ？では次に、左のスラッシュを消去してから実行してください

    /*
     * This is another type of comment, a block comment. In general,
     * line comments are the recommended comment style. But block comments
     * are extremely useful for temporarily disabling chunks of code.
     * /* Block comments can be /* nested, */ */ so it takes only a few
     * keystrokes to comment out everything in this main() function.
     * /*/*/* Try it yourself! */*/*/
     */
    /*
     * こちらはもう一つのタイプのコメントでブロックコメントと呼ばれます。
     * 普通はラインコメントの方が優れているのですが、こちらはデバッグ時に
     * 役立つ場合があります。
     */

    /*
    Note: The previous column of `*` was entirely for style. There's
    no actual need for it.
    */
    /*
    このように、`*`は、実際にはコメントの前後に１つずつあれば十分です。
    */

    // You can manipulate expressions more easily with block comments
    // than with line comments. Try deleting the comment delimiters
    // to change the result:
    // ではブロックコメントがどのようにデバッグに役立つか見てみましょう。
    // 例えば下の例の場合、ブロックコメントがなくなれば結果が変わります。
    let x = 5 + /* 90 + */ 5;
    println!("Is `x` 10 or 100? x = {}", x);
}

```

<!--
### See also:
-->
### 参照

<!--
[Library documentation][docs]
-->
[ライブラリドキュメンテーション][docs]

[docs]: ../meta/doc.md
