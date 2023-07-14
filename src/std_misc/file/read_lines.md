# `read_lines`

<!--
## A naive approach
-->
## 単純なやり方

<!--
This might be a reasonable first attempt for a beginner's first
implementation for reading lines from a file.
-->
テキストファイルの行を読み込むのを、初心者が初めて実装した場合、
以下のようになるでしょう。

```rust,norun
use std::fs::read_to_string;

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
}
```

<!--
Since the method `lines()` returns an iterator over the lines in the file,
we can also perform a map inline and collect the results, yielding a more
concise and fluent expression.
-->
`lines()`メソッドはファイルの各行のイテレータを返すので、
インラインでマップを実行し結果を収集することもできます。
そうすると、より簡潔で読みやすい表現となります。

```rust,norun
use std::fs::read_to_string;

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename) 
        .unwrap()  // panic on possible file-reading errors
                   // ファイル読み込みエラーの場合はパニックする。
        .lines()  // split the string into an iterator of string slices
                  // 文字列のスライスのイテレータに分割する。
        .map(String::from)  // make each slice into a string
                            // スライスを文字列に変換する。
        .collect()  // gather them together into a vector
                    // ベクタにまとめる。
}
```

<!--
Note that in both examples above, we must convert the `&str` reference
returned from `lines()` to the owned type `String`, using `.to_string()`
and `String::from` respectively.
-->
上の例では、`lines()`から返された`&str`を
それぞれ`to_string()`と`String::from`を使って、
所有権のある`String`型に変換しなければならない点に注意してください。

<!--
## A more efficient approach
-->
## より効率的なやり方

<!--
Here we pass ownership of the open `File` to a `BufReader` struct. `BufReader` uses an internal
buffer to reduce intermediate allocations.
-->
ここでは、開いた`File`の所有権を`BufReader`構造体に渡します。
`BufReader`は内部的なバッファを使い、中間のメモリ割り当てを削減します。

<!--
We also update `read_lines` to return an iterator instead of allocating new
`String` objects in memory for each line.
-->
`read_lines`を更新して、それぞれの行に対してメモリ上に新しい`String`オブジェクトを割り当てるのではなく、イテレータを返すようにします。

```rust,no_run
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    // File hosts.txt must exist in the current path
    // hosts.txtファイルは現在のパスに存在しなければならない。
    if let Ok(lines) = read_lines("./hosts.txt") {
        // Consumes the iterator, returns an (Optional) String
        // イテレータを消費し、Option型のStringを返す。
        for line in lines {
            if let Ok(ip) = line {
                println!("{}", ip);
            }
        }
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
// 出力はResult型にラップされ、エラーをマッチできるようになる。
// ファイルの各行のReaderへのイテレータを返す。
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
```

<!--
Running this program simply prints the lines individually.
-->
このプログラムを実行すると、単に各行をプリントします。

```shell
$ echo -e "127.0.0.1\n192.168.0.1\n" > hosts.txt
$ rustc read_lines.rs && ./read_lines
127.0.0.1
192.168.0.1
```

<!--
(Note that since `File::open` expects a generic `AsRef<Path>` as argument, we define our
generic `read_lines()` method with the same generic constraint, using the `where` keyword.)
-->
`File::open`はジェネリックな`AsRef<Path>`を引数にとるので、
ジェネリックな`read_lines`メソッドも、`where`キーワードを使って、同じジェネリックな制約を持たせています。

<!--
This process is more efficient than creating a `String` in memory with all of the file's
contents. This can especially cause performance issues when working with larger files.
-->
この処理は、ファイルの中身全てをメモリ上の`String`にするよりも効率的です。
メモリ上に`String`を作ると、より大きなファイルを取り扱う際に、パフォーマンスの問題につながります。
