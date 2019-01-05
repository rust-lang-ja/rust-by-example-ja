<!-- The `create` static method opens a file in write-only mode. If the file
already existed, the old content is destroyed. Otherwise, a new file is
created. -->
スタティックメソッドの`create`はファイルを書き込み専用モードで開きます。すでにファイルが存在している場合、破棄して新しい物を作成します。

``` rust,ignore
static LOREM_IPSUM: &'static str =
"Lorem ipsum dolor sit amet, consectetur adipisicing elit, sed do eiusmod
tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam,
quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo
consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse
cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non
proident, sunt in culpa qui officia deserunt mollit anim id est laborum.
";

use std::error::Error;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;

fn main() {
    let path = Path::new("out/lorem_ipsum.txt");
    let display = path.display();

    // ファイルを書き込み専用モードで開く。返り値は`io::Result<File>`
    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}",
                           display,
                           Error::description(&why)),
        Ok(file) => file,
    };

    // `LOREM_IPSUM`の文字列を`file`に書き込む。返り値は`io::Result<()>`
    match file.write_all(LOREM_IPSUM.as_bytes()) {
        Err(why) => {
            panic!("couldn't write to {}: {}", display,
                                               Error::description(&why))
        },
        Ok(_) => println!("successfully wrote to {}", display),
    }
}

```

<!-- Here's the expected successful output: -->
以下は成功時に期待されるアウトプットです。

``` bash
$ mkdir out
$ rustc create.rs && ./create
successfully wrote to out/lorem_ipsum.txt
$ cat out/lorem_ipsum.txt
Lorem ipsum dolor sit amet, consectetur adipisicing elit, sed do eiusmod
tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam,
quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo
consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse
cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non
proident, sunt in culpa qui officia deserunt mollit anim id est laborum.
```

<!-- (As in the previous example, you are encouraged to test this example under
failure conditions.) -->
<!-- 前項の例と同じように、様々な失敗パターンをためしてみることをオススメします。 -->

<!-- There is also a more generic `open_mode` method that can open files in other
modes like: read+write, append, etc. -->
よりジェネリックな`open_mode`メソッドというものもあります。これはファイルをより様々なモード(例: 読み込み+書き込み、アペンドなど。)で開くことができます。
