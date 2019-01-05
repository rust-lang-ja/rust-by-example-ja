<!-- The command line arguments can be accessed using `std::env::args`, which
returns an iterator that yields a String for each argument: -->
コマンドライン引数は`std::env::args`を介して取得できます。これはそれぞれの引数を文字列としてyieldするイテレータを返します。

``` rust,editable
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    // ひとつ目の引数はプログラムを呼び出す際に使用したパス
    println!("My path is {}.", args[0]);

    // 残りはプログラムに渡されたコマンドラインパラメータ。
    // プログラムはこんなふうに呼び出す。
    //   $ ./args arg1 arg2
    println!("I got {:?} arguments: {:?}.", args.len() - 1, &args[1..]);
}

```

``` bash
$ ./args 1 2 3
My path is ./args.
I got 3 arguments: ["1", "2", "3"].
```
