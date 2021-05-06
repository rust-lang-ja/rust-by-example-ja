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
└── tests
    ├── my_test.rs
    └── my_other_test.rs
```

<!--
Each file in `tests` is a separate integration test.
-->
`tests`内の各ファイルは個別の統合テストです。

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
that they don't race with each other. For example, if they all output to a
file, you should make them write to different files.
-->
注意：Cargoは複数のテストを並列で実行することがありますので、それらが互いに競合しないようにしてください。例えば、それらが全てファイルに出力する場合、それぞれ別のファイルに書き込むようにします。
