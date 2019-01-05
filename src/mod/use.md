<!-- The `use` declaration can be used to bind a full path to a new name, for easier
access. -->
`use`宣言をすることで、要素の絶対パスを新しい名前にバインドすることができ、より簡潔な記述が可能になります。

``` rust,editable
// `deeply::nested::function`を`other_function`にバインド
use deeply::nested::function as other_function;

fn function() {
    println!("called `function()`");
}

mod deeply {
    pub mod nested {
        pub fn function() {
            println!("called `deeply::nested::function()`")
        }
    }
}

fn main() {
    // `deeply::nested::function`へ、より簡潔にアクセス
    other_function();

    println!("Entering block");
    {
        // これは`use deeply::nested::function as function`と同等
        // この`function()`は外の`function()`をシャドウイングする
        use deeply::nested::function;
        function();

        // `use`バインディングは局所的なスコープを持つ。
        // この場合には`function()`のシャドウイングはこのブロック内のみ
        println!("Leaving block");
    }

    function();
}

```
