<!--
# Freezing
-->

# 値の凍結

<!--
When data is bound by the same name immutably, it also *freezes*. *Frozen* data can't be
modified until the immutable binding goes out of scope:
-->

イミュータブル（変更不可能）な変数で凍結されると、変更できなくなります。
変更できなくなった変数は、凍結されたスコープを抜けるまでは、変更できないです。

```rust,editable,ignore,mdbook-runnable
fn main() {
    let mut _mutable_integer = 7i32;

    {
        // Shadowing by immutable `_mutable_integer`
        let _mutable_integer = _mutable_integer;

        // Error! `_mutable_integer` is frozen in this scope
        // エラー! `_mutable_integer`はこのスコープでは凍結している。
        _mutable_integer = 50;
        // FIXME ^ Comment out this line
        // FIXME ^ この行をコメントアウトしましょう。

        // `_mutable_integer` goes out of scope
        // `_mutable_integer`はスコープを抜ける
    }

    // Ok! `_mutable_integer` is not frozen in this scope
    // OK! `_mutable_integer`はこのスコープでは凍結していない。
    _mutable_integer = 3;
}
```
