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
