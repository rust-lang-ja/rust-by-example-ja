<!-- Error handling is the process of handling the possibility of failure. For
example, failing to read a file and then continuing to use that *bad* input
would clearly be problematic. Error handling allows us to notice and handle
those errors in an explicit fashion, saving the rest of the program from
potential issues. -->
エラーハンドリングとは失敗の起きる可能性を扱うプロセスのことです。例えば、ファイルを読み込むのに失敗した際、その*誤った*インプットを使い続けるのは明らかに問題です。エラーハンドリングによって、そのようなエラーに気づき、よりきれいな方法で扱い、残りのプログラムに問題が波及することを防ぐことができるようになります。

<!-- The simplest error handling mechanism we will see is `panic`. It prints an
error message, starts unwinding the task, and usually exits the program.
Consider the following example: -->
これから見ていく中で、最もシンプルなエラーハンドリングの方法は`panic`です。これはエラーメッセージをプリントし、タスクのアンワインドを開始し、典型的にはプログラムを終了します。例えば以下の例を見てみましょう。

{error.play}

<!-- This shows that we can induce program failure at will, but raises a
question: what happens if the princess is *not* given a gift? We *could*
explicitly test this with a check against the null string (`""`) as we do
with the snake, but this is not reliable. The problem is that programmers do
not habitually test these checks unless required to by the compiler. -->
このように我々はプログラムを意図的に失敗させることができます。しかし次のような疑問が生じます。もしお姫様への贈り物が*なかったら*どうなるのでしょう。ヘビを与えたのと同じように、空の文字列(`""`)を与えることで明示的なテストを行うことが*可能です*が、これは現実的ではありません。コンパイラがチェックしない限り、こういったテストをプログラマは習慣的に書かないためです。

<!-- In order for this to be reliable, we'll want the compiler to point out
cases where there may not be a gift. In this chapter, we will learn to use
`Option` to take care of this condition, as well as various functions to
deal with the results of one or more uses of `Option`. -->
こういった状況に対処するため、贈り物が存在しないかもしれない場合というのをコンパイラに教えてあげたくなりますね。この章では`Option`を用いてこういった状況に対処する方法を学びます。1つ以上の`Option`の結果を扱う関数群も紹介します。

> 訳注: こちらのQiitaの日本語記事も参考になります。[「RustでOption値やResult値を上手に扱う」][qiita]

[qiita]: http://qiita.com/tatsuya6502/items/cd41599291e2e5f38a4a
