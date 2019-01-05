<!-- `Iterator::find` is a function which when passed an iterator, will return
the first element which satisfies the predicate as an `Option`. Its
signature: -->
`Iterator::find`はイテレータに渡される関数で、条件を満たす最初の値を`Option`として返します。型シグネチャは以下のようになります。

``` rust,ignore
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

``` rust,editable
fn main() {
    let vec1 = vec![1, 2, 3];
    let vec2 = vec![4, 5, 6];

    // ベクトル型に対する`iter`は`&i32`を`yield`する。
    let mut iter = vec1.iter();
    // `inter_iter()`の場合は`i32`を`yield`する。
    let mut into_iter = vec2.into_iter();

    // `yield`された要素へのリファレンスは`&&i32`となる。`i32`へとデストラクトする。
    println!("Find 2 in vec1: {:?}", iter     .find(|&&x| x == 2));
    // `into_iter`の場合は`&i32`が要素のリファレンス。
    println!("Find 2 in vec2: {:?}", into_iter.find(| &x| x == 2));

    let array1 = [1, 2, 3];
    let array2 = [4, 5, 6];

    // 配列に対する`iter`も`&i32`を`yield`する。
    println!("Find 2 in array1: {:?}", array1.iter()     .find(|&&x| x == 2));
    // 配列に`into_iter()`を使うと例外的に`&i32`を`yield`する。
    println!("Find 2 in array2: {:?}", array2.into_iter().find(|&&x| x == 2));
}


```

### See also:

[`std::iter::Iterator::find`][find]

[find]: http://doc.rust-lang.org/std/iter/trait.Iterator.html#method.find
