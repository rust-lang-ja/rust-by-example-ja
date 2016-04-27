ここでは伝統的な"Hello World!"プログラムのソースを紹介します

{hello.play}

`println!` は文字列をコンソールにプリントするための [*マクロ*][macros]です。

A binary can be generated using the Rust compiler: `rustc`.

```
$ rustc hello.rs
```

`rustc` will produce a `hello` binary that can be executed.

```
$ ./hello
Hello World!
```

### Activity

Click 'Run' above to see the expected output. Next, add a new
line with a second `println!` macro so that the output
shows:
```
Hello World!
I'm a Rustacean!
```

[macros]: ./macros.html
