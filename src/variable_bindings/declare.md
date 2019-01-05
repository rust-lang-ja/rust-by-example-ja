<!-- It's possible to declare variable bindings first, and initialize them later.
However, this form is seldom used, as it may lead to the use of uninitialized
variables. -->
変数の宣言だけを行っておき、初期化（定義）をのちに行うことも可能です。

しかし、最後まで初期化されない変数が生じる可能性があるため、ふつうは同時に行われます。


``` rust,editable,ignore,mdbook-runnable
fn main() {
    // 変数を宣言
    let a_binding;

    {
        let x = 2;

        // 変数を初期化
        a_binding = x * x;
    }

    println!("a binding: {}", a_binding);

    let another_binding;

    // Error! Use of uninitialized binding
    // エラー！ 初期化していない変数の使用
    println!("another binding: {}", another_binding);
    // FIXME ^ この行をコメントアウトしましょう。

    another_binding = 1;

    println!("another binding: {}", another_binding);
}

```

<!--
The compiler forbids use of uninitialized variables, as this would lead to
undefined behavior.
-->

未初期化の変数があると予期せぬ動作をする場合があるため、コンパイラは変数を初期化してから使用するよう強制します。
