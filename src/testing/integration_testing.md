<!--
# Integration testing
-->
# インテグレーションテスト

<!--
[Unit tests][unit] are testing one module in isolation at a time: they're small
and can test private code. Integration tests are external to your crate and use
only its public interface in the same way any other code would. Their purpose is
to test that many parts of your library work correctly together.
-->
[ユニットテスト][unit]は、独立したモジュールを一つずつテストするものであり、テストは小さく、プライベートなコードについてもテストすることができます。インテグレーションテストはクレートの外側にあるもので、他の外部のコードと同様に、パブリックなインタフェースだけを使います。インテグレーションテストの目的は、ライブラリのそれぞれのモジュールが連携して正しく動作するかどうかテストすることです。

<!--
Cargo looks for integration tests in `tests` directory next to `src`.
-->
Cargoは、`src`ディレクトリと並んで配置された`tests`ディレクトリをインテグレーションテストとして扱います。

<!--
File `src/lib.rs`:
-->
ファイル`src/lib.rs`:

```rust,ignore
// Define this in a crate called `adder`.
// Assume that crate is called adder, will have to extern it in integration test.
// `adder`という名前のクレートの内部で、次の関数を定義する。
// インテグレーションテストでadderクレートをexternで宣言する。
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

<!--
File with test: `tests/integration_test.rs`:
-->
テストを含むファイル`tests/integration_test.rs`:

```rust,ignore
// extern crate we're testing, same as any other code would do.
// 他の外部のコードと同様に、テスト対象のクレートをexternで宣言する。
extern crate adder;

#[test]
fn test_add() {
    assert_eq!(adder::add(3, 2), 5);
}
```

<!--
Running tests with `cargo test` command:
-->
`cargo test`コマンドでテストを実行します。

```shell
$ cargo test
running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

     Running target/debug/deps/integration_test-bcd60824f5fbfe19

running 1 test
test test_add ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

   Doc-tests adder

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

<!--
Each Rust source file in the `tests` directory is compiled as a separate crate. In
order to share some code between integration tests we can make a module with public
functions, importing and using it within tests.
-->
`tests`ディレクトリにあるRustのソースファイルは別のクレートとしてコンパイルされます。インテグレーションテストの間でコードを共有するには、パブリックな関数をモジュールに入れて、それぞれのテストでインポートして利用する方法があります。

<!--
File `tests/common/mod.rs`:
-->
ファイル`tests/common.rs`:

```rust,ignore
pub fn setup() {
    // some setup code, like creating required files/directories, starting
    // servers, etc.
    // 必要なファイル・ディレクトリの作成やサーバの起動といった準備を行うコードを記述する。
}
```

<!--
File with test: `tests/integration_test.rs`
-->
テストを含むファイル`tests/integration_test.rs`:

```rust,ignore
// extern crate we're testing, same as any other code will do.
// 他の外部のコードと同様に、テスト対象のクレートをexternで宣言する。
extern crate adder;

// importing common module.
// 共通のモジュールをインポートする。
mod common;

#[test]
fn test_add() {
    // using common code.
    // 共通のコードを利用する。
    common::setup();
    assert_eq!(adder::add(3, 2), 5);
}
```

<!-- 
Creating the module as `tests/common.rs` also works, but is not recommended
because the test runner will treat the file as a test crate and try to run tests
inside it.
-->
モジュールを`tests/common.rs`に記述することも可能ですが、`tests/common.rs`中のテストも自動的に実行されてしまうため非推奨です。

[unit]: unit_testing.md
[mod]: ../mod.md
