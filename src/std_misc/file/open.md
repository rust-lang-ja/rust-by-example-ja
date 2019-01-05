<!-- The `open` static method can be used to open a file in read-only mode. -->
スタティックメソッドの`open`を用いることで読み込み専用モードでファイルを開くことが可能です。

<!-- A `File` owns a resource, the file descriptor and takes care of closing the
file when it is `drop`ed. -->
`File`はファイルディスクリプタという資源を保持しており、`drop`時にはファイルを閉じるところまで面倒を見てくれます。

``` rust,ignore
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    // 目的ファイルに対する`Path`を作成
    let path = Path::new("hello.txt");
    let display = path.display();

    // pathを読み込み専用モードで開く。これは`io::Result<File>`を返す。
    let mut file = match File::open(&path) {
        // `io::Error`の`description`メソッドはエラーを説明する文字列を返す。
        Err(why) => panic!("couldn't open {}: {}", display,
                                                   Error::description(&why)),
        Ok(file) => file,
    };

    // ファイルの中身を文字列に読み込む。`io::Result<useize>`を返す。
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display,
                                                   Error::description(&why)),
        Ok(_) => print!("{} contains:\n{}", display, s),
    }

    // `file`がスコープから抜け、"hello.txt"が閉じられる。
}

```

<!-- Here's the expected successful output: -->
以下が成功時に期待されるアウトプットです。

``` bash
$ echo "Hello World!" > hello.txt
$ rustc open.rs && ./open
hello.txt contains:
Hello World!
```

<!-- (You are encouraged to test the previous example under different failure
conditions: `hello.txt` doesn't exist, or `hello.txt` is not readable,
etc.) -->
(気が向いたなら、上記の例を様々な形で失敗させてみましょう。例えば`hello.txt`が存在しないとか、読み込み権限がないとか、そういった状況で実行してみてください。)
