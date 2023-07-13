# `?`

<!--
Chaining results using match can get pretty untidy; luckily, the `?` operator
can be used to make things pretty again. `?` is used at the end of an expression
returning a `Result`, and is equivalent to a match expression, where the 
`Err(err)` branch expands to an early `return Err(From::from(err))`, and the `Ok(ok)`
branch expands to an `ok` expression.
-->
マッチを利用して結果をチェインするのは中々面倒です。
幸いなことに、`?`マクロを使用すればイケてるコードに戻すことができます。
`?`は`Result`を返す式の末尾で使い、マッチ式と等価です。
`Err(err)`の分岐は`return Err(From::from(err))`という早期リターンに展開され、
`Ok(ok)`の分岐は`ok`の式に展開されます。

```rust,editable,ignore,mdbook-runnable
mod checked {
    #[derive(Debug)]
    enum MathError {
        DivisionByZero,
        NonPositiveLogarithm,
        NegativeSquareRoot,
    }

    type MathResult = Result<f64, MathError>;

    fn div(x: f64, y: f64) -> MathResult {
        if y == 0.0 {
            Err(MathError::DivisionByZero)
        } else {
            Ok(x / y)
        }
    }

    fn sqrt(x: f64) -> MathResult {
        if x < 0.0 {
            Err(MathError::NegativeSquareRoot)
        } else {
            Ok(x.sqrt())
        }
    }

    fn ln(x: f64) -> MathResult {
        if x <= 0.0 {
            Err(MathError::NonPositiveLogarithm)
        } else {
            Ok(x.ln())
        }
    }

    // Intermediate function
    // 中間関数
    fn op_(x: f64, y: f64) -> MathResult {
        // if `div` "fails", then `DivisionByZero` will be `return`ed
        // `div`が"失敗"したら、`DivisionByZero`が`return`される。
        let ratio = div(x, y)?;

        // if `ln` "fails", then `NonPositiveLogarithm` will be `return`ed
        // もし`ln`が"失敗"したら、`NonPositiveLogarithm`が`return`される。
        let ln = ln(ratio)?;

        sqrt(ln)
    }

    pub fn op(x: f64, y: f64) {
        match op_(x, y) {
            Err(why) => panic!("{}", match why {
                MathError::NonPositiveLogarithm
                    => "logarithm of non-positive number",
                MathError::DivisionByZero
                    => "division by zero",
                MathError::NegativeSquareRoot
                    => "square root of negative number",
            }),
            Ok(value) => println!("{}", value),
        }
    }
}

fn main() {
    checked::op(1.0, 10.0);
}
```

<!--
Be sure to check the [documentation][docs],
as there are many methods to map/compose `Result`.
-->
[公式ドキュメント][docs]をチェックすることをオススメします。
`Result`型を扱う関数や`Result`型のメソッドが多く挙げられています。

[docs]: https://doc.rust-lang.org/std/result/index.html
