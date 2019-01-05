<!-- Closures in Rust, also called lambdas, are functions
that can capture the enclosing environment. Their syntax and capabilities make them
very convenient for on the fly usage. Some characteristics include: -->
Rustのクロージャはラムダ(`lambda`)とも呼ばれ、自身が定義された環境の外側の変数を参照できるような関数のことを指します。クロージャの文法と性質はサッと何かを仕上げたいときに非常に有用です。以下がクロージャの性質の一部です。


<!-- * uses `||` instead of `()` around input variables.
* *both* input and return *types* can be inferred.
* input variable *names* must be specified.
* body delimination (`{}`) is optional for a single expression. Mandatory
otherwise.
* the outer environment variables *may* be captured.
* calling a closure is exactly like a function: `call(var)`. -->
* 引数の周りを、`()`の代わりに`||`で囲む。
* 引数と返り値の*型*の*両方*を推測してくれる。
* 引数の*名前*は明示しなくてはならない。
* 関数が１文で終わるときは`{}`で関数のbodyを囲む必要はない。複数文の場合は必須。
* 外側の環境の変数を参照することができる。
* クロージャの呼び出しは関数と全く一緒である。例: `call(var)`

``` rust,editable
fn main() {
    // 関数とクロージャのそれぞれで数値をインクリメントする
    fn  function            (i: i32) -> i32 { i + 1 }

    // 型アノテーションは、通常の関数と同様の方法で行えるが、必須ではない。
    // `{}`も必須ではない。
    // クロージャは一種の無名関数なので、適切な変数にバインディングしてやるとよい
    let closure_annotated = |i: i32| -> i32 { i + 1 };
    let closure_inferred  = |i     |          i + 1  ;

    let i = 1;

    // 関数とクロージャを呼び出す。
    println!("function: {}", function(i));
    println!("annotated closure: {}", closure_annotated(i));
    println!("inferred closure: {}", closure_inferred(i));

    // 返り値の型を推測した結果、`i32`型を返すクロージャ。
    // 引数はとらない。
    let one = || 1;
    println!("closure returning one: {}", one());

    // クロージャは自身の周囲にある環境の変数を参照することができる。
    // 通常の関数では不可能
    let professor_x = "Charles Xavier";

    // 引数をとらず、返り値もないクロージャ。周りの環境にある変数を用いている。
    let print = || println!("Professor X's name is: {}", professor_x);

    // クロージャを実行
    print();
}

```
