<!-- Macros allow writing DRY code by factoring out the common parts of functions
and/or test suites. Here is an example that implements and tests the `+=`, `*=`
and `-=` operators on `Vec<T>`: -->
マクロは関数やテストなどにおいて、共通の部分を抽出することでDRYなコードを書くのに役立ちます。ここでは`+=`、`*=`、`-=`、`Vec<T>`を実装、テストするにあたって、マクロがどのように役立つかを見ていきます。

{dry.rs}

```
$ rustc --test dry.rs && ./dry
running 3 tests
test test::mul_assign ... ok
test test::add_assign ... ok
test test::sub_assign ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured
```
