<!--
# New Type Idiom
-->
# ニュータイプイディオム

<!--
The `newtype` idiom gives compile time guarantees that the right type of value is supplied
to a program.
-->
既に定義済みの型であっても`struct`を使うことであたかも別の型`newtype`であるかのように定義することが可能です。なお、それらが正しい型としてプログラムに供給されているか否かは、コンパイル時に保証されます。

<!--
For example, an age verification function that checks age in years, *must* be given
a value of type `Years`.
-->
例えば、年齢を年単位で確認する`old_enough`には「Years」という型の値を *与えなければならない* ようにすることが可能です。

```rust, editable
struct Years(i64);

struct Days(i64);

impl Years {
    pub fn to_days(&self) -> Days {
        Days(self.0 * 365)
    }
}


impl Days {
    /// truncates partial years
    /// 1年に満たない日付は切り捨て
    pub fn to_years(&self) -> Years {
        Years(self.0 / 365)
    }
}

fn old_enough(age: &Years) -> bool {
    age.0 >= 18
}

fn main() {
    let age = Years(5);
    let age_days = age.to_days();
    println!("Old enough {}", old_enough(&age));
    println!("Old enough {}", old_enough(&age_days.to_years()));
    // println!("Old enough {}", old_enough(&age_days));
}
```

<!--
Uncomment the last print statement to observe that the type supplied must be `Years`.
-->
最後の print文 のコメントを外して、与えられた型が `Years` でなければならないことを確認してください。

<!--
To obtain the `newtype`'s value as the base type, you may use the tuple or destructuring syntax like so:
-->
`newtype`の元に使われている型のデータを取得するには、以下のようにタプルや分配構文を用いることで取得できます。

```rust, editable
struct Years(i64);

fn main() {
    let years = Years(42);
    let years_as_primitive_1: i64 = years.0; // Tuple
    let Years(years_as_primitive_2) = years; // Destructuring
}
```

<!--
### See also:
-->
### 参照

[`structs`][struct]

[struct]: ../custom_types/structs.md

