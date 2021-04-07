<!--
# Variadic Interfaces
-->
# 可変個引数によるインターフェース

<!--
A _variadic_ interface takes an arbitrary number of arguments. For example,
`println!` can take an arbitrary number of arguments, as determined by the
format string.
-->
*可変個引数の*インターフェースとは、任意の数の引数を取るものです。
例えば、`println!`は、フォーマット文字列の定義に従い、任意の数の引数を取ることができます。

<!--
We can extend our `calculate!` macro from the previous section to be variadic:
-->
前のセクションの`calculate!`マクロを、可変個引数に拡張することができます：

```rust,editable
macro_rules! calculate {
    // 単一の`eval`のためのパターン
    // The pattern for a single `eval`
    (eval $e:expr) => {{
        {
            let val: usize = $e; // Force types to be integers
            println!("{} = {}", stringify!{$e}, val);
        }
    }};

    // 複数の`eval`を再帰的に分解する
    // Decompose multiple `eval`s recursively
    (eval $e:expr, $(eval $es:expr),+) => {{
        calculate! { eval $e }
        calculate! { $(eval $es),+ }
    }};
}

fn main() {
                 // ほら！可変な`calculate!`だよ！
    calculate! { // Look ma! Variadic `calculate!`!
        eval 1 + 2,
        eval 3 + 4,
        eval (2 * 3) + 1
    }
}
```

<!--
Output:
-->
出力：

```txt
1 + 2 = 3
3 + 4 = 7
(2 * 3) + 1 = 7
```
