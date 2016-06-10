<!-- A longer lifetime can be coerced into a shorter one
so that it works inside a scope it normally wouldn't work in.
This comes in the form of inferred coercion by the Rust compiler,
and also in the form of declaring a lifetime difference: -->
長いライフタイムは、短いものに圧縮(coerce)することで、そのままでは動作しないスコープの中でも使用できるようになります。これは、Rustコンパイラが推論の結果として圧縮する場合と、複数のライフタイムを比較して圧縮する場合があります。


{coercion.play}
