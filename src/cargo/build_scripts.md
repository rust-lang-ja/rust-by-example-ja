<!--
# Build Scripts
-->
# ビルドスクリプト

<!--
Sometimes a normal build from `cargo` is not enough. Perhaps your crate needs
some pre-requisites before `cargo` will successfully compile, things like code
generation, or some native code that needs to be compiled. To solve this problem
we have build scripts that Cargo can run.
-->
`cargo`による通常のビルドでは十分でないことが時々あります。コード生成や、コンパイルが必要なネイティブコードなど、`cargo`がクレートをうまくコンパイルするにはなんらかの前提条件が必要かもしれません。この問題を解決するため、Cargoが実行できるビルドスクリプトがあります。

<!--
To add a build script to your package it can either be specified in the
`Cargo.toml` as follows:
-->
ビルドスクリプトをパッケージに追加するには、以下のように`Cargo.toml`の中で指定できます。

```toml
[package]
...
build = "build.rs"
```

<!--
Otherwise Cargo will look for a `build.rs` file in the project directory by
default.
-->
それ以外の場合、Cargoはデフォルトでプロジェクトディレクトリから`build.rs`を探します。

<!--
## How to use a build script
-->
## ビルドスクリプトの使い方

<!--
The build script is simply another Rust file that will be compiled and invoked
prior to compiling anything else in the package. Hence it can be used to fulfill
pre-requisites of your crate.
-->
ビルドスクリプトは単にRustのファイルの1つで、パッケージ内の他のファイルをコンパイルする前にコンパイルされて起動されます。そのため、クレートの前提条件を満たすために使用できます。

<!--
Cargo provides the script with inputs via environment variables [specified
here] that can be used.
-->
Cargoは、[ここで指定された][specified here]環境変数を介してスクリプトに入力を与えます。

<!--
The script provides output via stdout. All lines printed are written to
`target/debug/build/<pkg>/output`. Further, lines prefixed with `cargo:` will be
interpreted by Cargo directly and hence can be used to define parameters for the
package's compilation.
-->
スクリプトは標準出力に出力します。出力される行は全て、`target/debug/build/<pkg>/output`に書き込まれます。さらに、行頭に`cargo:`がついた行はCargoに直接解釈されるため、パッケージのコンパイル時のパラメーターを定義するのに使用できます。

<!--
For further specification and examples have a read of the
[Cargo specification][cargo_specification].
-->
より詳細な仕様や例については、[Cargo specification][cargo_specification]を参照してください。

[specified here]: https://doc.rust-lang.org/cargo/reference/environment-variables.html#environment-variables-cargo-sets-for-build-scripts

[cargo_specification]: https://doc.rust-lang.org/cargo/reference/build-scripts.html
