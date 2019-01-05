<!-- `Iterator::find` is a function which when passed an iterator, will return
the first element which satisfies the predicate as an `Option`. Its
signature: -->
`Iterator::find`はイテレータに渡される関数で、条件を満たす最初の値を`Option`として返します。型シグネチャは以下のようになります。

```rust
pub trait Iterator {
    // イテレートされる値の型
    type Item;

    // `find`は`&mut self`を取るため、イテレータを呼び出した値を借用し
    // 変更しますが、消費し尽くすことはありません。
    fn find<P>(&mut self, predicate: P) -> Option<Self::Item> where
        // `FnMut`はクロージャによって補足される変数が変更される
        // 事はあっても消費されることはないということを示します。
        // `&Self::Item`はクロージャが変数を参照として取ることを示します。
        P: FnMut(&Self::Item) -> bool {}
}
```

{iter_find.play}

### See also:

[`std::iter::Iterator::find`][find]

[find]: http://doc.rust-lang.org/std/iter/trait.Iterator.html#method.find
