<!--
# Dependencies
-->
依存関係

<!--
Most programs have dependencies on some libraries. If you have ever managed
dependencies by hand, you know how much of a pain this can be. Luckily, the Rust
ecosystem comes standard with `cargo`! `cargo` can manage dependencies for a
project.
-->
ほとんどのプログラムはライブラリに依存関係を持ちます。もし依存関係を手動で管理したことがあれば、それがどれだけ苦痛であるか分かるでしょう。幸運なことに、Rustのエコシステムには`cargo`が標準装備されています！`cargo`によってプロジェクトの依存関係を管理することができます。

<!--
To create a new Rust project,
-->
Rustのプロジェクトを新しく作るには下記のようにします。

```sh
# A binary
# バイナリ
cargo new foo

# A library
# ライブラリ
cargo new --lib bar
```

<!--
For the rest of this chapter, let's assume we are making a binary, rather than
a library, but all of the concepts are the same.
-->
この章の残りでは、ライブラリではなくバイナリを作ることを想定しますが、コンセプトはすべて同じです。

<!--
After the above commands, you should see a file hierarchy like this:
-->
上のコマンドを実行すると、次のようなファイル階層ができます。

```txt
.
├── bar
│   ├── Cargo.toml
│   └── src
│       └── lib.rs
└── foo
    ├── Cargo.toml
    └── src
        └── main.rs
```

<!--
The `main.rs` is the root source file for your new `foo` project -- nothing new there.
The `Cargo.toml` is the config file for `cargo` for this project. If you
look inside it, you should see something like this:
-->
`main.rs`がこの新規プロジェクト `foo` のルートのソースファイルです。なにも新しいことはありませんね。`Cargo.toml`はこのプロジェクトの`cargo`の設定ファイルです。中を見てみるとこのようになっています。

```toml
[package]
name = "foo"
version = "0.1.0"
authors = ["mark"]

[dependencies]
```

<!--
The `name` field under `[package]` determines the name of the project. This is
used by `crates.io` if you publish the crate (more later). It is also the name
of the output binary when you compile.
-->
`[package]`の下の`name`フィールドがプロジェクトの名前を決定します。これはクレートを公開するときに`crates.io`によって使われます（詳細は後述）。またコンパイルしたときの出力ファイルの名前でもあります。

<!--
The `version` field is a crate version number using [Semantic
Versioning](http://semver.org/).
-->
`version`フィールドはクレートのバージョン番号で、[セマンティックバージョニング](http://semver.org/)を使っています。

<!--
The `authors` field is a list of authors used when publishing the crate.
-->
`authors`フィールドは作者のリストで、クレートを公開するときに使われます。

<!--
The `[dependencies]` section lets you add dependencies for your project.
-->
`[dependencies]`セクションにはプロジェクトの依存関係を追加できます。

<!--
For example, suppose that we want our program to have a great CLI. You can find
lots of great packages on [crates.io](https://crates.io) (the official Rust
package registry). One popular choice is [clap](https://crates.io/crates/clap).
As of this writing, the most recent published version of `clap` is `2.27.1`. To
add a dependency to our program, we can simply add the following to our
`Cargo.toml` under `[dependencies]`: `clap = "2.27.1"`. And that's it! You can start using
`clap` in your program.
-->
例えば、プログラムに素晴らしいCLIが欲しいとします。[crates.io](https://crates.io)（Rustの公式パッケージレジストリ）には素晴らしいパッケージがたくさんあります。よくある選択肢の1つは[clap](https://crates.io/crates/clap)です。この記事を書いている時点での`clap`の最新の公開バージョンは`2.27.1`です。依存関係をプログラムに追加するには、`Cargo.toml`の`[dependencies]`の下に`clap = "2.27.1"`と単に書き加えます。これだけです！`clap`をプログラム内で使用できます。

<!--
`cargo` also supports [other types of dependencies][dependencies]. Here is just
a small sampling:
-->
`cargo`は[他の形式の依存関係][dependencies]もサポートしています。その一部を示します。

```toml
[package]
name = "foo"
version = "0.1.0"
authors = ["mark"]

[dependencies]
clap = "2.27.1" # from crates.io
                # crates.ioから
rand = { git = "https://github.com/rust-lang-nursery/rand" } # from online repo
                                                             # オンラインのレポジトリから
bar = { path = "../bar" } # from a path in the local filesystem
                          # ローカルのファイルシステムのパスから
```

<!--
`cargo` is more than a dependency manager. All of the available
configuration options are listed in the [format specification][manifest] of
`Cargo.toml`.
-->
`cargo`は依存管理ツール以上のこともできます。`Cargo.toml`の[format specification][manifest]に全ての設定オプションがリストアップされています。

<!--
To build our project we can execute `cargo build` anywhere in the project
directory (including subdirectories!). We can also do `cargo run` to build and
run. Notice that these commands will resolve all dependencies, download crates
if needed, and build everything, including your crate. (Note that it only
rebuilds what it has not already built, similar to `make`).
-->
プロジェクトをビルドするには、プロジェクトディレクトリのどこか（サブディレクトでも！）で`cargo build`を実行します。また`cargo run`でビルドと実行をできます。これらのコマンドは、全ての依存関係の解決、必要なクレートのダウンロード、自分のクレートを含む全てのビルドを行うことに注意してください。（`make`と同様、まだビルドしていないものだけをビルドします。）

<!--
Voila! That's all there is to it!
-->
Voila！これで完成です！


[manifest]: https://doc.rust-lang.org/cargo/reference/manifest.html
[dependencies]: https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html
