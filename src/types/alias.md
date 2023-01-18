<!--
# Aliasing
-->
# エイリアス

<!--
The `type` statement can be used to give a new name to an existing type. Types
must have `UpperCamelCase` names, or the compiler will raise a warning. The
exception to this rule are the primitive types: `usize`, `f32`, etc.
-->
`type`文を使用することで既存の型に新しい名前(`alias`)を付けることができます。その場合、名前は`UpperCamelCase`でなくてはなりません。さもなくばコンパイラがエラーを出します。唯一の例外は`usize`や`f32`のようなプリミティブ型です。

```rust,editable
// `NanoSecond`, `Inch`, and `U64` are new names for `u64`.
// `NanoSecond` `Inch` `U64` を `u64`の別名として使用する。
type NanoSecond = u64;
type Inch = u64;
type U64 = u64;

// Use an attribute to silence warning.
// 警告を抑えるアトリビュートを使用。
#[allow(non_camel_case_types)]
type u64_t = u64;
// TODO ^ Try removing the attribute
// TODO ^ アトリビュートを使用しない場合、どうなるか見てみましょう。

fn main() {
    // `NanoSecond` = `Inch` = `U64` = `u64`.
    let nanoseconds: NanoSecond = 5 as U64;
    let inches: Inch = 2 as U64;

    // Note that type aliases *don't* provide any extra type safety, because
    // aliases are *not* new types
    // 型のエイリアスは、元の型をより型安全にしてくれる **わけではない** ことに注意しましょう。
    // なぜならば、エイリアスは新たな型を定義している **わけではない** からです。
    println!("{} nanoseconds + {} inches = {} unit?",
             nanoseconds,
             inches,
             nanoseconds + inches);
}
```

<!--
The main use of aliases is to reduce boilerplate; for example the `io::Result<T>` type
is an alias for the `Result<T, io::Error>` type.
-->
このようにエイリアスを付ける一番の理由はボイラープレートを減らすことです。例えば`io::Result<T>`型は`Result<T, io::Error>`の別名です。

<!--
### See also:
-->
### 参照

<!--
[Attributes](../attribute.md)
-->
[アトリビュート](../attribute.md)
