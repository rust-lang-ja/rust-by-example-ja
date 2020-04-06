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

{comment.play}

### See also:

[ライブラリドキュメンテーション][docs]

[docs]: ../meta/doc.html
