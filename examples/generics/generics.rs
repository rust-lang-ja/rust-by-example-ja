// `A`という具象型
struct A;

// `Single`という型を定義する際に`A`を使用しているが、その最初の使用よりも
// 先に`<A>`がないため、また、`A`自身も具象型であるため、`Single`は具象型となる。
struct Single(A);
//            ^ Singleによる`A`の一番最初の使用はここ

// ここでは`<T>`が一番初めの`T`の使用よりも先に来ている。よって`SingleGen`はジェネリック型
// となる。なぜならば型パラメータ`T`がジェネリックだからである。`T`はどんな方にもなりえるため、
// 上で定義した`A`を受け取ることもできる。
struct SingleGen<T>(T);

fn main() {
    // `Single`は具象型で、`A`のみを受け取る。
    let _s = Single(A);

    // `_char`という名の変数を生成する。これは`SingleGen<char>`
    // という型で、値は`SingleGen('a')`となる。ここでは、`SingleGen`には明示的な型パラメータ
    // が与えられている。
    let _char: SingleGen<char> = SingleGen('a');

    // `SingleGen`型の変数には明示的に型パラメータを与えなくてもよい。
    let _t    = SingleGen(A); // 上で定義した`A`を使用
    let _i32  = SingleGen(6); // `i32`を使用
    let _char = SingleGen('a'); // `char`を使用
}
