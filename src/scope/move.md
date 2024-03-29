<!--
# Ownership and moves
-->
# 所有権とムーブ

<!--
Because variables are in charge of freeing their own resources, 
**resources can only have one owner**. This also prevents resources 
from being freed more than once. Note that not all variables own 
resources (e.g. [references]).
-->
変数には自身の保持する資源を開放する責任があるため、 **資源は一度に一つの所有者** しか持つことができません。これはまた、資源を2度以上開放することができないということでもあります。ここで、全ての変数が資源を所有するわけではないということに注意しましょう。（e.g. [参照][references]）

<!--
When doing assignments (`let x = y`) or passing function arguments by value
(`foo(x)`), the *ownership* of the resources is transferred. In Rust-speak, 
this is known as a *move*.
-->
変数をアサインする(`let x = y`)際や、関数に引数を値渡しする(`foo(x)`)際は、資源の *所有権(`ownership`)* が移動します。Rustっぽく言うと、「 *ムーブ* 」です。

<!--
After moving resources, the previous owner can no longer be used. This avoids
creating dangling pointers.
-->
資源を移動すると、それまでの所有者（訳注: 変数などのこと）を使用することはできなくなります。これによりダングリングポインタの発生を防げます。

```rust,editable
// This function takes ownership of the heap allocated memory
// この関数はヒープメモリ上の資源の所有権を取る。
fn destroy_box(c: Box<i32>) {
    println!("Destroying a box that contains {}", c);

    // `c` is destroyed and the memory freed
    // `c`は破棄されメモリは開放される。
}

fn main() {
    // _Stack_ allocated integer
    // _スタック_上に置かれた整数
    let x = 5u32;

    // *Copy* `x` into `y` - no resources are moved
    // `x`を`y`に *コピー* する。元の値が移動するわけではない。
    let y = x;

    // Both values can be independently used
    // 両方の値はそれぞれ独立に使うことができる。
    println!("x is {}, and y is {}", x, y);

    // `a` is a pointer to a _heap_ allocated integer
    // `a`は_ヒープ_上の整数へのポインタ
    let a = Box::new(5i32);

    println!("a contains: {}", a);

    // *Move* `a` into `b`
    // `a`を`b`に *ムーブ* する。
    let b = a;
    // The pointer address of `a` is copied (not the data) into `b`.
    // Both are now pointers to the same heap allocated data, but
    // `b` now owns it.
    // すなわち、`a`の指すメモリ上の番地が`b`にコピーされるため
    // いずれもヒープ上の同じ値を指すポインタとなる。しかし所有権は`b`にある。
    
    // Error! `a` can no longer access the data, because it no longer owns the
    // heap memory
    // エラー! `a`は所有権を持たないため、ヒープ上のデータにアクセスできない。
    //println!("a contains: {}", a);
    // TODO ^ Try uncommenting this line
    // TODO ^ 試しにここをアンコメントしてみましょう。

    // This function takes ownership of the heap allocated memory from `b`
    // この関数はヒープメモリ上の所有権を`b`から取る。
    destroy_box(b);

    // Since the heap memory has been freed at this point, this action would
    // result in dereferencing freed memory, but it's forbidden by the compiler
    // Error! Same reason as the previous Error
    // この時点でヒープメモリ上の資源は開放されているので、次の操作は
    // 解放済みメモリをデリファレンスすることになる。しかしそれはコンパイラが許さない。
    // エラー! 上述の理由より
    //println!("b contains: {}", b);
    // TODO ^ Try uncommenting this line
    // TODO ^ 試しにここをアンコメントしてみましょう。
}
```

[references]: ../flow_control/match/destructuring/destructure_pointers.md
