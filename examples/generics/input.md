<!-- Generics is the topic of generalizing types and functionality to broader
cases. This is extremely useful in reducing code duplication in many ways,
but requires a rather involving syntax. However, we will find that being
generic involves taking great care to specify over what types a generic type
is actually considered valid. -->
ジェネリクスとは、型と関数の機能をより汎用的に使えるようにするための機能です。これはあらゆる局面でコードの重複を避けるために非常に役立ちますが、多少構文が複雑になります。すなわち、ジェネリック型を使いこなすには、どのようなジェネリック型がきちんと機能するかに細心の注意を払う必要があります。

<!-- A type parameter is specified as generic by the use of angle brackets and
[camel case][camelcase]: `<A, B, ...>`. "Generic type parameters" are
typically represented as `<T>`. In Rust, "generic" also describes anything that
accepts one or more generic type parameters `<T>`. Any type specified as a
generic type parameter is generic, and everything else is concrete (non-generic). -->
ジェネリック型の型パラメータにはかぎ括弧(`angle brackets`)と[キャメルケース(`camel case`)][camelcase]が使われます。: `<Aaa, Bbb, ...>`ジェネリックな型パラメータはたいていの場合`<T>`で示されます。Rustの場合、「ジェネリクス」には「１つ以上のジェネリック型を受け付けるもの」という意味もあります。ジェネリックな型パラメータを指定された場合、それは必ずジェネリック型になり、そうでなければ必ず非ジェネリック型、すなわち具象型(`concrete`)になります。

<!-- For example, defining a *generic function* named `foo` that takes an argument
`T` of any type: -->
例として、あらゆる型の引数`T`をとる*ジェネリック関数*`foo`を定義すると:

```rust
fn foo<T>(T) { ... }
```

<!-- Because `T` has been specified as a generic type parameter, it is considered
generic when used here as `(T)`. This is the case even if `T` has previously
been defined as a `struct`. -->
となります。`T`はジェネリックな型パラメータを付与されているので、`T`として使用するとジェネリック型として扱われます。これは`T`という構造体がそれ以前に定義されていても同様です。

<!-- This example shows some of the syntax in action: -->
 では、手を動かしながらジェネリック型の構文を体験していきましょう。

{generics.play}

### See also:

[構造体][structs]

[structs]: ./custom_types/structs.html
[camelcase]: https://en.wikipedia.org/wiki/CamelCase
