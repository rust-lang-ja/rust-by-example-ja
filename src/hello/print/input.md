<!--- Printing is handled by a series of [`macros`][macros] defined in [`std::fmt`][fmt] --->
<!--- some of which include: --->
プリント関係の機能は[`std::fmt`][fmt]で定義される幾つかの[マクロ][macros]によって扱われます。このマクロには以下が含まれます。

<!--- * `format!`: write formatted text to [`String`][string] --->
* `format!`: フォーマットされたテキストを文字列(String)型に書き込みます。
<!--- * `print!`: same as `format!` but the text is printed to the console. --->
* `print!`: `format!` と同様ですが、コンソールにそのテキストを出力します。
<!--- * `println!`: same as `print!` but a newline is appended. --->
* `println!`: `print!`: と同じですが改行が付け加えられます。

<!--- All parse text in the same fashion. A plus is that the formatting correctness will --->
<!--- be checked at compile time. --->
すべて同じやり方でテキストをパースし、正しくフォーマットできるかコンパイル時にチェックします。

{print.play}

<!--- [`std::fmt`][fmt] contains many [`traits`][traits] which govern the display --->
<!--- of text. The base form of two important ones are listed below: --->
[`std::fmt`][fmt]はいくつもの[トレイト][traits]を持ち、それによってどのようにディスプレイに表示されるかが決まります。
特に大事な形式は以下の２つです。

<!--- * `fmt::Debug`: Uses the `{:?}` marker. Format text for debugging purposes. --->
<!--- * `fmt::Display`: Uses the `{}` marker. Format text in a more elegant, user --->
<!--- friendly fashion. --->

* `fmt::Debug`: は、`{:?}`というマーカーを使用し、デバッギング目的に使われます。
* `fmt::Display`: は `{}`というマーカーを使用し、より美しく、ユーザフレンドリーに表示します。

<!--- Here, `fmt::Display` was used because the std library provides implementations --->
<!--- for these types. To print text for custom types, more steps are required. --->
この例で用いられている型は、標準ライブラリに含まれているため、ここでは`fmt::Display`を使用しています。他の型の場合、もうちょっと複雑なステップが要求される場合があります。

### 演習

<!---  * Fix the two issues in the above code (see FIXME) so that it runs without --->
   <!--- error. --->
 * 上の例を実行した際に生じるエラーを修復しましょう。
 * `println!`マクロを追加し、`Pi is rough ly 3.143`,という文字列を出力しましょう。
   ただし、3.143は22を7で割ることで作成してください。(ヒント: 10進数の数を出力する方法については、[`std::fmt`][fmt]をチェックする必要があるかもしれません。)
<!---  * Add a `println!` macro that prints: `Pi is roughly 3.143`, using twenty-two --->
   <!--- divided by seven to generate the estimate for Pi. (Hint: you may need to --->
   <!--- check the [`std::fmt`][fmt] documentation for setting the number of --->
   <!--- decimals to display) --->

### See also

[`std::fmt`][fmt],[マクロ][macros],[構造体][structs],
and [トレイト][traits]

[fmt]: http://doc.rust-lang.org/std/fmt/
[macros]: ../macros.html
[string]: ../std/str.html
[structs]: ../custom_types/structs.html
[traits]: ../trait.html
