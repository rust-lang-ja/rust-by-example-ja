fn main() {
    // `i32`型へのリファレンスをアサインする。
    // `&`によってリファレンスであることを明示している。
    let reference = &4;

    match reference {
        // 上で定義した`reference`という変数が`&val`とのパターンマッチ
        // に用いられた場合、以下の2つの値が比較されていることになる。
        // `&i32`
        // `&val`
        // ^ よって`&`を落とせば、`i32`が`val`にアサインされることがわかる。
        &val => println!("Got a value via destructuring: {:?}", val),
    }

    // `&`を使用したくない場合は、マッチングの前にデリファレンスする。
    match *reference {
        val => println!("Got a value via dereferencing: {:?}", val),
    }

    // いきなりリファレンスを変数に代入するのではない場合はどうでしょう。
    // 先ほどは右辺値が`&`で始まっていたのでリファレンスでしたが、
    // これは違います。
    let _not_a_reference = 3;

    // このような場合、Rustでは変数束縛時に`ref`を宣言します。
    // 要素のリファレンスが作られて、それが束縛対象になります。
    let ref _is_a_reference = 3;

    // 同様にミュータブルな値の場合`ref mut`を使用することでリファレンスを
    // 取得できます。イミュータブルの場合と合わせてみていきましょう。
    let value = 5;
    let mut mut_value = 6;

    // `ref`を使用してリファレンスを作成。
    match value {
        ref r => println!("Got a reference to a value: {:?}", r),
    }

    // 同様に`ref mut`を使用。
    match mut_value {
        ref mut m => {
            // リファレンスを取得、値を変更するためにはデリファレンスする必要がある。
            *m += 10;
            println!("We added 10. `mut_value`: {:?}", m);
        },
    }
}
