<!--
# Documentation testing
-->
# ドキュメンテーションテスト

<!--
The primary way of documenting a Rust project is through annotating the source
code. Documentation comments are written in 
[CommonMark Markdown specification][commonmark] and support code blocks in them.
Rust takes care about correctness, so these code blocks are compiled and used 
as documentation tests.
-->
Rustのプロジェクトでは、ソースコードに注釈する形でドキュメントを書くのが主流です。ドキュメンテーションコメントの記述は[CommonMark Markdown specification][commonmark]で行い、コードブロックも使えます。Rustは正確性を重視しているので、コードブロックもコンパイルされ、テストとして使われます。

```rust,ignore
/// First line is a short summary describing function.
/// 最初の行には関数の機能の短い要約を書きます。
///
/// The next lines present detailed documentation. Code blocks start with
/// triple backquotes and have implicit `fn main()` inside
/// and `extern crate <cratename>`. Assume we're testing `doccomments` crate:
/// 以降で詳細なドキュメンテーションを記述します。コードブロックは三重のバッククォートで始まり、
/// 暗黙的に`fn main()`と`extern crate <クレート名>`で囲われます。
/// `doccomments`クレートをテストしたいときには、次のように記述します。
///
/// ```
/// let result = doccomments::add(2, 3);
/// assert_eq!(result, 5);
/// ```
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

/// Usually doc comments may include sections "Examples", "Panics" and "Failures".
/// 一般的に、ドキュメンテーションコメントは「実行例」「パニック」「失敗」という章から成る。
///
/// The next function divides two numbers.
/// 次の関数は除算を実行する。
///
/// # Examples
/// # 実行例
///
/// ```
/// let result = doccomments::div(10, 2);
/// assert_eq!(result, 5);
/// ```
///
/// # Panics
/// # パニック
///
/// The function panics if the second argument is zero.
/// 第2引数がゼロであればパニックする。
///
/// ```rust,should_panic
/// // panics on division by zero
/// // ゼロで除算するとパニックする
/// doccomments::div(10, 0);
/// ```
pub fn div(a: i32, b: i32) -> i32 {
    if b == 0 {
        panic!("Divide-by-zero error");
    }

    a / b
}
```

Code blocks in documentation are automatically tested
when running the regular `cargo test` command:

```shell
$ cargo test
running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

   Doc-tests doccomments

running 3 tests
test src/lib.rs - add (line 7) ... ok
test src/lib.rs - div (line 21) ... ok
test src/lib.rs - div (line 31) ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

<!--
## Motivation behind documentation tests
-->
## ドキュメンテーションテストの目的

<!--
The main purpose of documentation tests is to serve as examples that exercise
the functionality, which is one of the most important
[guidelines][question-instead-of-unwrap]. It allows using examples from docs as
complete code snippets. But using `?` makes compilation fail since `main`
returns `unit`. The ability to hide some source lines from documentation comes
to the rescue: one may write `fn try_main() -> Result<(), ErrorType>`, hide it
and `unwrap` it in hidden `main`. Sounds complicated? Here's an example:
-->
ドキュメンテーションテストの主な目的は、実行例を示すことであり、これは最も大切な[ガイドライン][question-instead-of-unwrap]の一つにもなっています。これにより、ドキュメントの例を実際に動くコードとして使うことができます。しかしながら、`main`が`()`を返すために、`?`を使うとコンパイルに失敗してしまいます。ドキュメンテーションでコードブロックの一部を隠す機能で、この問題に対処できます。つまり、`fn try_main() -> Result<(), ErrorType>`を定義しておきながらそれを隠し、暗黙の`main`の内部で`unwrap`するのです。複雑なので、例を見てみましょう。

```rust,ignore
/// Using hidden `try_main` in doc tests.
/// ドキュメンテーションテストで、`try_main`を隠して使う。
///
/// ```
/// # // hidden lines start with `#` symbol, but they're still compilable!
/// # // 行頭に `#` を置くと行が隠されるが、コンパイルには成功する。
/// # fn try_main() -> Result<(), String> { // line that wraps the body shown in doc
/// #                                       // ドキュメントの本体を囲う行
/// let res = doccomments::try_div(10, 2)?;
/// # Ok(()) // returning from try_main
/// #        // try_mainから値を返す
/// # }
/// # fn main() { // starting main that'll unwrap()
/// #             // unwrap()を実行するmain
/// #    try_main().unwrap(); // calling try_main and unwrapping
/// #                         // so that test will panic in case of error
/// #                         // try_mainを呼びunwrapすると、エラーの場合にパニックする
/// # }
/// ```
pub fn try_div(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err(String::from("Divide-by-zero"))
    } else {
        Ok(a / b)
    }
}
```

<!--
## See Also
-->
## 参照

<!--
* [RFC505][RFC505] on documentation style
* [API Guidelines][doc-nursery] on documentation guidelines
-->
* [RFC505][RFC505] ドキュメンテーションのスタイルについて
* [API Guidelines][doc-nursery] ドキュメンテーションのガイドラインについて

[doc-nursery]: https://rust-lang-nursery.github.io/api-guidelines/documentation.html
[commonmark]: https://commonmark.org/
[markdown]: https://daringfireball.net/projects/markdown/
[RFC505]: https://github.com/rust-lang/rfcs/blob/master/text/0505-api-comment-conventions.md
[question-instead-of-unwrap]: https://rust-lang-nursery.github.io/api-guidelines/documentation.html#examples-use--not-try-not-unwrap-c-question-mark
