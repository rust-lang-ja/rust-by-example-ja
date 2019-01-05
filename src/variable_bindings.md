<!-- Rust provides type safety via static typing. Variable bindings can be type
annotated when declared. However, in most cases, the compiler will be able
to infer the type of the variable from the context, heavily reducing the
annotation burden. -->
Rustは静的(`static`)な型付けゆえに型安全です。変数は宣言時に型を明示できます。とはいえたいていの場合は、コンパイラは変数の型をコンテキストから推測することができますので、常に苦労して明示する必要があるわけではありません。


<!-- Values (like literals) can be bound to variables, using the `let` binding.
-->
値は（リテラルに似て）`let`を用いて変数に束縛されることができる。

``` rust,editable
fn main() {
    let an_integer = 1u32;
    let a_boolean = true;
    let unit = ();

    // `an_integer`を`copied_integer`へとコピー
    let copied_integer = an_integer;

    println!("An integer: {:?}", copied_integer);
    println!("A boolean: {:?}", a_boolean);
    println!("Meet the unit value: {:?}", unit);

    // 使用されていない変数があると、コンパイラは警告を出します。
    // 変数名の頭に`_`(アンダーバー)を付けると警告を消すことができます。
    let _unused_variable = 3u32;

    let noisy_unused_variable = 2u32;
    // FIXME ^ 頭にアンダーバーを付けて、警告を抑えましょう。
}

```
