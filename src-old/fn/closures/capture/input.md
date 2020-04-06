<!--- Closures are inherently flexible and will do what the functionality requires --->
<!--- to make the closure work without annotation. This allows capturing to --->
<!--- flexibly adapt to the use case, sometimes moving and sometimes borrowing. --->
<!--- Closures can capture variables: --->
クロージャはとてもフレキシブルに動作するように出来ています。クロージャにおいて型アノテーションをする必要が無いのは前述の仕組みのためですが、この仕組みのおかげでユースケースに応じて参照を取得したり値そのものを取得したりといった動作が可能になります。

クロージャは外側の環境にある要素を、以下の形で取得することができます。

<!--- * by reference: `&T` --->
<!--- * by mutable reference: `&mut T` --->
<!--- * by value: `T` --->
* リファレンス: `&T`
* ミュータブルなリファレンス: `&mut T`
* 値そのもの: `T`

<!--- They preferentially capture variables by reference and only go lower when --->
<!--- required. --->
クロージャは出来る限りリファレンスを取得しようとし、その他2つは必要なときのみ取得します。

{capture.play}

### See also:

[`Box`][box] and [`std::mem::drop`][drop]

[box]: ../../std/box.html
[drop]: http://doc.rust-lang.org/std/mem/fn.drop.html
