# Rust By Example

[Rust by Example](https://github.com/rust-lang/rust-by-example)の和訳リポジトリ [![CircleCI](https://circleci.com/gh/rust-lang-ja/rust-by-example-ja.svg?style=svg)](https://circleci.com/gh/rust-lang-ja/rust-by-example-ja)

和訳版は[こちら](https://doc.rust-jp.rs/rust-by-example-ja/)から読めます。

# ビルド方法

安定版（stable版）のRustをインストールしてください。
[rustup](https://www.rustup.rs/)がオススメです。

```bash
$ git clone https://github.com/rust-lang-ja/rust-by-example-ja
$ cd rust-by-example-ja
$ cargo install mdbook
$ mdbook build
$ mdbook serve
```

# テスト方法

```bash
$ cd rust-by-example-ja
$ cargo install mdbook-transcheck
$ mdbook test
$ git submodule init
$ git submodule update
$ mdbook-transcheck src-en src
```

`mdbook test`はドキュメント内のソースコードがコンパイル可能かどうかを、
`mdbook-transcheck`はコメントアウトされた原文とソースコードがオリジナルと一致するかどうかをテストします。

# 翻訳の仕方

[CONTRIBUTING](./CONTRIBUTING.md)を参照してください。
