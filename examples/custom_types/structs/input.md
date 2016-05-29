<!--- There are three types of structures ("structs") that can be created using the --->
<!--- `struct` keyword: --->
`struct`というキーワードを用いて作成できる構造体(「structre」)には3種類あります。

<!--- * Tuple structs, which are, basically, named tuples. --->
<!--- * The classic [C structs][c_struct] --->
<!--- * Unit structs, which are field-less, are useful for generics. --->
* タプル。（ほとんどの場合は名前付きタプル）
* クラシックな[C言語スタイルの構造体。][c_struct]
* ユニット。これはフィールドを持たず、ジェネリック型を扱う際に有効です。

{structs.play}

### See also:

[アトリビュート(`attributes`)][attributes] and [デストラクト][destructuring]

[attributes]: ./attribute.html
[c_struct]: http://en.wikipedia.org/wiki/Struct_(C_programming_language)
[destructuring]: ./flow_control/match/destructuring.html
