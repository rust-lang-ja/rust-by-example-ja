<!-- The compiler provides a `dead_code`
[*lint*][lint] that will warn
about unused functions. An *attribute* can be used to disable the lint. -->
コンパイラは`dead_code`と呼ばれる[リント][lint]機能を持つため、使用されていない関数が存在するときに警告を出します。*アトリビュート*によってこの機能を無効化することができます。

{unused.play}

<!-- Note that in real programs, you should eliminate dead code. In these examples
we'll allow dead code in some places because of the interactive nature of the
examples. -->
実際のコード中では、使用されていないコードが有る場合はそれを除外するべきです。この文書中では随所でアトリビュートによって警告を抑制していますが、それはあくまでインタラクティブな例を皆さんに提供するためです。

[lint]: https://en.wikipedia.org/wiki/Lint_%28software%29
