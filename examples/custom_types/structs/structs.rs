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
