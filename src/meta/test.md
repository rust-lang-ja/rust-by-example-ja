<!-- Functions can be tested by using these [attributes][attributes]: -->
関数のテストは以下の[アトリビュート][attributes]を用いて行うことができます。

<!-- * `#[test]` marks a function as a unit test. The function must take zero
parameters and return nothing.
* `#[should_panic]` marks a function as a panicking test. -->
* `#[test]`は関数をユニットテスト化します。その場合その関数は引数と返り値が空でなくてはなりません。
* `#[should_panic]`はパニックを引き起こすようなテストに用います。

``` rust
// テストを実行しない時に限り、`main`をコンパイルする。
#[cfg(not(test))]
fn main() {
    println!("If you see this, the tests were not compiled nor ran!");
}

// テストを実行するときのみ`test`モジュールをコンパイルする。
#[cfg(test)]
mod test {
    // `distance_test`が必要とするヘルパーメソッド
    fn distance(a: (f32, f32), b: (f32, f32)) -> f32 {
        (
            (b.0 - a.0).powi(2) +
            (b.1 - a.1).powi(2)
        ).sqrt()
    }

    #[test]
    fn distance_test() {
        assert!(distance((0f32, 0f32), (1f32, 1f32)) == (2f32).sqrt());
    }

    #[test]
    #[should_panic]
    fn failing_test() {
        assert!(1i32 == 2i32);
    }
}

```

<!-- Tests can be run with `cargo test` or `rustc --test`. -->
テストは`cargo test`あるいは`rustc --test`で実行します。

``` bash
$ rustc --test unit_test.rs
$ ./unit_test

running 2 tests
test test::distance_test ... ok
test test::failing_test ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured
```

<!-- If `--test` were not included, then this would happen -->
もし`--test`フラグがない場合、出力は以下のようになります。

``` bash
$ rustc unit_test.rs
$ ./unit_test
If you see this, the tests were not compiled nor ran!
```

### See also:

[アトリビュート][attributes], [環境に応じたコンパイル][cfg], [`mod`][mod].

[attributes]: ../attribute.html
[cfg]: ../attribute/cfg.html
[mod]: ../mod.html
