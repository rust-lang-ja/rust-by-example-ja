<!-- Rust provides a mechanism for spawning native OS threads via the `spawn`
function, the argument of this function is a moving closure. -->
Rustは`spawn`関数を用いてOSのネイティブスレッドを開始することができます。この関数の引数はmoveクロージャ(訳注: 参照ではなく値を取るクロージャ。　詳しくは[クロージャを返す関数][fn_output]を参照)です。

{threads.play}

<!-- These threads will be scheduled by the OS. -->
これらのスレッドのスケジューリングはOSによって行われる。


[fn_output]: ../fn/closures/output_parameters.html
