<!-- A consequence of how bounds work is that even if a `trait` doesn't
include any functionality, you can still use it as a bound. `Eq` and
`Ord` are examples of such `trait`s from the `std` library. -->
トレイト境界の仕組みから、「トレイトがなにも機能を持っていなくとも境界条件として使用できることには変わりはない」という帰結がもたらされます。`Eq`と`Ord`は`std`ライブラリにおけるそのような例です。

{empty.play}

### See also:

[`std::cmp::Eq`][eq], [`std::cmp::Ord`s][ord], [トレイト][traits]

[eq]: http://doc.rust-lang.org/std/cmp/trait.Eq.html
[ord]: http://doc.rust-lang.org/std/cmp/trait.Ord.html
[traits]: ../trait.html
