<!-- Because variables are in charge of freeing their own resources,
**resources can only have one owner**. This also prevents resources
from being freed more than once. Note that not all variables own
resources (e.g. [references]). -->
変数には自身の保持する資源を開放する責任があるため、**資源は一度に一つの所有者**しか持つことができません。これはまた、資源を2度以上開放することができないということでもあります。ここで、全ての変数が資源を所有するわけではないということに注意しましょう。(e.g. [参照][references])

<!-- When doing assignments (`let x = y`) or passing function arguments by value
(`foo(x)`), the *ownership* of the resources is transferred. In Rust-speak,
this is known as a *move*. -->
変数をアサインする(`let x = y`)際や、関数に引数を値渡しする(`foo(x)`)際は、資源の*所有権(`ownership`)*が移動します。Rustっぽく言うと、「*ムーブ*」です。

<!-- After moving resources, the previous owner can no longer be used. This avoids
creating dangling pointers. -->
資源を移動すると、それまでの所有者(訳注: 変数などのこと)を使用することはできなくなります。これによりダングリングポインタの発生を防げます。

{move.play}

[references]: /flow_control/match/destructuring/destructure_pointers.html
