<!--
# Visibility
-->
# プライベートとパブリック

<!--
By default, the items in a module have private visibility, but this can be
overridden with the `pub` modifier. Only the public items of a module can be
accessed from outside the module scope.
-->
デフォルトでは、モジュール内の要素はプライベートですが、これは`pub`で修飾することでパブリックな属性にすることができます。パブリックな属性のみがモジュールの外のスコープからアクセスすることができるようになります。

```rust,editable
// A module named `my_mod`
// `my_mod`という名称のモジュール
mod my_mod {
    // Items in modules default to private visibility.
    // モジュール内の要素はデフォルトでプライベート
    fn private_function() {
        println!("called `my_mod::private_function()`");
    }

    // Use the `pub` modifier to override default visibility.
    // `pub`を用いてパブリックに変更
    pub fn function() {
        println!("called `my_mod::function()`");
    }

    // Items can access other items in the same module,
    // even when private.
    // モジュール内からならば、プライベートな属性にアクセスすることに支障はない。
    pub fn indirect_access() {
        print!("called `my_mod::indirect_access()`, that\n> ");
        private_function();
    }

    // Modules can also be nested
    // モジュールもネストできる
    pub mod nested {
        pub fn function() {
            println!("called `my_mod::nested::function()`");
        }

        #[allow(dead_code)]
        fn private_function() {
            println!("called `my_mod::nested::private_function()`");
        }

        // Functions declared using `pub(in path)` syntax are only visible
        // within the given path. `path` must be a parent or ancestor module
        // `pub(in path)`形式で宣言された関数は該当のパス内でのみアクセスできる。
        // `path`は親や先祖のモジュールでなくてはならない。
        pub(in crate::my_mod) fn public_function_in_my_mod() {
            print!("called `my_mod::nested::public_function_in_my_mod()`, that\n> ");
            public_function_in_nested();
        }

        // Functions declared using `pub(self)` syntax are only visible within
        // the current module, which is the same as leaving them private
        // `pub(self)`形式で宣言された関数は現在のモジュール内でのみアクセスできる。
        // つまり、プライベートにするのと同じである。
        pub(self) fn public_function_in_nested() {
            println!("called `my_mod::nested::public_function_in_nested()`");
        }

        // Functions declared using `pub(super)` syntax are only visible within
        // the parent module
        // `pub(super)`形式で宣言された関数は親モジュール内でのみアクセスできる。
        pub(super) fn public_function_in_super_mod() {
            println!("called `my_mod::nested::public_function_in_super_mod()`");
        }
    }

    pub fn call_public_function_in_my_mod() {
        print!("called `my_mod::call_public_function_in_my_mod()`, that\n> ");
        nested::public_function_in_my_mod();
        print!("> ");
        nested::public_function_in_super_mod();
    }

    // pub(crate) makes functions visible only within the current crate
    // pub(crate)により関数は現在のクレート内でのみアクセスできる。
    pub(crate) fn public_function_in_crate() {
        println!("called `my_mod::public_function_in_crate()`");
    }

    // Nested modules follow the same rules for visibility
    // ネストしたモジュールも、同様の性質を示す。
    mod private_nested {
        #[allow(dead_code)]
        pub fn function() {
            println!("called `my_mod::private_nested::function()`");
        }

        // Private parent items will still restrict the visibility of a child item,
        // even if it is declared as visible within a bigger scope.
        // 親がプライベートな場合、子要素がより大きなスコープでアクセスできるように宣言されていても、
        // 子要素にアクセス可能な範囲は制限されます。
        #[allow(dead_code)]
        pub(crate) fn restricted_function() {
            println!("called `my_mod::private_nested::restricted_function()`");
        }
    }
}

fn function() {
    println!("called `function()`");
}

fn main() {
    // Modules allow disambiguation between items that have the same name.
    // モジュールによって、同名の関数を区別することができる。
    function();
    my_mod::function();

    // Public items, including those inside nested modules, can be
    // accessed from outside the parent module.
    // パブリックな要素ならば、たとえネストしたものでも、
    // モジュールの外からアクセスすることができる。
    my_mod::indirect_access();
    my_mod::nested::function();
    my_mod::call_public_function_in_my_mod();

    // pub(crate) items can be called from anywhere in the same crate
    // pub(crate)の要素は同じクレートのどこからでも呼び出すことができる。
    my_mod::public_function_in_crate();

    // pub(in path) items can only be called from within the module specified
    // Error! function `public_function_in_my_mod` is private
    // pub(in path)の要素は指定されたモジュールからのみ呼び出すことができる。
    // エラー! `public_function_in_my_mod`関数はプライベート。
    //my_mod::nested::public_function_in_my_mod();
    // TODO ^ Try uncommenting this line
    // TODO ^ 試しにこの行をアンコメントしてみましょう。

    // Private items of a module cannot be directly accessed, even if
    // nested in a public module:
    // プライベートな要素は、たとえパブリックなモジュール内に存在していても
    // 直接アクセスすることはできない。

    // Error! `private_function` is private
    // エラー!`private_function`はプライベート。
    //my_mod::private_function();
    // TODO ^ Try uncommenting this line
    // TODO ^ 試しにこの行をアンコメントしてみましょう。

    // Error! `private_function` is private
    // エラー！`private_function`はプライベート。
    //my_mod::nested::private_function();
    // TODO ^ Try uncommenting this line
    // TODO ^ 試しにこの行をアンコメントしてみましょう。

    // Error! `private_nested` is a private module
    // エラー！`private_nested`はプライベートなモジュール。
    //my_mod::private_nested::function();
    // TODO ^ Try uncommenting this line
    // TODO ^ 試しにこの行をアンコメントしてみましょう。

    // Error! `private_nested` is a private module
    // エラー! `private_nested`はプライベートなモジュール。
    //my_mod::private_nested::restricted_function();
    // TODO ^ Try uncommenting this line
    // TODO ^ 試しにこの行をアンコメントしてみましょう。
}
```
