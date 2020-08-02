<!--
# Unpacking options with `?`
-->
# `?`による`Option`のアンパック

<!--
You can unpack `Option`s by using `match` statements, but it's often easier to
use the `?` operator. If `x` is an `Option`, then evaluating `x?` will return
the underlying value if `x` is `Some`, otherwise it will terminate whatever
function is being executed and return `None`.
-->
`Option`をアンパックするには`match`文を使うこともできますが、`?`を使う方が簡単になることが多いでしょう。`Option`の`x`があるとすると、`x?`を評価した値は、`x`が`Some`の場合は`x`の値となり、そうでなければ実行中の関数を終了させ、`None`を返します。


```rust,editable
fn next_birthday(current_age: Option<u8>) -> Option<String> {
	// If `current_age` is `None`, this returns `None`.
	// If `current_age` is `Some`, the inner `u8` gets assigned to `next_age`
    // `current_age`が`None`の場合、`None`を返す。
    // `current_age`が`Some`の場合、内部の`u8`型の値が`next_age`に代入される。
    let next_age: u8 = current_age?;
    Some(format!("Next year I will be {}", next_age))
}
```

<!--
You can chain many `?`s together to make your code much more readable.
-->
多くの`?`を共に使うことで、リーダブルなコードを描くことができます。

```rust,editable
struct Person {
    job: Option<Job>,
}

#[derive(Clone, Copy)]
struct Job {
    phone_number: Option<PhoneNumber>,
}

#[derive(Clone, Copy)]
struct PhoneNumber {
    area_code: Option<u8>,
    number: u32,
}

impl Person {

    // Gets the area code of the phone number of the person's job, if it exists.
    // その人の市外局番が存在する場合、取得する。
    fn work_phone_area_code(&self) -> Option<u8> {
        // This would need many nested `match` statements without the `?` operator.
        // It would take a lot more code - try writing it yourself and see which
        // is easier.
        // `?`がなければ、多くのネストされた`match`文を必要とするため、より長いコードとなる。
        // 実際に書いて、どちらの方が簡単か確かめてみましょう。
        self.job?.phone_number?.area_code
    }
}

fn main() {
    let p = Person {
        job: Some(Job {
            phone_number: Some(PhoneNumber {
                area_code: Some(61),
                number: 439222222,
            }),
        }),
    };

    assert_eq!(p.work_phone_area_code(), Some(61));
}
```
