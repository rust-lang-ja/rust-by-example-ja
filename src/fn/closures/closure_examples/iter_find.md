# Searching through iterators

<!--
`Iterator::find` is a function which iterates over an iterator and searches for the 
first value which satisfies some condition. If none of the values satisfy the 
condition, it returns `None`. Its signature:
-->
`Iterator::find`はイテレータを辿る関数で、条件を満たす最初の値を探します。もし条件を満たす値がなければ`None`を返します。型シグネチャは以下のようになります。

```rust,ignore
pub trait Iterator {
    // The type being iterated over.
    // イテレートされる値の型
    type Item;

    // `find` takes `&mut self` meaning the caller may be borrowed
    // and modified, but not consumed.
    // `find`は`&mut self`を取るため、イテレータを呼び出した値を借用し
    // 変更しますが、消費し尽くすことはありません。
    fn find<P>(&mut self, predicate: P) -> Option<Self::Item> where
        // `FnMut` meaning any captured variable may at most be
        // modified, not consumed. `&Self::Item` states it takes
        // arguments to the closure by reference.
        // `FnMut`はクロージャによって補足される変数が変更される
        // 事はあっても消費されることはないということを示します。
        // `&Self::Item`はクロージャが変数を参照として取ることを示します。
        P: FnMut(&Self::Item) -> bool {}
}
```

```rust,editable
fn main() {
    let vec1 = vec![1, 2, 3];
    let vec2 = vec![4, 5, 6];

    // `iter()` for vecs yields `&i32`.
    // ベクトル型に対する`iter`は`&i32`を`yield`する。
    let mut iter = vec1.iter();
    // `into_iter()` for vecs yields `i32`.
    // `inter_iter()`の場合は`i32`を`yield`する。
    let mut into_iter = vec2.into_iter();

    // `iter()` for vecs yields `&i32`, and we want to reference one of its
    // items, so we have to destructure `&&i32` to `i32`
    // `yield`された要素へのリファレンスは`&&i32`となる。`i32`へとデストラクトする。
    println!("Find 2 in vec1: {:?}", iter     .find(|&&x| x == 2));
    // `into_iter()` for vecs yields `i32`, and we want to reference one of
    // its items, so we have to destructure `&i32` to `i32`
    // `into_iter`の場合は`&i32`が要素のリファレンス。
    println!("Find 2 in vec2: {:?}", into_iter.find(| &x| x == 2));

    let array1 = [1, 2, 3];
    let array2 = [4, 5, 6];

    // `iter()` for arrays yields `&i32`
    // 配列に対する`iter`も`&i32`を`yield`する。
    println!("Find 2 in array1: {:?}", array1.iter()     .find(|&&x| x == 2));
    // `into_iter()` for arrays unusually yields `&i32`
    // 配列に`into_iter()`を使うと例外的に`&i32`を`yield`する。
    println!("Find 2 in array2: {:?}", array2.into_iter().find(|&&x| x == 2));
}
```

<!--
`Iterator::find` gives you a reference to the item. But if you want the _index_ of the
item, use `Iterator::position`.
-->
`Iterator::find`はアイテムへの参照を返します。
アイテムの _インデックス_ を使用したい場合、`Iterator::position`を使用してください。

```rust,editable
fn main() {
    let vec = vec![1, 9, 3, 3, 13, 2];

    let index_of_first_even_number = vec.iter().position(|x| x % 2 == 0);
    assert_eq!(index_of_first_even_number, Some(5));
    
    
    let index_of_first_negative_number = vec.iter().position(|x| x < &0);
    assert_eq!(index_of_first_negative_number, None);
}
```

<!--
### See also:
-->
### 参照

[`std::iter::Iterator::find`][find]

[`std::iter::Iterator::find_map`][find_map]

[`std::iter::Iterator::position`][position]

[`std::iter::Iterator::rposition`][rposition]

[find]: https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.find
[find_map]: https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.find_map
[position]: https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.position
[rposition]: https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.rposition
