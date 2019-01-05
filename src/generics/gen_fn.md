<!-- The same set of rules can be applied to functions: a type `T` becomes
generic when preceded by `<T>`. -->
「型`T`はその前に`<T>`があるとジェネリック型になる」というルールは関数に対しても当てはまります。

<!-- Using generic functions sometimes requires explicitly specifying type
parameters. This may be if the function is called where the return type
is generic, or if the compiler doesn't have enough information to infer
the necessary type parameters. -->
ジェネリック関数を使用する際、以下の様な場合には型パラメータを明示する必要があります。

* 返り値がジェネリック型である場合。
* コンパイラが型パラメータを推論するのに十分な情報がない場合

<!-- A function call with explicitly specified type parameters looks like:
`fun::<A, B, ...>()`. -->
型パラメータを明示したうえでの関数呼び出しの構文は`fun::<A, B, ...>()`のようになります。

``` rust,editable
struct A;          // 具象型`A`.
struct S(A);       // 具象型`S`.
struct SGen<T>(T); // ジェネリック型`SGen`.

// 以下の関数は全て変数の所有権をとった後すぐにスコープを抜けて
// 変数をメモリ上から開放する。

// `S`という型の引数`_s`をとる`reg_fn`という関数を定義
// `<T>`がないのでジェネリック関数ではない
fn reg_fn(_s: S) {}

// `gen_spec_t`という関数を定義。これは`A`という型を与えられた`Sgen<T>`
// という型の引数`_s`を取る。関数名の直後に`<A>`という型パラメータでAが
// ジェネリックであることを明示していないので、この関数はAをジェネリック型
// としては取らない
fn gen_spec_t(_s: SGen<A>) {}

// `gen_spec_i32`という関数を定義。
// これは明示的な型パラメータとして`i32`を与えられた`Sgen<i32>`型の引数`_s`をとる
// この関数もジェネリックではない
fn gen_spec_i32(_s: SGen<i32>) {}

// `generic`という関数を定義。`SGen<T>`という型の引数`_s`を取る。`<T>`が`SGen<T>`に
// 先行しているため、これはTに対してジェネリックな関数
fn generic<T>(_s: SGen<T>) {}

fn main() {
    // ジェネリックでない関数を使用する
    reg_fn(S(A));          // 具象型
    gen_spec_t(SGen(A));   // 型パラメータ`A`を暗黙のうちに受け取る
    gen_spec_i32(SGen(6)); // 型パラメータ`i32`を暗黙のうちに受け取る

    // 型パラメータ`char`を明示的に`generic()`に渡す
    generic::<char>(SGen('a'));

    // 型パラメータ`char`を暗黙的に`generic()`に渡す
    generic(SGen('c'));
}


```

### See also:

[関数][fn] and [構造体][structs]

[fn]:../fn.html
[structs]: ../custom_types/structs.html
