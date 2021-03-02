<!--
# Structs
-->
# 構造体

<!--
Annotation of lifetimes in structures are also similar to functions:
-->
構造体におけるライフタイムも関数のそれと似ている。

```rust,editable
// A type `Borrowed` which houses a reference to an
// `i32`. The reference to `i32` must outlive `Borrowed`.
// `i32`への参照をメンバに持つ`Borrowed`型。
// 参照は`Borrowed`自体よりも長生きでなくてはならない。
#[derive(Debug)]
struct Borrowed<'a>(&'a i32);

// Similarly, both references here must outlive this structure.
// 同様に、ここでも参照は構造体よりも長生きでなくてはならない。
#[derive(Debug)]
struct NamedBorrowed<'a> {
    x: &'a i32,
    y: &'a i32,
}

// An enum which is either an `i32` or a reference to one.
// `i32`、あるいは`i32`への参照のいずれかとなる列挙型
#[derive(Debug)]
enum Either<'a> {
    Num(i32),
    Ref(&'a i32),
}

fn main() {
    let x = 18;
    let y = 15;

    let single = Borrowed(&x);
    let double = NamedBorrowed { x: &x, y: &y };
    let reference = Either::Ref(&x);
    let number    = Either::Num(y);

    println!("x is borrowed in {:?}", single);
    println!("x and y are borrowed in {:?}", double);
    println!("x is borrowed in {:?}", reference);
    println!("y is *not* borrowed in {:?}", number);
}
```

<!--
### See also:
-->
### 参照

[`struct`s][structs]


[structs]: ../../custom_types/structs.md
