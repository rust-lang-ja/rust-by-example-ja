# Hello World

<!--
This is the source code of the traditional Hello World program.
-->
ここでは伝統的な"Hello World!"プログラムのソースを紹介します。　

```rust,editable
// This is a comment, and is ignored by the compiler
// You can test this code by clicking the "Run" button over there ->
// or if you prefer to use your keyboard, you can use the "Ctrl + Enter" shortcut
// これはコメントです。コンパイラによって無視されます。
// 右にある「Run」ボタンからこのコードをテストできます。
// キーボードを使いたければ「Ctrl + Enter」もOKです。

// This code is editable, feel free to hack it!
// You can always return to the original code by clicking the "Reset" button ->
// このコードは編集可能です。ぜひハックしてみましょう！
// 「Reset」ボタンでいつでも元のコードに戻すことができます ->

// This is the main function
// main関数です
fn main() {
    // Statements here are executed when the compiled binary is called
    // コンパイルされたバイナリが実行されるとこの関数が呼び出されます

    // Print text to the console
    // コンソールに文字列を出力する
    println!("Hello World!");
}
```

<!--
`println!` is a [*macro*][macros] that prints text to the
console.
-->
`println!`は文字列をコンソールにプリントするための[ *マクロ* ][macros]です。

<!--
A binary can be generated using the Rust compiler: `rustc`.
-->
バイナリファイルは`rustc`と呼ばれるRustのコンパイラを用いて生成することができます。

```bash
$ rustc hello.rs
```

<!--
`rustc` will produce a `hello` binary that can be executed.
-->
すると`hello`という名前の実行可能なバイナリファイルができます。

```bash
$ ./hello
Hello World!
```

<!--
### Activity
-->
### 演習

<!--
Click 'Run' above to see the expected output. Next, add a new
line with a second `println!` macro so that the output
shows:
-->
上に書いている'Run'をクリックしてアウトプットを見てみましょう。
次に、`println!`マクロをもう一行追加してアウトプットがどうなるか見てみましょう。

```text
Hello World!
I'm a Rustacean!
```

[macros]: macros.md
