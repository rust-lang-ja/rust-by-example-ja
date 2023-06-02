<!--
# Playground
-->
# プレイグラウンド

<!--
The [Rust Playground](https://play.rust-lang.org/) is a way to experiment with Rust code through a web interface.
-->
[Rust Playground](https://play.rust-lang.org/)では、RustのコードをWebのインターフェースを通じて実験できます。

<!--
## Using it with `mdbook`
-->
## `mdbook`と組み合わせる

<!--
In [`mdbook`][mdbook], you can make code examples playable and editable.
-->
[`mdbook`][mdbook]では、コード例を実行・編集可能にできます。

```rust,editable
fn main() {
    println!("Hello World!");
}
```

<!--
This allows the reader to both run your code sample, but also modify and tweak it. The key here is the adding the word `editable` to your codefence block separated by a comma.
-->
これにより、読者はあなたのコード例を実行するだけでなく、変更することもできます。
`editable`という単語をカンマで区切って、あなたのコードブロックに追加するのがキーです。

<!--
````markdown
```rust,editable
//...place your code here
```
````
-->
````markdown
```rust,editable
//...ここにあなたのコードを書きます
```
````

<!--
Additionally, you can add `ignore` if you want `mdbook` to skip your code when it builds and tests.
-->
加えて、`mdbook`がビルドやテストを実行するときに、あなたのコードをスキップさせたい場合は、`ignore`を追加できます。

<!--
````markdown
```rust,editable,ignore
//...place your code here
```
````
-->
````markdown
```rust,editable,ignore
//...ここにあなたのコードを書きます
```
````

<!--
## Using it with docs
-->
## ドキュメントと組み合わせる

<!--
You may have noticed in some of the [official Rust docs][official-rust-docs] a button that says "Run", which opens the code sample up in a new tab in Rust Playground. This feature is enabled if you use the #[doc] attribute called [`html_playground_url`][html-playground-url].
-->
[Rustの公式ドキュメント][official-rust-docs]には、「実行(Run)」ボタンがある場合があります。
クリックすると、新しいタブでRust Playgroundが開き、コード例が表示されます。
この機能は、#[doc]アトリビュートの[`html_playground_url`][html-playground-url]の値で有効化できます。

<!--
### See also:
-->
### 参照

- [The Rust Playground][rust-playground]
- [rust-playground][rust-playground]
- [The rustdoc Book][rustdoc-book]

[rust-playground]: https://play.rust-lang.org/
[rust-playground]: https://github.com/integer32llc/rust-playground/
[mdbook]: https://github.com/rust-lang/mdBook
[official-rust-docs]: https://doc.rust-lang.org/core/
[rustdoc-book]: https://doc.rust-lang.org/rustdoc/what-is-rustdoc.html
[html-playground-url]: https://doc.rust-lang.org/rustdoc/the-doc-attribute.html#html_playground_url
