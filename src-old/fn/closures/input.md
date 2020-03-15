<!-- Closures in Rust, also called lambdas, are functions
that can capture the enclosing environment. Their syntax and capabilities make them
very convenient for on the fly usage. Some characteristics include: -->
Rustのクロージャはラムダ(`lambda`)とも呼ばれ、自身が定義された環境の外側の変数を参照できるような関数のことを指します。クロージャの文法と性質はサッと何かを仕上げたいときに非常に有用です。以下がクロージャの性質の一部です。


<!-- * uses `||` instead of `()` around input variables.
* *both* input and return *types* can be inferred.
* input variable *names* must be specified.
* body delimination (`{}`) is optional for a single expression. Mandatory
otherwise.
* the outer environment variables *may* be captured.
* calling a closure is exactly like a function: `call(var)`. -->
* 引数の周りを、`()`の代わりに`||`で囲む。
* 引数と返り値の*型*の*両方*を推測してくれる。
* 引数の*名前*は明示しなくてはならない。
* 関数が１文で終わるときは`{}`で関数のbodyを囲む必要はない。複数文の場合は必須。
* 外側の環境の変数を参照することができる。
* クロージャの呼び出しは関数と全く一緒である。例: `call(var)`

{closures.play}
