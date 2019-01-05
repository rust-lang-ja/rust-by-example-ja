<!-- The `Iterator` trait is used to implement iterators over collections such as arrays. -->
`Iterator`トレイトは、例えば配列のような、要素の集合に対してイテレータを実装するためのトレイトです。

<!-- The trait requires only a method to be defined for the `next` element,
which may be manually defined in an `impl` block or automatically
defined (as in arrays and ranges). -->
このトレイトは`next`の要素に相当するものを決定するためのメソッドのみを要求します。このメソッドは`impl`ブロック内で手動で実装するか、あるいは(配列やrangeのように)自動で定義されます。

<!-- As a point of convenience for common situations, the `for` construct
turns some collections into iterators using the [`.into_iterator()`][intoiter] method. -->
サッとイテレータを使いたい時は、`for`文で集合からイテレータを作成することが良くあります。これは[`.into_iterator()`][intoiter]メソッドを呼び出しています。

<!-- Methods that can be accessed using the `Iterator` trait in addition
to those shown in the example below can be found [here][iter]. -->
`Iterator`トレイトからアクセスできるメソッドの一覧は[ここ][iter]にあります。以下の例ではその一部を使用しています。

``` rust,editable
struct Fibonacci {
    curr: u32,
    next: u32,
}

// `Iterator`を`Fibonacci`に対して実装する。
// `Iterator`トレイトは次(`next`)の要素を取得するメソッドの定義だけを要求する。
impl Iterator for Fibonacci {
    type Item = u32;

    // ここではイテレーションの流れを`.curr`と`.next`を使用して定義している。
    // 返り値の型は`Option<T>`で、これは:
    //     * `Iterator`が終了した時は`None`を返し、
    //     * そうでなければ`Some`でラップされた値を返す。
    fn next(&mut self) -> Option<u32> {
        let new_next = self.curr + self.next;

        self.curr = self.next;
        self.next = new_next;

        // フィボナッチ数列には終端がないので、`Iterator`は決して
        // `None`を返さず、常に`Some`が返される。
        Some(self.curr)
    }
}

// フィボナッチ数列のジェネレータを返す。
fn fibonacci() -> Fibonacci {
    Fibonacci { curr: 1, next: 1 }
}

fn main() {
    // `0..3`は0, 1, 2をジェネレートする`Iterator`
    let mut sequence = 0..3;

    println!("Four consecutive `next` calls on 0..3");
    println!("> {:?}", sequence.next());
    println!("> {:?}", sequence.next());
    println!("> {:?}", sequence.next());
    println!("> {:?}", sequence.next());

    // `for`は`None`を返すまで、イテレータを舐めていき、出てきた`Some`を
    // アンラップして変数(ここでは`i`)に束縛する。
    println!("Iterate through 0..3 using `for`");
    for i in 0..3 {
        println!("> {}", i);
    }

    // `take(n)`メソッドは`Iterator`をはじめから`n`番目の要素までの部分に減らす。
    println!("The first four terms of the Fibonacci sequence are: ");
    for i in fibonacci().take(4) {
        println!("> {}", i);
    }

    // `skip(n)`メソッドは`Iterator`のはじめから`n`番目までの要素をとばす。
    println!("The next four terms of the Fibonacci sequence are: ");
    for i in fibonacci().skip(4).take(4) {
        println!("> {}", i);
    }

    let array = [1u32, 3, 3, 7];

    // `iter`メソッドは配列やスライスからイテレータを作成する。
    println!("Iterate the following array {:?}", &array);
    for i in array.iter() {
        println!("> {}", i);
    }
}

```

[intoiter]: https://doc.rust-lang.org/std/iter/trait.IntoIterator.html
[iter]: http://doc.rust-lang.org/core/iter/trait.Iterator.html
