<!--
# Functions
-->
# 関数

<!--
The same set of rules can be applied to functions: a type `T` becomes
generic when preceded by `<T>`.
-->
「型`T`はその前に`<T>`があるとジェネリック型になる」というルールは関数に対しても当てはまります。

<!--
Using generic functions sometimes requires explicitly specifying type 
parameters. This may be the case if the function is called where the return type 
is generic, or if the compiler doesn't have enough information to infer 
the necessary type parameters.
-->
ジェネリック関数を使用する際、以下の様な場合には型パラメータを明示する必要があります。

* 返り値がジェネリック型である場合。
* コンパイラが型パラメータを推論するのに十分な情報がない場合

<!--
A function call with explicitly specified type parameters looks like:
`fun::<A, B, ...>()`.
-->
型パラメータを明示したうえでの関数呼び出しの構文は`fun::<A, B, ...>()`のようになります。

```rust,editable
struct A;          // Concrete type `A`.
                   // 具象型`A`.
struct S(A);       // Concrete type `S`.
                   // 具象型`S`.
struct SGen<T>(T); // Generic type `SGen`.
                   // ジェネリック型`SGen`.

// The following functions all take ownership of the variable passed into
// them and immediately go out of scope, freeing the variable.
// 以下の関数は全て変数の所有権をとった後すぐにスコープを抜けて
// 変数をメモリ上から開放する。

// Define a function `reg_fn` that takes an argument `_s` of type `S`.
// This has no `<T>` so this is not a generic function.
// `S`という型の引数`_s`をとる`reg_fn`という関数を定義
// `<T>`がないのでジェネリック関数ではない
fn reg_fn(_s: S) {}

// Define a function `gen_spec_t` that takes an argument `_s` of type `SGen<T>`.
// It has been explicitly given the type parameter `A`, but because `A` has not 
// been specified as a generic type parameter for `gen_spec_t`, it is not generic.
// `gen_spec_t`という関数を定義。これは`A`という型を与えられた`SGen<T>`
// という型の引数`_s`を取る。関数名の直後に`<A>`という型パラメータでAが
// ジェネリックであることを明示していないので、この関数はAをジェネリック型
// としては取らない
fn gen_spec_t(_s: SGen<A>) {}

// Define a function `gen_spec_i32` that takes an argument `_s` of type `SGen<i32>`.
// It has been explicitly given the type parameter `i32`, which is a specific type.
// Because `i32` is not a generic type, this function is also not generic.
// `gen_spec_i32`という関数を定義。
// これは明示的な型パラメータとして`i32`を与えられた`SGen<i32>`型の引数`_s`をとる
// この関数もジェネリックではない
fn gen_spec_i32(_s: SGen<i32>) {}

// Define a function `generic` that takes an argument `_s` of type `SGen<T>`.
// Because `SGen<T>` is preceded by `<T>`, this function is generic over `T`.
// `generic`という関数を定義。`SGen<T>`という型の引数`_s`を取る。`<T>`が`SGen<T>`に
// 先行しているため、これはTに対してジェネリックな関数
fn generic<T>(_s: SGen<T>) {}

fn main() {
    // Using the non-generic functions
    // ジェネリックでない関数を使用する
    reg_fn(S(A));          // Concrete type.
                           // 具象型
    gen_spec_t(SGen(A));   // Implicitly specified type parameter `A`.
                           // 型パラメータ`A`を暗黙のうちに受け取る
    gen_spec_i32(SGen(6)); // Implicitly specified type parameter `i32`.
                           // 型パラメータ`i32`を暗黙のうちに受け取る

    // Explicitly specified type parameter `char` to `generic()`.
    generic::<char>(SGen('a'));

    // Implicitly specified type parameter `char` to `generic()`.
    // 型パラメータ`char`を暗黙的に`generic()`に渡す
    generic(SGen('c'));
}
```

<!--
### See also:
-->
### 参照

<!--
[functions][fn] and [`struct`s][structs]
-->
[関数][fn], [構造体][structs]

[fn]: ../fn.md
[structs]: ../custom_types/structs.md
