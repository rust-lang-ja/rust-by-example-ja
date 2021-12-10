<!--
# Testing
-->
# テスト

<!--
As we know testing is integral to any piece of software! Rust has first-class
support for unit and integration testing ([see this
chapter](https://doc.rust-lang.org/book/ch11-00-testing.html) in
TRPL).
-->
知っての通り、テストはどんなソフトウェアにも不可欠です！Rustはユニットテストと統合テストを第一級にサポートしています（TRPLの[この章を参照してください](https://doc.rust-lang.org/book/ch11-00-testing.html)）。

<!--
From the testing chapters linked above, we see how to write unit tests and
integration tests. Organizationally, we can place unit tests in the modules they
test and integration tests in their own `tests/` directory:
-->
上のリンク先のテストの章では、ユニットテストと統合テストの書き方を紹介しています。ユニットテストはテスト対象のモジュール内に、統合テストは`tests/`ディレクトリ内に置きます。

```txt
foo
├── Cargo.toml
├── src
│   └── main.rs
│   └── lib.rs
└── tests
    ├── my_test.rs
    └── my_other_test.rs
```

<!--
Each file in `tests` is a separate 
[integration test](https://doc.rust-lang.org/book/ch11-03-test-organization.html#integration-tests),
i.e. a test that is meant to test your library as if it were being called from a dependent
crate.
-->
`tests`内の各ファイルは個別の[統合テスト](https://doc.rust-lang.org/book/ch11-03-test-organization.html#integration-tests)です。
これはライブラリを依存クレートから呼ばれたかのようにテストできます。

The [Testing][testing] chapter elaborates on the three different testing styles: 
[Unit][unit_testing], [Doc][doc_testing], and [Integration][integration_testing]. 

<!--
`cargo` naturally provides an easy way to run all of your tests!
-->
`cargo`は、全てのテストを簡単に実行する方法を提供します。

```shell
$ cargo test
```

<!--
You should see output like this:
-->
出力はこのようになります。

```shell
$ cargo test
   Compiling blah v0.1.0 (file:///nobackup/blah)
    Finished dev [unoptimized + debuginfo] target(s) in 0.89 secs
     Running target/debug/deps/blah-d3b32b97275ec472

running 3 tests
test test_bar ... ok
test test_baz ... ok
test test_foo_bar ... ok
test test_foo ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

<!--
You can also run tests whose name matches a pattern:
-->
パターンにマッチする名前のテストを実行することもできます。

```shell
$ cargo test test_foo
```

```shell
$ cargo test test_foo
   Compiling blah v0.1.0 (file:///nobackup/blah)
    Finished dev [unoptimized + debuginfo] target(s) in 0.35 secs
     Running target/debug/deps/blah-d3b32b97275ec472

running 2 tests
test test_foo ... ok
test test_foo_bar ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 2 filtered out
```

<!--
One word of caution: Cargo may run multiple tests concurrently, so make sure
that they don't race with each other. 
-->
注意：Cargoは複数のテストを並列で実行することがありますので、それらが互いに競合しないようにしてください。

One example of this concurrency causing issues is if two tests output to a
file, such as below:

```rust
#[cfg(test)]
mod tests {
    // Import the necessary modules
    use std::fs::OpenOptions;
    use std::io::Write;

    // This test writes to a file
    #[test]
    fn test_file() {
        // Opens the file ferris.txt or creates one if it doesn't exist.
        let mut file = OpenOptions::new()
            .append(true)
            .create(true)
            .open("ferris.txt")
            .expect("Failed to open ferris.txt");

        // Print "Ferris" 5 times.
        for _ in 0..5 {
            file.write_all("Ferris\n".as_bytes())
                .expect("Could not write to ferris.txt");
        }
    }

    // This test tries to write to the same file
    #[test]
    fn test_file_also() {
        // Opens the file ferris.txt or creates one if it doesn't exist.
        let mut file = OpenOptions::new()
            .append(true)
            .create(true)
            .open("ferris.txt")
            .expect("Failed to open ferris.txt");

        // Print "Corro" 5 times.
        for _ in 0..5 {
            file.write_all("Corro\n".as_bytes())
                .expect("Could not write to ferris.txt");
        }
    }
}
```

Although the intent is to get the following:
```shell
$ cat ferris.txt
Ferris
Ferris
Ferris
Ferris
Ferris
Corro
Corro
Corro
Corro
Corro
```
What actually gets put into `ferris.txt` is this:
```shell
$ cargo test test_foo
Corro
Ferris
Corro
Ferris
Corro
Ferris
Corro
Ferris
Corro
Ferris
```

[testing]: ../testing.md
[unit_testing]: ../testing/unit_testing.md
[integration_testing]: ../testing/integration_testing.md
[doc_testing]: ../testing/doc_testing.md
