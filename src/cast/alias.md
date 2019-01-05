<!-- The `type` statement can be used to give a new name to an existing type. Types
must have `CamelCase` names, or the compiler will raise a warning. The
exception to this rule are the primitive types: `usize`, `f32`, etc. -->
`type`文を使用することで既存の型に新しい名前(`alias`)を付けることができます。その場合、名前は`CamelCase`でなくてはなりません。さもなくばコンパイラがエラーを出します。唯一の例外は`usize`や`f32`のようなプリミティブ型です。

``` rust,editable
// `NanoSecond` を `u64`の別名として使用する。
type NanoSecond = u64;
type Inch = u64;

// 警告を抑えるアトリビュートを使用。
#[allow(non_camel_case_types)]
type u64_t = u64;
// TODO ^ アトリビュートを使用しない場合、どうなるか見てみましょう。
// こちらも警告を抑えるアトリビュート
#[allow(trivial_numeric_casts)]
fn main() {
    // `NanoSecond` = `Inch` = `u64_t` = `u64`.
    let nanoseconds: NanoSecond = 5 as u64_t;
    let inches: Inch = 2 as u64_t;
    // 型のエイリアスは、元の型をより型安全にしてくれる **わけではない** ことに注意しましょう。
    // なぜならば、エイリアスは新たな型を定義している **わけではない** からです。
    println!("{} nanoseconds + {} inches = {} unit?",
             nanoseconds,
             inches,
             nanoseconds + inches);
}

```

<!-- The main use of aliases is to reduce typing; for example the `IoResult<T>` type
is an alias for the `Result<T, IoError>` type. -->
このようにエイリアスを付ける一番の理由はタイプ量を減らすことです。例えば`IoResult<T>`型は`Result<T, IoError>`の別名です。


### See also:

[アトリビュート](../attribute.html)
