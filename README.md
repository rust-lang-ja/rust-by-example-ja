# rust-by-example-ja

[rust-by-example](https://github.com/rust-lang/rust-by-example)の和訳リポジトリ

# ビルド方法

[nightly](http://www.rust-lang.org/install.html)版のRustをインストールしてください。

[rustup](https://www.rustup.rs/)がオススメです。
`node`,`npm`,`subversion`を予めインストールしておく必要があります。


- `make all` ... `stage/`以下にソースコードと統合されたマークダウンを出力する
- `make book` ...[gitbook](https://www.gitbook.com/)を利用して`_book/`にhtmlを出力
- `make serve` ... ローカルのhtmlをブラウザ上で確認
- `./deploy.sh` ... Github-pagesに反映

翻訳文書は[こちら](http://rust-lang-ja.org/rust-by-example/index.html)
