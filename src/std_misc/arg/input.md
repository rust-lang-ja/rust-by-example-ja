<!-- The command line arguments can be accessed using `std::env::args`, which
returns an iterator that yields a String for each argument: -->
コマンドライン引数は`std::env::args`を介して取得できます。これはそれぞれの引数を文字列としてyieldするイテレータを返します。

{args.play}

```
$ ./args 1 2 3
My path is ./args.
I got 3 arguments: ["1", "2", "3"].
```
