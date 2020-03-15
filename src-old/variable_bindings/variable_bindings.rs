fn main() {
    let an_integer = 1u32;
    let a_boolean = true;
    let unit = ();

    // `an_integer`を`copied_integer`へとコピー
    let copied_integer = an_integer;

    println!("An integer: {:?}", copied_integer);
    println!("A boolean: {:?}", a_boolean);
    println!("Meet the unit value: {:?}", unit);

    // 使用されていない変数があると、コンパイラは警告を出します。
    // 変数名の頭に`_`(アンダーバー)を付けると警告を消すことができます。
    let _unused_variable = 3u32;

    let noisy_unused_variable = 2u32;
    // FIXME ^ 頭にアンダーバーを付けて、警告を抑えましょう。
}
