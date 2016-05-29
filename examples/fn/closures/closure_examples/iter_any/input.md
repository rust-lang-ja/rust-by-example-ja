<!-- `Iterator::any` is a function which when passed an iterator, will return
`true` if any element satisfies the predicate. Otherwise `false`. Its
signature: -->
`iterator::any`は、イテレータ内に一つでも条件を満たす要素があれば、`true`を返し、さもなくば`false`を返すイテレータです。以下がそのシグネチャです

```rust
pub trait Iterator {
    // イテレートされる値の型
    type Item;

    // `any`は`&mut self`を取るため、イテレータを呼び出した値を借用し
    // 変更しますが、消費し尽くすことはありません。
    fn any<F>(&mut self, f: F) -> bool where
        // `FnMut`はクロージャによって補足される変数が変更される
        // 事はあっても消費されることはないということを示します。
        // `&Self::Item`はクロージャが変数を参照として取ることを示します。
        F: FnMut(Self::Item) -> bool {}
}
```

{iter_any.play}

### See also:

[`std::iter::Iterator::any`][any]

[any]: http://doc.rust-lang.org/std/iter/trait.Iterator.html#method.any
