<!-- Mutable data can be mutably borrowed using `&mut T`. This is called
a *mutable reference* and gives read/write access to the borrower.
In contrast, `&T` borrows the data via an immutable reference, and
the borrower can read the data but not modify it: -->
ミュータブルなデータは`&mut T`でミュータブルに(変更可能な形で)借用することができます。これは*ミュータブルな参照*と呼ばれ、読み込み・書き込みの権限を借用者に与えます。対照的に`&T`はデータをイミュータブルな参照を介して借用し、借用した側はデータを読み込みはできますが書き込みはできません。

{mut.play}

### See Also
[`static`][static]

[static]: scope/lifetime/static_lifetime.html
