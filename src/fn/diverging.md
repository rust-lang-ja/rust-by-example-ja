<!--
# Diverging functions
-->

# 発散する関数

<!--
Diverging functions never return. They are marked using `!`, which is an empty type.
-->

発散する関数 (Diverging functions) は決してリターンしない関数です。こうした関数は `!` を使って、空の型であることが示されます。

```rust
fn foo() -> ! {
    // この呼び出しは決してリターンしない。
    panic!("This call never returns.");
}
```
<!--
As opposed to all the other types, this one cannot be instantiated, because the
set of all possible values this type can have is empty. Note that, it is
different from the `()` type, which has exactly one possible value.
-->

他の全ての型と異なり、この型はインスタンス化できません。
この型が持ちうる全ての値の集合は空です。
この型は`()`型とは異なることに注意してください。
`()`型は値をただ1つだけ持つ型です。

<!--
For example, this function returns as usual, although there is no information
in the return value.
-->

例えば、この関数は通常どおりリターンしますが、戻り値には何の情報も含みません。

```rust
fn some_fn() {
    ()
}

fn main() {
    let a: () = some_fn();
    // この関数はリターンするので、この行は実行される。
    println!("This function returns and you can see this line.")
}
```

<!--
As opposed to this function, which will never return the control back to the caller.
-->

一方、この関数は呼び出し元に決してリターンしません。

```rust,ignore
#![feature(never_type)]

fn main() {
    // この呼び出しは決してリターンしない。
    let x: ! = panic!("This call never returns.");
    // この行は決して実行されない。
    println!("You will never see this line!");
}
```

<!--
Although this might seem like an abstract concept, it is in fact very useful and
often handy. The main advantage of this type is that it can be cast to any other
one and therefore used at places where an exact type is required, for instance
in `match` branches. This allows us to write code like this:
-->

これは抽象的な概念に見えるかもしれませんが、実際のところはとても実用的で、便利なことも多いのです。
この型の主な利点は、他のどのような型にもキャストできることです。
そのため、例えば`match`の分岐の中のような正確な型が要求される場所でも使用できます。

```rust
fn main() {
    fn sum_odd_numbers(up_to: u32) -> u32 {
        let mut acc = 0;
        for i in 0..up_to {
            // Notice that the return type of this match expression must be u32
            // because of the type of the "addition" variable.
            // 変数"addition"の型がu32であるため、
            // このmatch式はu32をリターンしなければならないことに注意。
            let addition: u32 = match i%2 == 1 {
                // The "i" variable is of type u32, which is perfectly fine.
                // 変数"i"はu32型であるため、全く問題ない。
                true => i,
                // On the other hand, the "continue" expression does not return
                // u32, but it is still fine, because it never returns and therefore
                // does not violate the type requirements of the match expression.
                // 一方、"continue"式はu32をリターンしないが、これでも問題ない。
                // 決してリターンしないため、このmatch式が要求する型に違反しないからである。
                false => continue,
            };
            acc += addition;
        }
        acc
    }
    println!("Sum of odd numbers up to 9 (excluding): {}", sum_odd_numbers(9));
}
```

<!--
It is also the return type of functions that loop forever (e.g. `loop {}`) like
network servers or functions that terminate the process (e.g. `exit()`).
-->

この型は、ネットワークサーバのような永遠にループする関数（例：`loop {}`）の戻り値の型や、プロセスを終了させる関数（例：`exit()`）の戻り値の型としても使用されます。
