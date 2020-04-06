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
