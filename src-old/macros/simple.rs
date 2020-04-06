// `say_hello`という名のシンプルなマクロ
macro_rules! say_hello {
    // `()`はマクロが引数をとらないことを示す。
    () => (
        // マクロは(訳注: プリコンパイルの段階で)このブロック内の内容に展開されます。
        println!("Hello!");
    )
}

fn main() {
    // この呼び出しは`println!("Hello");`に置き換えられます。
    say_hello!()
}
