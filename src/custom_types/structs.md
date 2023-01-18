<!--
# Structures
-->
# 構造体

<!--
There are three types of structures ("structs") that can be created using the
`struct` keyword:
-->
`struct`というキーワードを用いて作成できる構造体には3種類あります。

<!--
* Tuple structs, which are, basically, named tuples.
* The classic [C structs][c_struct]
* Unit structs, which are field-less, are useful for generics.
-->
* タプル。（すなわちタプルに名前が付いたようなもの）
* クラシックな[C言語スタイルの構造体。][c_struct]
* ユニット。これはフィールドを持たず、ジェネリック型を扱う際に有効です。

```rust,editable
// An attribute to hide warnings for unused code.
#![allow(dead_code)]

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

// A unit struct
// ユニット
struct Unit;

// A tuple struct
// タプル
struct Pair(i32, f32);

// A struct with two fields
// 2つのフィールドを持つ（クラシックな）構造体
struct Point {
    x: f32,
    y: f32,
}

// Structs can be reused as fields of another struct
// 構造体は他の構造体のフィールドになることができる
#[allow(dead_code)]
struct Rectangle {
    // A rectangle can be specified by where the top left and bottom right
    // corners are in space.
    // 長方形は座標空間上における左上隅と右下隅の位置によって指定できる
    top_left: Point,
    bottom_right: Point,
}

fn main() {
    // Create struct with field init shorthand
    // 構造体をフィールド初期化の簡略記法で生成
    let name = String::from("Peter");
    let age = 27;
    let peter = Person { name, age };

    // Print debug struct
    // 構造体のデバッグ表示を行う
    println!("{:?}", peter);


    // Instantiate a `Point`
    // `Point` のインスタンス化
    let point: Point = Point { x: 10.3, y: 0.4 };

    // Access the fields of the point
    // pointのフィールドにアクセスする。
    println!("point coordinates: ({}, {})", point.x, point.y);

    // Make a new point by using struct update syntax to use the fields of our
    // other one
    // 構造体の更新記法を用いて、別の構造体のフィールドの値を基に
    // 新たなpointを生成
    let bottom_right = Point { x: 5.2, ..point };

    // `bottom_right.y` will be the same as `point.y` because we used that field
    // from `point`
    // `bottom_right.y` の値は `point.y` と同一になるが、
    // これは `point` のフィールドの値を用いて生成したためである
    println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

    // Destructure the point using a `let` binding
    // `let`を使用してpointをデストラクトする。
    let Point { x: left_edge, y: top_edge } = point;

    let _rectangle = Rectangle {
        // struct instantiation is an expression too
        // 構造体の定義とインスタンスの作成を同時に行う
        top_left: Point { x: left_edge, y: top_edge },
        bottom_right: bottom_right,
    };

    // Instantiate a unit struct
    // ユニットをインスタンス化
    let _unit = Unit;

    // Instantiate a tuple struct
    // タプルをインスタンス化
    let pair = Pair(1, 0.1);

    // Access the fields of a tuple struct
    // タプルのフィールドにアクセス
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    // Destructure a tuple struct
    // タプルをデストラクト
    let Pair(integer, decimal) = pair;

    println!("pair contains {:?} and {:?}", integer, decimal);
}
```

<!--
### Activity
-->
### 演習

<!--
1. Add a function `rect_area` which calculates the area of a `Rectangle` (try
   using nested destructuring).
2. Add a function `square` which takes a `Point` and a `f32` as arguments, and
   returns a `Rectangle` with its top left corner on the point, and a width and
   height corresponding to the `f32`.
-->
1. `Rectangle` の面積を計算する `rect_area` 関数を追加してください。ネストしたデストラクトを使ってみましょう。
2. `Point` と `f32` を引数とした時に `Rectangle` を返す `square` 関数を追加してください。 `Rectangle` の左下の点が `Point` になり、`f32` が `Rectangle` の幅と高さになります。

### See also

<!--
[`attributes`][attributes], and [destructuring][destructuring]
-->
[アトリビュート(`attributes`)][attributes], [デストラクト][destructuring]

[attributes]: ../attribute.md
[c_struct]: https://en.wikipedia.org/wiki/Struct_(C_programming_language)
[destructuring]: ../flow_control/match/destructuring.md
