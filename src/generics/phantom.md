<!-- A phantom type parameter is one that doesn't show up at runtime,
but is checked statically (and only) at compile time. -->
幽霊型(Phantom Type)とは実行時には存在しないけれども、コンパイル時に静的に型チェックされるような型のことです。

<!-- Data types can use extra generic type parameters to act as markers
or to perform compile-time type checking. These extra parameters
hold no storage values, and have no run-time behavior. -->
構造体などのデータ型は、ジェネリック型パラメータを一つ余分に持ち、それをマーカーとして使ったりコンパイル時の型検査に使ったりすることができます。このマーカーは実際の値を何も持たず、したがって実行時の挙動そのものにはいかなる影響ももたらしません。

<!-- In the following example, we combine [std::marker::PhantomData]
with the phantom type parameter concept to create tuples containing
different data types. -->
以下の例では、そのようなマーカーとして幽霊型([std::marker::PhantomData])を用い、それぞれ異なった型の値を持つタプルを作成します。

``` rust,editable
use std::marker::PhantomData;

// ジェネリックなタプル構造体。2つ目のパラメータは幽霊型
#[derive(PartialEq)] // 比較演算子(`==`)での比較を可能にする。
struct PhantomTuple<A, B>(A,PhantomData<B>);

// 同様に構造体を定義
#[derive(PartialEq)] // 比較演算子での比較を可能にする。
struct PhantomStruct<A, B> { first: A, phantom: PhantomData<B> }

// 注意点:  ジェネリック型Aに対してはメモリが割り当てられているが、
//          Bには割り当てられていないため、計算に使うことはできない。

fn main() {
    // <char, f32>と型宣言されたPhantomTupleを作成
    let _tuple1: PhantomTuple<char, f32> = PhantomTuple('Q', PhantomData);
    // <chr, f64>のPhantomTuple。 PhantomDataがいかなる浮動小数点でもないことに注目
    let _tuple2: PhantomTuple<char, f64> = PhantomTuple('Q', PhantomData);

    // <char, f32>の型が与えられた構造体を作成
    let _struct1: PhantomStruct<char, f32> = PhantomStruct {
        first: 'Q',
        phantom: PhantomData,
    };
    // 同様に<char, f64>の構造体
    let _struct2: PhantomStruct<char, f64> = PhantomStruct {
        first: 'Q',
        phantom: PhantomData,
    };

    // コンパイルエラー！型が違うので`==`で比較することができない！
    //println!("_tuple1 == _tuple2 yields: {}",
    //          _tuple1 == _tuple2);

    // コンパイルエラー! 型が違うので比較することができない!
    //println!("_struct1 == _struct2 yields: {}",
    //          _struct1 == _struct2);
}

```

### See also:

[継承(`Derive`)][Derive], [構造体][struct], [タプル][TupleStructs]

[Derive]: ../trait/derive.html
[struct]: ../custom_types/structs.html
[TupleStructs]: ../custom_types/structs.html
[std::marker::PhantomData]: https://doc.rust-lang.org/std/marker/struct.PhantomData.html
