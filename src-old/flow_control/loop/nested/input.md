<!--- It's possible to `break` or `continue` outer loops when dealing with nested --->
<!--- loops. In these cases, the loops must be annotated with some `'label`, and the --->
<!--- label must be passed to the `break`/`continue` statement. --->
ネストしたループを回している時に外側のループを`break`または`continue`したい場合があります。こういった場合には`label`を用いてループにラベルを貼り、`break`/`continue`にそのラベルを渡します。

{nested.play}
