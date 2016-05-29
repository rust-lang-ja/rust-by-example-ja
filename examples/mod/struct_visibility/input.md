<!-- Structs have an extra level of visibility with their fields. The visibility
defaults to private, and can be overridden with the `pub` modifier. This
visibility only matters when a struct is accessed from outside the module
where it is defined, and has the goal of hiding information (encapsulation). -->
構造体はそれ自身に加え、フィールドごとにもパブリック・プライベートを設定することができます。デフォルトではプライベートですが、`pub`宣言をすることで、フィールドをパブリックにすることができます。これは、構造体がモジュールの外から参照される時に限り意味のあるもので、情報の隠蔽（カプセル化）を達成するための機能です。


{struct.play}

### See also:

[ジェネリック型][generics]、 [メソッド][methods]

[generics]: ../generics.html
[methods]: ../fn/methods.html
