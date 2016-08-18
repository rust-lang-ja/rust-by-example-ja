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
