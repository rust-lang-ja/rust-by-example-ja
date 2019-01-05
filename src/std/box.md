<!-- All values in Rust are stack allocated by default. Values can be *boxed*
(allocated in the heap) by creating a `Box<T>`. A box is a smart pointer to a
heap allocated value of type `T`. When a box goes out of scope, its destructor
is called, the inner object is destroyed, and the memory in the heap is freed. -->
Rustにおいて、すべての値はデフォルトでスタックに割り当てられます。`Box<T>`を作成することで、値を*ボックス化*、すなわちヒープ上に割り当てることができます。ボックスとは正確にはヒープ上におかれた`T`の値へのスマートポインタです。ボックスがスコープを抜けると、デストラクタが呼ばれて内包するオブジェクトが破棄され、ヒープメモリが解放されます。

<!-- Boxed values can be dereferenced using the `*` operator; this removes one layer
of indirection. -->
ボックス化された値は`*`オペレータを用いてデリファレンスすることができます。これにより一段と直接的な操作が可能になります。

``` rust,editable
use std::mem;

#[derive(Clone, Copy)]
struct Point {
    x: f64,
    y: f64,
}

#[allow(dead_code)]
struct Rectangle {
    p1: Point,
    p2: Point,
}

fn origin() -> Point {
    Point { x: 0.0, y: 0.0 }
}

fn boxed_origin() -> Box<Point> {
    // このPointをヒープ上に割り当て、ポインタを返す。
    Box::new(Point { x: 0.0, y: 0.0 })
}

fn main() {

    // (以下では型を全て明示していますが、必須ではありません。)
    // この変数ははすべてスタック上に割り当てられる。
    let point: Point = origin();
    let rectangle: Rectangle = Rectangle {
        p1: origin(),
        p2: Point { x: 3.0, y: 4.0 }
    };

    // ヒープ上に割り当てられたRectangle
    let boxed_rectangle: Box<Rectangle> = Box::new(Rectangle {
        p1: origin(),
        p2: origin()
    });

    // 関数の返り値をボックス化
    let boxed_point: Box<Point> = Box::new(origin());

    // 間にもう一つポインタを挟む
    let box_in_a_box: Box<Box<Point>> = Box::new(boxed_origin());

    println!("Point occupies {} bytes in the stack",
             mem::size_of_val(&point));
    println!("Rectangle occupies {} bytes in the stack",
             mem::size_of_val(&rectangle));

    // ボックスのサイズはポインタのサイズに等しい
    println!("Boxed point occupies {} bytes in the stack",
             mem::size_of_val(&boxed_point));
    println!("Boxed rectangle occupies {} bytes in the stack",
             mem::size_of_val(&boxed_rectangle));
    println!("Boxed box occupies {} bytes in the stack",
             mem::size_of_val(&box_in_a_box));

    // `boxed_point`の保持するデータを`unboxed_point`にコピーする
    let unboxed_point: Point = *boxed_point;
    println!("Unboxed point occupies {} bytes in the stack",
             mem::size_of_val(&unboxed_point));
}

```
