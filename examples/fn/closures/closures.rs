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
