<!-- Functions can be tested by using these [attributes][attributes]: -->
関数のテストは以下の[アトリビュート][attributes]を用いて行うことができます。

<!-- * `#[test]` marks a function as a unit test. The function must take zero
parameters and return nothing.
* `#[should_panic]` marks a function as a panicking test. -->
* `#[test]`は関数をユニットテスト化します。その場合その関数は引数と返り値が空でなくてはなりません。
* `#[should_panic]`はパニックを引き起こすようなテストに用います。

{unit_test.rs}

<!-- Tests can be run with `cargo test` or `rustc --test`. -->
テストは`cargo test`あるいは`rustc --test`で実行します。

```
$ rustc --test unit_test.rs
$ ./unit_test

running 2 tests
test test::distance_test ... ok
test test::failing_test ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured
```

<!-- If `--test` were not included, then this would happen -->
もし`--test`フラグがない場合、出力は以下のようになります。

```
$ rustc unit_test.rs
$ ./unit_test
If you see this, the tests were not compiled nor ran!
```

### See also:

[アトリビュート][attributes], [環境に応じたコンパイル][cfg], [`mod`][mod].

[attributes]: ../attribute.html
[cfg]: ../attribute/cfg.html
[mod]: ../mod.html
