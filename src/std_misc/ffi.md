<!-- Rust provides a Foreign Function Interface (FFI) to C libraries. Foreign
functions must be declared inside an `extern` block annotated with a `#[link]`
attribute containing the name of the foreign library. -->
RustはCのライブラリを呼び出すために他言語関数インターフェイス(Foreign Function Interface, FFI)を持っています。他言語の関数を使用する際には、そのライブラリ名を`#[link]`アトリビュートに渡し、更にそれでアノテーションされた`extern`ブロック内で宣言する必要があります。

``` rust
use std::fmt;

// このexternブロックはlibmライブラリをリンクする。
#[link(name = "m")]
extern {
    // 他言語の関数宣言。
    // この関数は単精度浮動小数の複素数型の平方根を計算するためのもの
    fn csqrtf(z: Complex) -> Complex;
}

fn main() {
    // z = -1 + 0i
    let z = Complex { re: -1., im: 0. };

    // 安全でない方法で他言語関数を呼び出す。
    let z_sqrt = unsafe {
        csqrtf(z)
    };

    println!("the square root of {:?} is {:?}", z, z_sqrt);
}

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

``` bash
$ rustc ffi.rs && ./ffi
the square root of -1+0i is 0+1i
```



<!-- Since calling foreign functions is considered unsafe, it's common to write safe
wrappers around them. -->
他言語関数呼び出しは安全でない(unsafe)ので、安全にするためのラッパーを書くことが一般的です。

``` rust
use std::fmt;

#[link(name = "m")]
extern {
    fn ccosf(z: Complex) -> Complex;
}

// 型安全ににするためのラッパ
fn cos(z: Complex) -> Complex {
    unsafe { ccosf(z) }
}

fn main() {
    // z = 0 + 1i
    let z = Complex { re: 0., im: 1. };

    println!("cos({:?}) = {:?}", z, cos(z));
}

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

``` bash
$ rustc safe.rs && ./safe
cos(0+1i) = 1.5430806+0i
```
