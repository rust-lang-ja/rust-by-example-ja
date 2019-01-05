<!--- There are three types of structures ("structs") that can be created using the --->
<!--- `struct` keyword: --->
`struct`というキーワードを用いて作成できる構造体(「structre」)には3種類あります。

<!--- * Tuple structs, which are, basically, named tuples. --->
<!--- * The classic [C structs][c_struct] --->
<!--- * Unit structs, which are field-less, are useful for generics. --->
* タプル。（ほとんどの場合は名前付きタプル）
* クラシックな[C言語スタイルの構造体。][c_struct]
* ユニット。これはフィールドを持たず、ジェネリック型を扱う際に有効です。

``` rust,editable
// ユニット
struct Nil;

// タプル
struct Pair(i32, f64);

// 2つのフィールドを持つ（クラシックな）構造体
struct Point {
    x: f64,
    y: f64,
}

// 構造体は他の構造体のフィールドになることができる
#[allow(dead_code)]
struct Rectangle {
    p1: Point,
    p2: Point,
}

fn main() {
    // `Point`型のインスタンスを作成
    let point: Point = Point { x: 0.3, y: 0.4 };

    // pointのフィールドにアクセスする。
    println!("point coordinates: ({}, {})", point.x, point.y);

    // `let`を使用してpointをデストラクトする。
    let Point { x: my_x, y: my_y } = point;

    let _rectangle = Rectangle {
        // 構造体の定義とインスタンスの作成を同時に行う
        p1: Point { x: my_y, y: my_x },
        p2: point,
    };

    // ユニットをインスタンス化
    let _nil = Nil;

    // タプルをインスタンス化
    let pair = Pair(1, 0.1);

    // タプルをデストラクト
    let Pair(integer, decimal) = pair;

    println!("pair contains {:?} and {:?}", integer, decimal);
}

```

### See also:

[アトリビュート(`attributes`)][attributes] and [デストラクト][destructuring]

[attributes]: ./attribute.html
[c_struct]: http://en.wikipedia.org/wiki/Struct_(C_programming_language)
[destructuring]: ./flow_control/match/destructuring.html
