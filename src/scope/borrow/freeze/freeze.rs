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
