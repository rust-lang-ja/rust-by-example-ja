<!--- We've seen that formatting is specified via a *format string*: --->
これまで、文字列がどのようにフォーマットされるかは*フォーマット文字列*によって決まるということを見てきました 。

* `format!("{}", foo)` -> `"3735928559"`
* `format!("0x{:X}", foo)` ->
  [`"0xDEADBEEF"`][deadbeef]
* `format!("0o{:o}", foo)` -> `"0o33653337357"`

<!--- The same variable (`foo`) can be formatted differently depending on which --->
<!--- *argument type* is used: `X` vs `o` vs *unspecified*. --->
ここでは （`foo`）という単一の変数が`X`、`o`、*指定なし*、という様々な*引数タイプ*(argument type)に応じてフォーマットされています。


<!--- This formatting functionality is implemented via traits, and there is one trait --->
<!--- for each argument type. The most common formatting trait is `Display`, which --->
<!--- handles cases where the argument type is left unspecified: `{}` for instance. --->
フォーマットの機能はそれぞれの引数タイプごとに個別のトレイトを用いて実装されています。
最も一般的なトレイトは`Display`で、これは引数タイプが未指定（たとえば`{}`）の時に呼び出されます。

{show.play}

<!--- You can view a [full list of formatting traits][fmt_traits] and their argument --->
<!--- types in the [`std::fmt`][fmt] documentation. --->
フォーマット用トレイトの全リスト、及び引数の型は[こちら][fmt_traits]から、引数の型については[`std::fmt`のドキュメンテーション][fmt]から参照できます。

<!--- ### Activity --->
### 演習

<!--- Add an implementation of the `fmt::Display` trait for the `Color` struct above --->
<!--- so that the output displays as: --->
上にあるソースコード中の`Color`という構造体のための`fmt::Display`トレイトの実装を追加しましょう。アウトプットは以下のように表示されるはずです。

```
RGB (128, 255, 90) 0x80FF5A
RGB (0, 3, 254) 0x0003FE
RGB (0, 0, 0) 0x000000
```
<!--- Two hints if you get stuck: --->
詰まったら以下の2つがヒントになります。
<!---  * You [may need to list each color more than once][argument_types], --->
<!---  * You can [pad with zeros to a width of 2][fmt_width] with `:02`. --->
* [それぞれの色を2回以上記述する必要があるかもしれません。][argument_typs]
* `:02`で、[幅を2に指定し、空白を0で埋める事ができます。][fmt_width]


### See also
[`std::fmt`][fmt]

[argument_types]: http://doc.rust-lang.org/std/fmt/#argument-types
[deadbeef]: https://en.wikipedia.org/wiki/Deadbeef#Magic_debug_values
[fmt]: http://doc.rust-lang.org/std/fmt/
[fmt_traits]: http://doc.rust-lang.org/std/fmt/#formatting-traits
[fmt_width]: http://doc.rust-lang.org/std/fmt/#width
