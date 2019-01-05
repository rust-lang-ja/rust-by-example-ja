<!-- Chaining results using match can get pretty untidy; luckily, the `try!` macro
can be used to make things pretty again. The `try!` macro expands to a match
expression, where the `Err(err)` branch expands to an early `return Err(err)`,
and the `Ok(ok)` branch expands to an `ok` expression. -->
マッチを使用して結果をチェインするのは中々面倒です。幸いなことに、`try!`マクロを使用すればイケてるコードに戻すことができます。`try!`マクロはマッチ構文に展開され、`Err(err)`を返す分岐は早い段階で`return Err(err)`し、`Ok(ok)`を返す分岐は`ok`に展開されます。

``` rust,editable,ignore,mdbook-runnable
mod checked {
    #[derive(Debug)]
    enum MathError {
        DivisionByZero,
        NegativeLogarithm,
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
        if x < 0.0 {
            Err(MathError::NegativeLogarithm)
        } else {
            Ok(x.ln())
        }
    }

    // `op`と、他の3つの関数との間の関数
    fn op_(x: f64, y: f64) -> MathResult {
        // もし`div`が「失敗」したら、`DivisionByZero`が返される。
        let ratio = try!(div(x, y));

        // もし`ln`が「失敗」したら、`NegativeLogarithm`が返される
        let ln = try!(ln(ratio));

        sqrt(ln)
    }

    pub fn op(x: f64, y: f64) {
        match op_(x, y) {
            Err(why) => panic!(match why {
                MathError::NegativeLogarithm
                    => "logarithm of negative number",
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

<!-- Be sure to check the [documentation][docs],
as there are many methods to map/compose `Result`. -->
[公式ドキュメント][docs]をチェックすることをオススメします。`Result`型を扱う関数や`Result`型のメソッドが多く挙げられています。

[docs]: http://doc.rust-lang.org/std/result/index.html
