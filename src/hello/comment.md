<!--- Any program requires comments and indeed Rust supports --->
<!--- a few different varieties: --->
あらゆるプログラムにはコメントが必要です。ラストには何種類かのコメントがあります

<!--- * *Regular comments* which are ignored by the compiler: --->
<!---  - `// Line comments which go to the end of the line.` --->
<!---  - `/* Block comments which go to the closing delimiter. */` --->
<!--- * *Doc comments* which are parsed into HTML library --->
<!--- [documentation][docs]: --->
<!---  - `/// Generate library docs for the following item.` --->
<!---  - `//! Generate library docs for the enclosing item.` --->

* *通常のコメント* これはコンパイラによって完全に無視されます。
 - `// 行末までコメントアウト`
 - `/* ブロックによって囲まれた部分をコメントアウト */`
* *ドキュメンテーションコメント* ライブラリのドキュメンテーションとしてhtmlにパースされます。
 - `/// このコメントの下の内容に関するドキュメントとなります`
 - `//! このコメントを含むソースのドキュメントになります`

``` rust,editable
fn main() {
    // こちらはラインコメントです
    // 一番左にスラッシュが2つある行と、何も書かれていない行は
    // どちらもコンパイラによって無視されます。試しに実行してみてください

    // println!("Hello, world!");

    // でしょ？では次に、左のスラッシュを消去してから実行してください

    /*
     * こちらはもう一つのタイプのコメントでブロックコメントと呼ばれます。
     * 普通はラインコメントの方が優れているのですが、こちらはデバッグ時に
     * 役立つ場合があります。
     */

     /*
     このように、`*`は、実際にはコメントの前後に１つずつあれば十分です。
     */

     // ではブロックコメントがどのようにデバッグに役立つか見てみましょう。
     // 例えば下の例の場合、ブロックコメントがなくなれば結果が変わります。
     let x = 5 + /* 90 + */ 5;
     println!("Is `x` 10 or 100? x = {}", x);
}

```

### See also:

[ライブラリドキュメンテーション][docs]

[docs]: ../meta/doc.html
