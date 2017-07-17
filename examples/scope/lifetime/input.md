<!-- A *lifetime* is a construct the compiler (also called the borrow checker)
uses to ensure all borrows are valid. Specifically, a variable's lifetime
begins when it is created and ends when it is destroyed. While lifetimes
and scopes are often referred to together, they are not the same.  -->
*ライフタイム*はコンパイラ(借用チェッカーと呼ばれる場合もあります)が、全ての借用に問題がないことを確認するために使用する仕組みです。正確にいうと、変数のライフタイムは作成時に開始し、破棄された時に終了します。ライフタイムとスコープは同時に語られることが多いですが、同じものではありません。

<!-- Take, for example, the case where we borrow a variable via `&`. The
borrow has a lifetime that is determined by where it is declared. As a result,
the borrow is valid as long as it ends before the lender is destroyed. However,
the scope of the borrow is determined by where the reference is used. -->
例として`&`を用いて変数を借用する場合をあげましょう。借用のライフタイムは宣言時に決定し、そこから貸し手が破棄されるまで続きます。しかしながら、借用のスコープは参照が使われる際に決定します。

<!-- In the following example and in the rest of this section, we will see how
lifetimes relate to scopes, as well as how the two differ. -->
以下の例からこのセクションの残りでは、ライフタイムとスコープの関係、そしてそれらがいかに異なるものであるかを見ていきます。

{lifetime.play}

<!-- Note that no names or types are assigned to label lifetimes.
This restricts how lifetimes will be able to be used as we will see. -->
ここで、一切の名前や型がライフタイムにアサインされていないことに注意しましょう。これにより、ライフタイムの使われ方がこれから見ていくようなやり方に制限されます。

