<!--- A tuple is a collection of values of different types. Tuples are constructed --->
<!--- using parentheses `()`, and each tuple itself is a value with type signature --->
<!--- `(T1, T2, ...)`, where `T1`, `T2` are the types of its members. Functions can --->
<!--- use tuples to return multiple values, as tuples can hold any number of values. --->
タプルは異なる型の値の集合です。括弧`()`を用いて生成します。タプル自体がそのメンバに対する型シグネチャを保持していますので、明示すると`(T1, T2, ...)`のようになります。タプルは大きさに制限がありませんので、関数が複数の値を返したい時に使われます。

{tuples.play}

### 演習

<!---  1. *Recap*: Add the `fmt::Display` trait to the Matrix `struct` in the above example, --->
<!--- so that if you switch from printing the debug format `{:?}` to the display --->
<!--- format `{}`, you see the following output: --->

 1. *復習*: 上にある`Matrix`という構造体に、`fmt::Display`トレイトを追加しましょう。デバッグフォーマット`{:?}`ではなくディスプレイフォーマット`{}`でプリントすることができるようになるはずです。

```
( 1.1 1.2 )
( 2.1 2.2 )
```
<!--- You may want to refer back to the example for [print display][print_display]. --->

必要に応じて[print displayのページ][print_display]に戻る必要があるかもしれません。

<!---  2. Add a `transpose` function using the `reverse` function as a template, which --->
<!--- accepts a matrix as an argument, and returns a matrix in which two elements --->
<!--- have been swapped. For example: --->

 2. `reverse`関数を雛形にした`transpose`関数を実装してください。この関数は`Matrix`を引数として受け取り、要素のうち2つを入れ替えたものを返します。つまり

```
println!("Matrix:\n{}", matrix)
println!("Transpose:\n{}", transpose(matrix))
```
は以下の様な出力になります:
```
Matrix:
( 1.1 1.2 )
( 2.1 2.2 )
Transpose:
( 1.1 2.1 )
( 1.2 2.2 )
```

[print_display]: ../hello/print/print_display.html
