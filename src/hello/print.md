<!--
# Formatted print
-->
# フォーマットしてプリント

<!--
Printing is handled by a series of [`macros`][macros] defined in [`std::fmt`][fmt]
some of which include:
-->
プリント関係の機能は[`std::fmt`][fmt]で定義される幾つかの[マクロ][macros]によって扱われます。このマクロには以下が含まれます。

<!--
* `format!`: write formatted text to [`String`][string]
* `print!`: same as `format!` but the text is printed to the console (io::stdout).
* `println!`: same as `print!` but a newline is appended.
* `eprint!`: same as `format!` but the text is printed to the standard error (io::stderr).
* `eprintln!`: same as `eprint!`but a newline is appended.
-->
* `format!`: フォーマットされたテキストを文字列(String)型に書き込みます。
* `print!`: `format!` と同様ですが、コンソール (io::stdout) にそのテキストを出力します。
* `println!`: `print!`: と同じですが改行が付け加えられます。
* `eprint!`: `format!` と同様ですが、標準エラー出力 (io::stderr) にそのテキストを出力します。
* `eprintln!`: `eprint!`と同じですが改行が付け加えられます。

<!--
All parse text in the same fashion. As a plus, Rust checks formatting
correctness at compile time.
-->
すべて同じやり方でテキストをパースし、正しくフォーマットできるかコンパイル時にチェックします。

```rust,editable,ignore,mdbook-runnable
fn main() {
    // In general, the `{}` will be automatically replaced with any
    // arguments. These will be stringified.
    // 一般的に `{} `はどんな引数であろうと自動的に置き換えられます。
    // 例えば以下は文字列に変換されます
    println!("{} days", 31);

    // Without a suffix, 31 becomes an i32. You can change what type 31 is
    // by providing a suffix. The number 31i64 for example has the type i64.
    // サフィックスで型を指定しなければ31はi32として扱われます。
    // サフィックスの指定により、31の型を自由に変換することができます。

    // There are various optional patterns this works with. Positional
    // arguments can be used.
    // 引数の位置から埋め込まれる場所を指定することができます。
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    // As can named arguments.
    // 名前での指定も可能です。
    println!("{subject} {verb} {object}",
             object="the lazy dog",
             subject="the quick brown fox",
             verb="jumps over");

    // Special formatting can be specified after a `:`.
    // `:` のあとにフォーマット型を指定することによる特殊なフォーマットも可能です.
    println!("{} of {:b} people know binary, the other half doesn't", 1, 2);

    // You can right-align text with a specified width. This will output
    // "     1". 5 white spaces and a "1".
    // 指定した幅の中に、右寄せで文字列を挿入することができます。
    // 以下の例では"     1". というように、５つの半角空白のあとに"1"が入ります.
    println!("{number:>width$}", number=1, width=6);

    // You can pad numbers with extra zeroes. This will output "000001".
    // 空白の代わりに0を使うこともできます. このアウトプットは "000001" になります.
    println!("{number:0>width$}", number=1, width=6);

    // Rust even checks to make sure the correct number of arguments are
    // used.
    // 引数の数が正しいかのチェックも行ってくれます。
    println!("My name is {0}, {1} {0}", "Bond");
    // FIXME ^ Add the missing argument: "James"

    // Create a structure named `Structure` which contains an `i32`.
    // `i32`保持する `Structure` という名の構造体を定義します.
    #[allow(dead_code)]
    struct Structure(i32);

    // However, custom types such as this structure require more complicated
    // handling. This will not work.
    // このようにカスタム型を用いる場合、少々扱いが複雑になります。
    // 以下は動作しません。
    println!("This struct `{}` won't print...", Structure(3));
    // FIXME ^ Comment out this line.
}
```

<!--
[`std::fmt`][fmt] contains many [`traits`][traits] which govern the display
of text. The base form of two important ones are listed below:
-->
[`std::fmt`][fmt]はいくつもの[トレイト][traits]を持ち、それによってどのようにディスプレイに表示されるかが決まります。
特に大事な形式は以下の２つです。

<!--
* `fmt::Debug`: Uses the `{:?}` marker. Format text for debugging purposes.
* `fmt::Display`: Uses the `{}` marker. Format text in a more elegant, user
friendly fashion.
-->
* `fmt::Debug`: は、`{:?}`というマーカーを使用し、デバッギング目的に使われます。
* `fmt::Display`: は `{}`というマーカーを使用し、より美しく、ユーザフレンドリーに表示します。

<!--
Here, we used `fmt::Display` because the std library provides implementations
for these types. To print text for custom types, more steps are required.
-->
この例で用いられている型は、標準ライブラリに含まれているため、ここでは`fmt::Display`を使用しています。カスタム型をテキストとして表示する場合は、さらに手順が必要です。

<!--
Implementing the `fmt::Display` trait automatically implements the
[`ToString`] trait which allows us to [convert] the type to [`String`][string].
-->
`fmt::Display`トレイトを実装すると、自動的に[`ToString`]トレイトが実装されます。これにより[`String`][string]型への型変換ができるようになります。

<!--
### Activities
-->
### 演習

<!--
 * Fix the two issues in the above code (see FIXME) so that it runs without
   error.
 * Add a `println!` macro that prints: `Pi is roughly 3.142` by controlling
   the number of decimal places shown. For the purposes of this exercise,
   use `let pi = 3.141592` as an estimate for pi. (Hint: you may need to
   check the [`std::fmt`][fmt] documentation for setting the number of
   decimals to display)
-->
 * 上の例を実行した際に生じるエラーを修復しましょう。
 * `println!`マクロを追加し、表示される小数部の桁数を調整して`Pi is roughly 3.142`という文字列を出力しましょう。
   ただし、円周率の値は`let pi = 3.141592`を使ってください。（ヒント: 小数部の桁数を調整する方法については、[`std::fmt`][fmt]をチェックする必要があるかもしれません。）

<!--
### See also:
-->
### 参照

<!--
[`std::fmt`][fmt], [`macros`][macros], [`struct`][structs],
and [`traits`][traits]
-->
[`std::fmt`][fmt], [マクロ][macros], [構造体][structs],
[トレイト][traits]

[fmt]: https://doc.rust-lang.org/std/fmt/
[macros]: ../macros.md
[string]: ../std/str.md
[structs]: ../custom_types/structs.md
[traits]: https://doc.rust-lang.org/std/fmt/#formatting-traits
[`ToString`]: https://doc.rust-lang.org/std/string/trait.ToString.html
[convert]: ../conversion/string.md
