<!--- This is the source code of the traditional Hello World program. --->
ここでは伝統的な"Hello World!"プログラムのソースを紹介します。　

{hello.play}


<!--- `println!` is a [*macro*][macros] that prints text to the console --->
`println!` は文字列をコンソールにプリントするための [*マクロ*][macros]です。

<!--- A binary can be generated using the Rust compiler: `rustc`. --->
バイナリファイルは`rustc`と呼ばれるRustのコンパイラを用いて生成することができます。

```
$ rustc hello.rs
```

<!--- `rustc` will produce a `hello` binary that can be executed. --->
すると`hello`という名前の実行可能なバイナリファイルができます。

```
$ ./hello
Hello World!
```

<!--- ### Activity --->
### 演習

<!--- Click 'Run' above to see the expected output. Next, add a new --->
<!--- line with a second `println!` macro so that the output --->
上に書いている'Run'をクリックしてアウトプットを見てみましょう。
次に、`println!`マクロをもう一行追加してアウトプットがどうなるか見てみましょう。

shows:
```
Hello World!
I'm a Rustacean!
```

[macros]: ./macros.html
