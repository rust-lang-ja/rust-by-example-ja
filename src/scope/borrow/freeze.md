<!-- When data is immutably borrowed, it also *freezes*. *Frozen* data can't be
modified via the original object until all references to it go out of scope: -->
(訳注: ミュータブルな)データがイミュータブルに借用された場合、同時に*フリーズ*します。*フリーズ*したデータは、参照が全てスコープから出て行かないかぎり、元のオブジェクトを介して変更することはできません。


``` rust,editable,ignore,mdbook-runnable
fn main() {
    let mut _mutable_integer = 7i32;

    {
        // `_mutable_integer`を借用
        let _large_integer = &_mutable_integer;

        // エラー! `_mutable_integer`はこのスコープではフリーズしている。
        _mutable_integer = 50;
        // FIXME ^ この行をコメントアウトしましょう。

        // `_large_integer`はスコープを抜ける
    }

    // OK! `_mutable_integer`はこのスコープではフリーズしていない。
    _mutable_integer = 3;
}

```
