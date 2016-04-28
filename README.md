# rust-by-example-ja

rust-by-exampleの和訳リポジトリ

# ビルド方法

[nightly](http://www.rust-lang.org/install.html)版のRustをインストールしてください。
[multirust](https://github.com/brson/multirust)がオススメです。
`node`,`npm`,`subversion`を予めインストールしておく必要があります。


- `make all` ... `stage/`以下にソースコードと統合されたマークダウンを出力する
- `make book` ...[gitbook](https://www.gitbook.com/)を利用して`_book/`にhtmlを出力
- `make serve` ... ローカルのhtmlをブラウザ上で確認
- `./deploy.sh` ... Github-pagesに反映

翻訳文書は[こちら](http://rust-lang-ja.github.io/rust-by-example-ja/index.html)
