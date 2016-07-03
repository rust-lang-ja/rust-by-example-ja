<!-- The `open` static method can be used to open a file in read-only mode. -->
スタティックメソッドの`open`を用いることで読み込み専用モードでファイルを開くことが可能です。

<!-- A `File` owns a resource, the file descriptor and takes care of closing the
file when it is `drop`ed. -->
`File`はファイルディスクリプタという資源を保持しており、`drop`時にはファイルを閉じるところまで面倒を見てくれます。

{open.rs}

<!-- Here's the expected successful output: -->
以下が成功時に期待されるアウトプットです。

```
$ echo "Hello World!" > hello.txt
$ rustc open.rs && ./open
hello.txt contains:
Hello World!
```

<!-- (You are encouraged to test the previous example under different failure
conditions: `hello.txt` doesn't exist, or `hello.txt` is not readable,
etc.) -->
(気が向いたなら、上記の例を様々な形で失敗させてみましょう。例えば`hello.txt`が存在しないとか、読み込み権限がないとか、そういった状況で実行してみてください。)
