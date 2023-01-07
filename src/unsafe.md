<!--
# Unsafe Operations
-->
# 安全でない操作

<!--
As an introduction to this section, to borrow from [the official docs][unsafe],
"one should try to minimize the amount of unsafe code in a code base." With that
in mind, let's get started! Unsafe annotations in Rust are used to bypass
protections put in place by the compiler; specifically, there are four primary
things that unsafe is used for:
-->
この章の内容を見る前に、[公式ドキュメント](http://doc.rust-lang.org/book/unsafe.html)から引用した以下の文章をお読みください。

> コードベース中の、アンセーフな操作をするコードの量は、可能な限り小さく無くてはならない。

この戒めを頭に叩き込んだ上で、さあはじめましょう！
Rustにおいて、アンセーフなブロックはコンパイラのチェックをスルーするために使われます。具体的には以下の4つの主要なユースケースがあります。

<!--
* dereferencing raw pointers
* calling functions or methods which are `unsafe` (including calling a function
  over FFI, see [a previous chapter](std_misc/ffi.md) of the book) 
* accessing or modifying static mutable variables
* implementing unsafe traits
-->
* 生ポインタのデリファレンス
* 安全でない関数やメソッドの呼び出し(FFI経由の関数の呼び出しを含む (詳細は [本書のFFIに関する説明](std_misc/ffi.md) を参照ください))
* 静的なミュータブル変数へのアクセスや変更
* 安全でないトレイトの実装

<!--
### Raw Pointers
Raw pointers `*` and references `&T` function similarly, but references are
always safe because they are guaranteed to point to valid data due to the
borrow checker. Dereferencing a raw pointer can only be done through an unsafe
block.
-->
### 生ポインタ
生ポインタ`*`と参照`&T`はよく似た機能を持ちますが、後者は必ず有効なデータを指していることが借用チェッカーによって保証されているので、常に安全です。生ポインタのデリファレンスはアンセーフなブロックでしか実行できません。

```rust,editable
fn main() {
    let raw_p: *const u32 = &10;

    unsafe {
        assert!(*raw_p == 10);
    }
}
```

<!--
### Calling Unsafe Functions
Some functions can be declared as `unsafe`, meaning it is the programmer's
responsibility to ensure correctness instead of the compiler's. One example
of this is [`std::slice::from_raw_parts`] which will create a slice given a
pointer to the first element and a length.
-->
### 安全でない関数呼び出し
関数は `unsafe` として宣言できます。これはコンパイラの代わりにプログラマの責任で正しさを保証することを意味します。
例として [`std::slice::from_raw_parts`] があります。この関数は最初の要素へのポインタと長さを指定してスライスを作成します。

```rust,editable
use std::slice;

fn main() {
    let some_vector = vec![1, 2, 3, 4];

    let pointer = some_vector.as_ptr();
    let length = some_vector.len();

    unsafe {
        let my_slice: &[u32] = slice::from_raw_parts(pointer, length);

        assert_eq!(some_vector.as_slice(), my_slice);
    }
}
```

<!--
For `slice::from_raw_parts`, one of the assumptions which *must* be upheld is 
that the pointer passed in points to valid memory and that the memory pointed to
is of the correct type. If these invariants aren't upheld then the program's 
behaviour is undefined and there is no knowing what will happen.
-->
`slice::from_raw_parts` は、以下の仮定に基づいて処理します。
- 渡されたポインタが有効なメモリ位置を指していること
- そのメモリに格納された値が正しい型であること

この仮定を満たさない場合、プログラムの動作は不定となり、何が起こるかわかりません。

[unsafe]: https://doc.rust-lang.org/book/ch19-01-unsafe-rust.html
[`std::slice::from_raw_parts`]: https://doc.rust-lang.org/std/slice/fn.from_raw_parts.html
