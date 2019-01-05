<!-- To link a crate to this new library, the `extern crate` declaration must be
used. This will not only link the library, but also import all its items under
a module named the same as the library. The visibility rules that apply to
modules also apply to libraries. -->
クレイトをこの新しいライブラリにリンクするには、`extern crate`宣言を使用する必要があります。これはライブラリをリンクするだけでなく、その要素を全てライブラリと同じ名前のモジュールにインポートします。モジュールにおけるパブリック・プライベートなどのスコープのルールは全て、ライブラリにおいても当てはまります。

``` rust,ignore
// `library`にリンクし、`rary`モジュール内の要素を全てインポートする。
extern crate rary;

fn main() {
    rary::public_function();

    // エラー！`private_function`はプライベート
    //rary::private_function();

    rary::indirect_access();
}

```

``` bash
# `-L .`を引数として与えることで、カレントディレクトリをライブラリのサーチパスに追加します。
$ rustc -L . executable.rs && ./executable
called rary's `public_function()`
called rary's `indirect_access()`, that
> called rary's `private_function()`
```
