fn function() {
    println!("called `function()`");
}

mod cool {
    pub fn function() {
        println!("called `cool::function()`");
    }
}

mod my {
    fn function() {
        println!("called `my::function()`");
    }

    mod cool {
        pub fn function() {
            println!("called `my::cool::function()`");
        }
    }

    pub fn indirect_call() {
        // `function`という名の様々な関数をこのスコープ内から参照してみましょう。
        print!("called `my::indirect_call()`, that\n> ");

        // `self`キーワードは現在のモジュールスコープを示す。この場合は`my`。
        // `self::function()`と`funcition()`は同じ関数であるため、同じ結果になる。
        self::function();
        function();

        // `my`以下の別のモジュールを呼び出す際に`self`を用いて明示的に参照できる。
        self::cool::function();

        // `super`は親スコープ(`my`の外側)を参照する。
        super::function();

        // 以下は*クレイト*スコープ内の`cool::function`をバインディングする。
        // この場合、クレイトスコープは一番外側のスコープである。
        {
            use cool::function as root_function;
            root_function();
        }
    }
}

fn main() {
    my::indirect_call();
}
