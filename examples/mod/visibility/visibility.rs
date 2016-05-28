// `my`という名称のモジュール
mod my {
    // モジュール内の要素はデフォルトでプライベート
    fn private_function() {
        println!("called `my::private_function()`");
    }

    // `pub`を用いてパブリックに変更
    pub fn function() {
        println!("called `my::function()`");
    }

    // モジュール内からならば、プライベートな属性にアクセスすることに支障はない。
    pub fn indirect_access() {
        print!("called `my::indirect_access()`, that\n> ");
        private_function();
    }

    // モジュールはネストすることができる。
    pub mod nested {
        pub fn function() {
            println!("called `my::nested::function()`");
        }

        #[allow(dead_code)]
        fn private_function() {
            println!("called `my::nested::private_function()`");
        }
    }

    // ネストしたモジュールも、同様の性質を示す。
    mod private_nested {
        #[allow(dead_code)]
        pub fn function() {
            println!("called `my::private_nested::function()`");
        }
    }
}

fn function() {
    println!("called `function()`");
}

fn main() {
    // モジュールによって、同名の関数を区別することができる。
    function();
    my::function();

    // パブリックな要素ならば、たとえネストしたものでも、
    // モジュールの外からアクセスすることができる。
    my::indirect_access();
    my::nested::function();

    // プライベートな要素は、たとえパブリックなモジュール内に存在していても
    // 直接アクセスすることはできない。

    // エラー!`private_function`はプライベート。
    //my::private_function();
    // TODO ^ 試しにこの行をアンコメントしてみましょう。

    // エラー！`private_function`はプライベート。
    //my::nested::private_function();
    // TODO ^ 試しにこの行をアンコメントしてみましょう。

    // エラー！`private_nested`はプライベートなモジュール 。
    //my::private_nested::function();
    // TODO ^ 試しにこの行をアンコメントしてみましょう。

}
