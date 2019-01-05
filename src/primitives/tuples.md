<!--- A tuple is a collection of values of different types. Tuples are constructed --->
<!--- using parentheses `()`, and each tuple itself is a value with type signature --->
<!--- `(T1, T2, ...)`, where `T1`, `T2` are the types of its members. Functions can --->
<!--- use tuples to return multiple values, as tuples can hold any number of values. --->
タプルは異なる型の値の集合です。括弧`()`を用いて生成します。タプル自体がそのメンバに対する型シグネチャを保持していますので、明示すると`(T1, T2, ...)`のようになります。タプルは大きさに制限がありませんので、関数が複数の値を返したい時に使われます。

``` rust,editable
// タプルを関数の引数及び返り値として使用している。
fn reverse(pair: (i32, bool)) -> (bool, i32) {
    // `let`でタプルの中の値を別の変数に束縛することができる。
    let (integer, boolean) = pair;

    (boolean, integer)
}

// 以下の構造体は後ほど「演習」で用いる。
#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

fn main() {
    // 様々な型を値に持つタプル
    let long_tuple = (1u8, 2u16, 3u32, 4u64,
                      -1i8, -2i16, -3i32, -4i64,
                      0.1f32, 0.2f64,
                      'a', true);

    // インデックスを用いて、タプル内の要素を参照できる。
    println!("long tuple first value: {}", long_tuple.0);
    println!("long tuple second value: {}", long_tuple.1);

    // タプルはタプルのメンバになれる
    let tuple_of_tuples = ((1u8, 2u16, 2u32), (4u64, -1i8), -2i16);

    // タプルはプリント可能である。
    println!("tuple of tuples: {:?}", tuple_of_tuples);

    let pair = (1, true);
    println!("pair is {:?}", pair);

    println!("the reversed pair is {:?}", reverse(pair));

    // 要素を1つしか持たないタプルを作成する場合、括弧で囲まれたただのリテラル
    // と区別するため、カンマが必要になる。
    println!("one element tuple: {:?}", (5u32,));
    println!("just an integer: {:?}", (5u32));

    //タプルを分解して別の変数にそれぞれの値を代入
    let tuple = (1, "hello", 4.5, true);

    let (a, b, c, d) = tuple;
    println!("{:?}, {:?}, {:?}, {:?}", a, b, c, d);

    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("{:?}", matrix)

}

```

### 演習

<!---  1. *Recap*: Add the `fmt::Display` trait to the Matrix `struct` in the above example, --->
<!--- so that if you switch from printing the debug format `{:?}` to the display --->
<!--- format `{}`, you see the following output: --->

 1. *復習*: 上にある`Matrix`という構造体に、`fmt::Display`トレイトを追加しましょう。デバッグフォーマット`{:?}`ではなくディスプレイフォーマット`{}`でプリントすることができるようになるはずです。

``` text
( 1.1 1.2 )
( 2.1 2.2 )
```
<!--- You may want to refer back to the example for [print display][print_display]. --->

必要に応じて[print displayのページ][print_display]に戻る必要があるかもしれません。

<!---  2. Add a `transpose` function using the `reverse` function as a template, which --->
<!--- accepts a matrix as an argument, and returns a matrix in which two elements --->
<!--- have been swapped. For example: --->

 2. `reverse`関数を雛形にした`transpose`関数を実装してください。この関数は`Matrix`を引数として受け取り、要素のうち2つを入れ替えたものを返します。つまり

``` rust,ignore
println!("Matrix:\n{}", matrix)
println!("Transpose:\n{}", transpose(matrix))
```
は以下の様な出力になります:
``` text
Matrix:
( 1.1 1.2 )
( 2.1 2.2 )
Transpose:
( 1.1 2.1 )
( 1.2 2.2 )
```

[print_display]: ../hello/print/print_display.html
