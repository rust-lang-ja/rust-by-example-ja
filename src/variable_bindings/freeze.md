<!--
# Freezing
-->
# 値のフリーズ
<!--
When data is bound by the same name immutably, it also *freezes*. *Frozen* data can't be 
modified until the immutable binding goes out of scope:
-->
データを同じ名前のイミュータブルな変数に束縛することを、データを*フリーズ*するといいます。*フリーズされた*データは、イミュータブルな束縛がスコープ外になるまで変更できません。

```rust,editable,ignore,mdbook-runnable
fn main() {
    let mut _mutable_integer = 7i32;

    {
        // Shadowing by immutable `_mutable_integer`
        // イミュータブルな`_mutable_integer`でシャドーイングする
        let _mutable_integer = _mutable_integer;

        // Error! `_mutable_integer` is frozen in this scope
        // エラー! `_mutable_integer`はこのスコープではフリーズしている。
        _mutable_integer = 50;
        // FIXME ^ Comment out this line
        // FIXME ^ この行をコメントアウトしましょう。

        // `_mutable_integer` goes out of scope
        // `_mutable_integer`はスコープを抜ける
    }

    // Ok! `_mutable_integer` is not frozen in this scope
    // OK! `_mutable_integer`はこのスコープではフリーズしていない。
    _mutable_integer = 3;
}
```
