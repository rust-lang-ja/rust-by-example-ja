<!--
# Capturing
-->
# 要素の捕捉

<!--
Closures are inherently flexible and will do what the functionality requires
to make the closure work without annotation. This allows capturing to
flexibly adapt to the use case, sometimes moving and sometimes borrowing.
Closures can capture variables:
-->
クロージャはとてもフレキシブルに動作するように出来ています。クロージャにおいて型アノテーションをする必要が無いのは前述の仕組みのためですが、この仕組みのおかげでユースケースに応じて参照を取得したり値そのものを取得したりといった動作が可能になります。
クロージャは外側の環境にある要素を、以下の形で取得することができます。

<!--
* by reference: `&T`
* by mutable reference: `&mut T`
* by value: `T`
-->
* リファレンス: `&T`
* ミュータブルなリファレンス: `&mut T`
* 値そのもの: `T`

<!--
They preferentially capture variables by reference and only go lower when
required.
-->
クロージャは出来る限りリファレンスを取得しようとし、その他2つは必要なときのみ取得します。

```rust,editable
fn main() {
    use std::mem;
    
    let color = String::from("green");

    // A closure to print `color` which immediately borrows (`&`) `color` and
    // stores the borrow and closure in the `print` variable. It will remain
    // borrowed until `print` is used the last time. 
    //
    // `println!` only requires arguments by immutable reference so it doesn't
    // impose anything more restrictive.
    // `color`をプリントするためのクロージャ。
    // これは`color`を借用(`&`)し、その借用とクロージャを`print`
    // という名の変数に保持する。
    // 借用は`print`がスコープから出るまで続く。
    // `println!`は参照を与えれば機能するので、これ以上なにかする必要はない。
    let print = || println!("`color`: {}", color);

    // Call the closure using the borrow.
    // 借用を行ったクロージャをコールする。
    print();

    // `color` can be borrowed immutably again, because the closure only holds
    // an immutable reference to `color`. 
    // `color`を再びイミュータブルで借用することができる。
    // これはクロージャが`color`に対するイミュータブルな参照しか保持しないからである。
    let _reborrow = &color;
    print();

    // A move or reborrow is allowed after the final use of `print`
    // 最後に`print`を使用した後は移動や再借用が許可される。
    let _color_moved = color;


    let mut count = 0;
    // A closure to increment `count` could take either `&mut count` or `count`
    // but `&mut count` is less restrictive so it takes that. Immediately
    // borrows `count`.
    //
    // A `mut` is required on `inc` because a `&mut` is stored inside. Thus,
    // calling the closure mutates the closure which requires a `mut`.
    // `count`をインクリメントするためのクロージャ。`count`と`&mut count`
    // の両方を取ることができるが、後者のほうが制限が少ないため、
    // （訳注: `count`だと`&mut count`と違い、一度しか呼ぶことができない。）
    // そちらを取る。直後に`count`を借用する。
    //
    // `inc`には`mut`をつける必要がある。なぜならミュータブルな型が
    // 中で使用されているからである。ミュータブルなクロージャは呼ぶたびに
    // 内部変数を変更する。
    let mut inc = || {
        count += 1;
        println!("`count`: {}", count);
    };

    // Call the closure using a mutable borrow.
    // クロージャを実行
    inc();

    // The closure still mutably borrows `count` because it is called later.
    // An attempt to reborrow will lead to an error.
    // クロージャはまだ `count` をミュータブルで借用することができる。
    // なぜなら呼ばれのが後であるからである。
    // 再借用しようとするとエラーになる。
    // let _reborrow = &count; 
    // ^ TODO: try uncommenting this line.
    // ^ TODO: この行のコメントアウトを解除しましょう。
    inc();

    // The closure no longer needs to borrow `&mut count`. Therefore, it is
    // possible to reborrow without an error
    // クロージャはもう`&mut count`を借用する必要がない。
    // なので、エラーを起こさず再借用することができる。
    let _count_reborrowed = &mut count; 

    
    // A non-copy type.
    // コピー不可能な型
    let movable = Box::new(3);

    // `mem::drop` requires `T` so this must take by value. A copy type
    // would copy into the closure leaving the original untouched.
    // A non-copy must move and so `movable` immediately moves into
    // the closure.
    // `mem::drop`は`T`（ジェネリック型）を取る必要があるため、このクロージャは
    // 参照ではなく値を取る。その場合、もしもコピー可能な値ならば、
    // 元の値はそのままでコピーのみを取る。不可能ならば値そのものを移動させる。
    let consume = || {
        println!("`movable`: {:?}", movable);
        mem::drop(movable);
    };

    // `consume` consumes the variable so this can only be called once.
    // `consume`は変数を消費（開放）するため、一度しか呼び出すことができない。
    consume();
    // consume();
    // ^ TODO: Try uncommenting this line.
    // ^ TODO: この行のコメントアウトを解除しましょう。
}
```

<!--
Using `move` before vertical pipes forces closure
to take ownership of captured variables:
-->

バーティカルパイプ（訳注：縦線記号`||`）の前に`move`を使用することで、キャプチャする変数の所有権を取ることをクロージャに強制します。

```rust,editable
fn main() {
    // `Vec` has non-copy semantics.
    // `Vec`はコピー不可能なセマンティクスである。
    let haystack = vec![1, 2, 3];

    let contains = move |needle| haystack.contains(needle);

    println!("{}", contains(&1));
    println!("{}", contains(&4));

    // println!("There're {} elements in vec", haystack.len());
    // ^ Uncommenting above line will result in compile-time error
    // because borrow checker doesn't allow re-using variable after it
    // has been moved.
    // ^ 上の行のコメントアウトを解除すると、コンパイル時エラーになる。
    // これは変数の所有権が移された後の再利用を借用チェッカーが許可しないからである。
    
    // Removing `move` from closure's signature will cause closure
    // to borrow _haystack_ variable immutably, hence _haystack_ is still
    // available and uncommenting above line will not cause an error.
    // クロージャのシグネチャから`move`を削除すると、クロージャは _haystack_ 変数を
    // イミュータブルで借用するようになる。
    // そのため _haystack_ はまだ利用可能となり、上の行のコメントアウトを解除しても
    // エラーを発生させなくなる。
}
```

<!--
### See also:
-->
### 参照

[`Box`][box] and [`std::mem::drop`][drop]

[box]: ../../std/box.md
[drop]: https://doc.rust-lang.org/std/mem/fn.drop.html
