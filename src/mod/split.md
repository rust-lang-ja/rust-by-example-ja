<!-- Modules can be mapped to a file/directory hierarchy. Let's break down the
[visibility example][visibility] in files: -->
モジュールはファイル・ディレクトリ間の階層構造と対応関係にあります。モジュールに[お互いがどのように見えているか][visibility]、以下の様なファイルを例に詳しく見ていきましょう。

``` bash
$ tree .
.
|-- my
|   |-- inaccessible.rs
|   |-- mod.rs
|   `-- nested.rs
`-- split.rs
```

``` rust,ignore
// このように宣言すると、`my.rs`または、`my/mod.rs`という名のファイルを探し、
// その内容をこのファイル中で`my`という名から使用することができるようにします。
mod my;

fn function() {
    println!("called `function()`");
}

fn main() {
    my::function();

    function();

    my::indirect_access();

    my::nested::function();
}

```

``` rust
// 同様に`mod inaccessible`、`mod nested`によって、`nested.rs`、`inaccessible.rs`の内容をこの中で使用することができるようになる。
// 訳注: `pub`をつけないかぎり、この中でしか使用できない。
mod inaccessible;
pub mod nested;

pub fn function() {
    println!("called `my::function()`");
}

fn private_function() {
    println!("called `my::private_function()`");
}

pub fn indirect_access() {
    print!("called `my::indirect_access()`, that\n> ");

    private_function();
}

```

``` rust
pub fn function() {
    println!("called `my::nested::function()`");
}

#[allow(dead_code)]
fn private_function() {
    println!("called `my::nested::private_function()`");
}

```

``` rust
#[allow(dead_code)]
pub fn public_function() {
    println!("called `my::inaccessible::public_function()`");
}

// 訳注: この関数は`my/mod.rs`中で`pub mod`されていないため、
// `split.rs`からは呼び出すことができない。

```

<!-- Let's check that things still work as before: -->
では、以前と同じように実行できるか確認しましょう。

``` bash
$ rustc split.rs && ./split
called `my::function()`
called `function()`
called `my::indirect_access()`, that
> called `my::private_function()`
called `my::nested::function()`
```

[visibility]: ../mod/visibility.html
