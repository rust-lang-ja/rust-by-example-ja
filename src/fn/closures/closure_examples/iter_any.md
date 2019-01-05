<!-- `Iterator::any` is a function which when passed an iterator, will return
`true` if any element satisfies the predicate. Otherwise `false`. Its
signature: -->
`iterator::any`は、イテレータ内に一つでも条件を満たす要素があれば、`true`を返し、さもなくば`false`を返すイテレータです。以下がそのシグネチャです

``` rust,ignore
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

``` rust,editable
fn main() {
    let vec1 = vec![1, 2, 3];
    let vec2 = vec![4, 5, 6];

    // ベクトル型に対する`iter`は`&i32`を`yield`するので、`i32`へとデストラクト
    println!("2 in vec1: {}", vec1.iter()     .any(|&x| x == 2));
    // `into_iter()`の場合は`i32`を`yield`するので、デストラクトする必要はない。
    println!("2 in vec2: {}", vec2.into_iter().any(| x| x == 2));

    let array1 = [1, 2, 3];
    let array2 = [4, 5, 6];

    // 配列に対する`iter()`は`&i32`をyieldする。
    println!("2 in array1: {}", array1.iter()     .any(|&x| x == 2));
    // 配列に`into_iter()`を使うと例外的に`&i32`を`yield`する。
    println!("2 in array2: {}", array2.into_iter().any(|&x| x == 2));
}

```

### See also:

[`std::iter::Iterator::any`][any]

[any]: http://doc.rust-lang.org/std/iter/trait.Iterator.html#method.any
