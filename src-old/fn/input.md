<!-- Functions are declared using the `fn` keyword. Its arguments are type
annotated, just like variables, and, if the function returns a value, the
return type must be specified after an arrow `->`. -->
関数は`fn`キーワードを用いて定義することができます。引数は変数と同様に型を指定する必要があり、もし関数が値を返すならば`->`の後にその型も指定する必要があります。

<!-- The final expression in the function will be used as return value.
Alternatively, the `return` statement can be used to return a value earlier
from within the function, even from inside loops or `if`s. -->
関数内の最後の式文が返り値となります。関数の途中で値を返したい場合は`return`文を使用します。`loop`の最中や`if`文の中からも値を返すことができます。

<!-- Let's rewrite FizzBuzz using functions! -->
では、もう一度FizzBuzz問題を解く関数を書いてみましょう！

{fn.play}
