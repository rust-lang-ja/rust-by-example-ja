<!--
# Foreign Function Interface
-->
# 他言語関数インターフェイス

<!--
Rust provides a Foreign Function Interface (FFI) to C libraries. Foreign
functions must be declared inside an `extern` block annotated with a `#[link]`
attribute containing the name of the foreign library.
-->
RustはCのライブラリを呼び出すために他言語関数インターフェイス(Foreign Function Interface, FFI)を持っています。他言語の関数を使用する際には、そのライブラリ名を`#[link]`アトリビュートに渡し、更にそれでアノテーションされた`extern`ブロック内で宣言する必要があります。

```rust,ignore
use std::fmt;

// this extern block links to the libm library
// このexternブロックはlibmライブラリをリンクする。
#[link(name = "m")]
extern {
    // this is a foreign function
    // that computes the square root of a single precision complex number
    // 他言語の関数宣言。
    // この関数は単精度浮動小数の複素数型の平方根を計算するためのもの
    fn csqrtf(z: Complex) -> Complex;

    fn ccosf(z: Complex) -> Complex;
}

// Since calling foreign functions is considered unsafe,
// it's common to write safe wrappers around them.
// 型安全にするためのラッパ
fn cos(z: Complex) -> Complex {
    unsafe { ccosf(z) }
}

fn main() {
    // z = -1 + 0i
    let z = Complex { re: -1., im: 0. };

    // calling a foreign function is an unsafe operation
    let z_sqrt = unsafe { csqrtf(z) };

    println!("the square root of {:?} is {:?}", z, z_sqrt);

    // calling safe API wrapped around unsafe operation
    println!("cos({:?}) = {:?}", z, cos(z));
}

// Minimal implementation of single precision complex numbers
// 単精度浮動小数の複素数型の最小限の実装
#[repr(C)]
#[derive(Clone, Copy)]
struct Complex {
    re: f32,
    im: f32,
}

impl fmt::Debug for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.im < 0. {
            write!(f, "{}-{}i", self.re, -self.im)
        } else {
            write!(f, "{}+{}i", self.re, self.im)
        }
    }
}
```
