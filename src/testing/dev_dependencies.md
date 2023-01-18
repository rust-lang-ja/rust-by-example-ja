<!--
# Development dependencies
-->
# 開発中の依存関係

<!--
Sometimes there is a need to have dependencies for tests (or examples,
or benchmarks) only. Such dependencies are added to `Cargo.toml` in the
`[dev-dependencies]` section. These dependencies are not propagated to other
packages which depend on this package.
-->
テスト（あるいは例やベンチマーク）のためだけに、あるクレートに依存しなければならないことがあります。このような依存関係は、`Cargo.toml`の`[dev-dependencies]`セクションに追加します。このセクションに追加した依存関係は、このパッケージに依存するパッケージには適用されません。

<!--
One such example is [`pretty_assertions`](https://docs.rs/pretty_assertions/1.0.0/pretty_assertions/index.html), which extends standard `assert_eq!` and `assert_ne!` macros, to provide colorful diff.  
File `Cargo.toml`:
-->
そのようなクレートの例として、[`pretty_assertions`](https://docs.rs/pretty_assertions/1.0.0/pretty_assertions/index.html)クレートが挙げられます。これは、標準の`assert_eq!`と`assert_ne!`マクロを拡張して、差分をカラフルに表示するものです。
ファイル`Cargo.toml`:

```toml
# standard crate data is left out
[dev-dependencies]
pretty_assertions = "1"
```

<!--
File `src/lib.rs`:
-->
ファイル`src/lib.rs`:

```rust,ignore
// externing crate for test-only use
// テストにのみ使うクレートをexternで宣言する
#[cfg(test)]
#[macro_use]
extern crate pretty_assertions;

pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq; // crate for test-only use. Cannot be used in non-test code.
                                      // テストのためのクレートであり、テスト以外のコードには使えない。

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
    }
}
```

<!--
## See Also
-->
## 参照
<!--
[Cargo][cargo] docs on specifying dependencies.
-->
依存関係の記述については、[Cargo][cargo]のドキュメントを参照してください。

[cargo]: http://doc.crates.io/specifying-dependencies.html
