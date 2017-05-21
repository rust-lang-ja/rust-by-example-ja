<!-- The borrow checker uses explicit lifetime annotations to determine
how long references should be valid. In cases where lifetimes are not
elided[^1], Rust requires explicit annotations to determine what the
lifetime of a reference should be. The syntax for explicitly annotating
a lifetime uses an apostrophe character as follows: -->
借用チェッカーは参照がどれだけの間有効かを決定するために、明示的なアノテーションを使用します。ライフタイムが省略[^1]されなかった場合、Rustは参照のライフタイムがどのようなものであるか、明示的なアノテーションを必要とします。

```rust
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

```rust
foo<'a, 'b>
// `foo` has lifetime parameters `'a` and `'b`
// `foo`は`'a`と`'b`というライフタイムパラメータを持ちます。
```

<!-- In this case, the lifetime of `foo` cannot exceed that of either `'a` *or* `'b`.

See the following example for explicit lifetime annotation in use: -->
この場合は、`foo`のライフタイムは`'a`、`'b`の*いずれよりも*長くなってはなりません。
以下はライフタイムを明示的に書く場合の例です。

{explicit.play}

<!-- [^1]: [elision][elision] implicitly annotates lifetimes and so is different.-->
[^1]: [省略][elision] はライフタイムが暗黙のうちに(プログラマから見えない形で)アノテートされることを指します。

### See also:

[ジェネリクス][generics]、 [クロージャ][closures]

[anonymity]: /fn/closures/anonymity.html
[closures]: /fn/closures.html
[elision]: /scope/lifetime/elision.html
[generics]: /generics.html
