<!-- The `std::io::fs` module contains several functions that deal with the
filesystem. -->
`std::io::fs`モジュールはファイルシステムとやり取りするための関数をいくつか持っています。

{fs.rs}

<!-- Here's the expected successful output: -->
以下が成功時に期待されるアウトプットです。

```
$ rustc fs.rs && ./fs
`mkdir a`
`echo hello > a/b.txt`
`mkdir -p a/c/d`
`touch a/c/e.txt`
`ln -s ../b.txt a/c/b.txt`
`cat a/c/b.txt`
> hello
`ls a`
> a/b.txt
> a/c
`walk a`
> a/c
> a/c/b.txt
> a/c/e.txt
> a/c/d
> a/b.txt
`rm a/c/e.txt`
`rmdir a/c/d`
```

<!-- And the final state of the `a` directory is: -->
最終的な`a`ディレクトリの状態は以下です。

```
$ tree a
a
|-- b.txt
`-- c
    `-- b.txt -> ../b.txt

1 directory, 2 files
```

### See also:

[`cfg!`][cfg]

[cfg]: ../attribute/cfg.html
