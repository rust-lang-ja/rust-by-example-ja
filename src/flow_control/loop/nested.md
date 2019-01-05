<!--- It's possible to `break` or `continue` outer loops when dealing with nested --->
<!--- loops. In these cases, the loops must be annotated with some `'label`, and the --->
<!--- label must be passed to the `break`/`continue` statement. --->
ネストしたループを回している時に外側のループを`break`または`continue`したい場合があります。こういった場合には`label`を用いてループにラベルを貼り、`break`/`continue`にそのラベルを渡します。

``` rust,editable
#![allow(unreachable_code)]

fn main() {
    'outer: loop {
        println!("Entered the outer loop");

        'inner: loop {
            println!("Entered the inner loop");

            // これは内側のループのみを中断します。
            //break;

            // こちらは外側を中断します
            break 'outer;
        }

        println!("This point will never be reached");
    }

    println!("Exited the outer loop");
}

```
