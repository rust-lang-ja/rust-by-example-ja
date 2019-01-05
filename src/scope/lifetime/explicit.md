<!-- The borrow checker uses explicit lifetime annotations to determine
how long references should be valid. In cases where lifetimes are not
elided[^1], Rust requires explicit annotations to determine what the
lifetime of a reference should be. The syntax for explicitly annotating
a lifetime uses an apostrophe character as follows: -->
借用チェッカーは参照がどれだけの間有効かを決定するために、明示的なアノテーションを使用します。ライフタイムが省略[^1]されなかった場合、Rustは参照のライフタイムがどのようなものであるか、明示的なアノテーションを必要とします。

``` rust,ignore
foo<'a>
// `foo`は`'a`というライフタイムパラメータを持ちます。
```

<!-- Similar to [closures][anonymity], using lifetimes requires generics.
Additionally, this lifetime syntax indicates that the lifetime of `foo`
may not exceed that of `'a`. Explicit annotation of a type has the form
`&'a T` where `'a` has already been introduced. -->
[クロージャ][anonymity]と同様、ライフタイムの使用はジェネリクスを必要とします。もう少し詳しく言うと、この書き方は「`foo`のライフタイムは`'a`のそれを超えることはない。」ということを示しており、型を明示した場合`'a`は`&'a T`となるということです。

<!-- In cases with multiple lifetimes, the syntax is similar: -->
ライフタイムが複数ある場合も、同じような構文になります。

``` rust,ignore
foo<'a, 'b>
// `foo` has lifetime parameters `'a` and `'b`
// `foo`は`'a`と`'b`というライフタイムパラメータを持ちます。
```

<!-- In this case, the lifetime of `foo` cannot exceed that of either `'a` *or* `'b`.

See the following example for explicit lifetime annotation in use: -->
この場合は、`foo`のライフタイムは`'a`、`'b`の*いずれよりも*長くなってはなりません。
以下はライフタイムを明示的に書く場合の例です。

``` rust,editable
// `print_refs`は`i32`への参照を2つとり、それぞれ`'a`と`'b`という
// ライフタイムを持つ。これらのライフタイムは最短でも`print_refs`
// 関数と同じになる。
fn print_refs<'a, 'b>(x: &'a i32, y: &'b i32) {
    println!("x is {} and y is {}", x, y);
}

// 引数を取らないがライフタイムパラメータ`'a`を持つ関数。
fn failed_borrow<'a>() {
    let _x = 12;

    // エラー: `_x`の寿命が短すぎる。
    //let y: &'a i32 = &_x;
    // `&_x`のライフタイムは`y`のそれよりも短いため、関数内で`'a`を使用して
    // 変数のライフタイムを指定しようとすると失敗する。つまり、短いライフタイム
    // を持つ参照をより長いものに強制的に代入することはできない。
}

fn main() {
    // 下で借用するための変数を作成
    let (four, nine) = (4, 9);

    // 2つの変数の借用(`&`)が関数に渡される。
    print_refs(&four, &nine);
    // 借用された変数の寿命は、借り手のそれよりも長くなくてはならない。
    // つまり、`four`、`nine`のライフタイムは`print_refs`のそれよりも
    // 長くなくてはならない。

    failed_borrow();
    // `failed_borrow`は関数のライフタイムよりも`'a`を長くさせるような
    // 参照を持たないが、それでも`'a`のほうが長くなる。なぜならそのような
    // 場合`'a`はデフォルトで`'static`になるからである。
}

```

<!-- [^1]: [elision][elision] implicitly annotates lifetimes and so is different.-->
[^1]: [省略][elision] はライフタイムが暗黙のうちに(プログラマから見えない形で)アノテートされることを指します。

### See also:

[ジェネリクス][generics]、 [クロージャ][closures]

[anonymity]: /fn/closures/anonymity.html
[closures]: /fn/closures.html
[elision]: /scope/lifetime/elision.html
[generics]: /generics.html
