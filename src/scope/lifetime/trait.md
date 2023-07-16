<!--
# Traits
-->
# トレイト

<!--
Annotation of lifetimes in trait methods basically are similar to functions.
Note that `impl` may have annotation of lifetimes too.
-->
トレイトのメソッドにおけるライフタイムのアノテーションは、
基本的には関数に似ています。
`impl`にもライフタイムのアノテーションがあることに注意してください。

```rust,editable
// A struct with annotation of lifetimes.
// ライフタイムのアノテーションつき構造体。
#[derive(Debug)]
struct Borrowed<'a> {
    x: &'a i32,
}

// Annotate lifetimes to impl.
// ライフタイムのアノテーションつきimpl。
impl<'a> Default for Borrowed<'a> {
    fn default() -> Self {
        Self {
            x: &10,
        }
    }
}

fn main() {
    let b: Borrowed = Default::default();
    println!("b is {:?}", b);
}
```

<!--
### See also:
-->
### 参照

[トレイト][trait]


[trait]: ../../trait.md
