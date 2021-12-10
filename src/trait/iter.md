<!--
# Iterators
-->
# イテレータ

<!--
The [`Iterator`][iter] trait is used to implement iterators over collections such as arrays.
-->
[`Iterator`][iter]トレイトは、例えば配列のような、要素の集合に対してイテレータを実装するためのトレイトです。

<!--
The trait requires only a method to be defined for the `next` element, 
which may be manually defined in an `impl` block or automatically 
defined (as in arrays and ranges).
-->
このトレイトは`next`の要素に相当するものを決定するためのメソッドのみを要求します。このメソッドは`impl`ブロック内で手動で実装するか、あるいは（配列やrangeのように）自動で定義されます。

<!--
As a point of convenience for common situations, the `for` construct 
turns some collections into iterators using the [`.into_iter()`][intoiter] method.
-->
サッとイテレータを使いたい時は、`for`文で集合からイテレータを作成することが良くあります。これは[`.into_iter()`][intoiter]メソッドを呼び出しています。

```rust,editable
struct Fibonacci {
    curr: u32,
    next: u32,
}

// Implement `Iterator` for `Fibonacci`.
// The `Iterator` trait only requires a method to be defined for the `next` element.
// `Iterator`を`Fibonacci`に対して実装する。
// `Iterator`トレイトは次(`next`)の要素を取得するメソッドの定義だけを要求する。
impl Iterator for Fibonacci {
    // We can refer to this type using Self::Item
    type Item = u32;
    
    // Here, we define the sequence using `.curr` and `.next`.
    // The return type is `Option<T>`:
    //     * When the `Iterator` is finished, `None` is returned.
    //     * Otherwise, the next value is wrapped in `Some` and returned.
    // We use Self::Item in the return type, so we can change
    // the type without having to update the function signatures.
    // ここではイテレーションの流れを`.curr`と`.next`を使用して定義している。
    // 返り値の型は`Option<T>`で、これは:
    //     * `Iterator`が終了した時は`None`を返し、
    //     * そうでなければ`Some`でラップされた値を返す。
    fn next(&mut self) -> Option<Self::Item> {
        let new_next = self.curr + self.next;

        self.curr = self.next;
        self.next = new_next;

        // Since there's no endpoint to a Fibonacci sequence, the `Iterator` 
        // will never return `None`, and `Some` is always returned.
        // フィボナッチ数列には終端がないので、`Iterator`は決して
        // `None`を返さず、常に`Some`が返される。
        Some(self.curr)
    }
}

// Returns a Fibonacci sequence generator
// フィボナッチ数列のジェネレータを返す。
fn fibonacci() -> Fibonacci {
    Fibonacci { curr: 0, next: 1 }
}

fn main() {
    // `0..3` is an `Iterator` that generates: 0, 1, and 2.
    // `0..3`は0, 1, 2をジェネレートする`Iterator`
    let mut sequence = 0..3;

    println!("Four consecutive `next` calls on 0..3");
    println!("> {:?}", sequence.next());
    println!("> {:?}", sequence.next());
    println!("> {:?}", sequence.next());
    println!("> {:?}", sequence.next());

    // `for` works through an `Iterator` until it returns `None`.
    // Each `Some` value is unwrapped and bound to a variable (here, `i`).
    // `for`は`None`を返すまで、イテレータを舐めていき、出てきた`Some`を
    // アンラップして変数（ここでは`i`）に束縛する。
    println!("Iterate through 0..3 using `for`");
    for i in 0..3 {
        println!("> {}", i);
    }

    // The `take(n)` method reduces an `Iterator` to its first `n` terms.
    // `take(n)`メソッドは`Iterator`をはじめから`n`番目の要素までの部分に減らす。
    println!("The first four terms of the Fibonacci sequence are: ");
    for i in fibonacci().take(4) {
        println!("> {}", i);
    }

    // The `skip(n)` method shortens an `Iterator` by dropping its first `n` terms.
    // `skip(n)`メソッドは`Iterator`のはじめから`n`番目までの要素をとばす。
    println!("The next four terms of the Fibonacci sequence are: ");
    for i in fibonacci().skip(4).take(4) {
        println!("> {}", i);
    }

    let array = [1u32, 3, 3, 7];

    // The `iter` method produces an `Iterator` over an array/slice.
    // `iter`メソッドは配列やスライスからイテレータを作成する。
    println!("Iterate the following array {:?}", &array);
    for i in array.iter() {
        println!("> {}", i);
    }
}
```

[intoiter]: https://doc.rust-lang.org/std/iter/trait.IntoIterator.html
[iter]: https://doc.rust-lang.org/core/iter/trait.Iterator.html
