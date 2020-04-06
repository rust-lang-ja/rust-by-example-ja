<!--
# Methods
-->
# メソッド

<!--
Methods are functions attached to objects. These methods have access to the
data of the object and its other methods via the `self` keyword. Methods are
defined under an `impl` block.
-->
メソッドとはオブジェクトに付属した関数のことです。オブジェクトの持つデータや他のメソッドへは`self`という語を介してアクセスすることができます。`impl`キーワードによって定義します。

```rust,editable
struct Point {
    x: f64,
    y: f64,
}

// Implementation block, all `Point` methods go in here
// メソッドの実装のためのブロック。`Point`の持つメソッドを全て定義する。
impl Point {
    // This is a static method
    // Static methods don't need to be called by an instance
    // These methods are generally used as constructors
    // スタティックメソッド。つまり、インスタンスからでなくても
    // 呼び出せるメソッド。以下のようにコンストラクタとして使用されることが多い。
    fn origin() -> Point {
        Point { x: 0.0, y: 0.0 }
    }

    // Another static method, taking two arguments:
    // もう一つスタティックメソッド。引数を2つ取る。
    fn new(x: f64, y: f64) -> Point {
        Point { x: x, y: y }
    }
}

struct Rectangle {
    p1: Point,
    p2: Point,
}

impl Rectangle {
    // This is an instance method
    // `&self` is sugar for `self: &Self`, where `Self` is the type of the
    // caller object. In this case `Self` = `Rectangle`
    // こちらはインスタンスメソッド。`&self`は`self: &Self`の糖衣構文。
    // `Self`は呼び出し元オブジェクトの型。この場合は`Rectangle`。
    fn area(&self) -> f64 {
        // `self` gives access to the struct fields via the dot operator
        // `self`はドット演算子によって構造体のfieldを参照できる。
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        // `abs` is a `f64` method that returns the absolute value of the
        // caller
        // `abs`は`f64`のメソッドで、呼び出し元の値の絶対値を返す。
        ((x1 - x2) * (y1 - y2)).abs()
    }

    fn perimeter(&self) -> f64 {
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        2.0 * ((x1 - x2).abs() + (y1 - y2).abs())
    }

    // This method requires the caller object to be mutable
    // `&mut self` desugars to `self: &mut Self`
    // このメソッドは呼び出し元オブジェクトがミュータブルであることを
    // 必要とする。`&mut self`は`self: &mut Self`の糖衣構文である。
    fn translate(&mut self, x: f64, y: f64) {
        self.p1.x += x;
        self.p2.x += x;

        self.p1.y += y;
        self.p2.y += y;
    }
}

// `Pair` owns resources: two heap allocated integers
// `Pair`はヒープ上の整数を2つ保持する。
struct Pair(Box<i32>, Box<i32>);

impl Pair {
    // This method "consumes" the resources of the caller object
    // `self` desugars to `self: Self`
    // このメソッドは呼び出し元オブジェクトの持つ要素を「消費」する。
    // `self`は`self: Self`の糖衣構文である。
    fn destroy(self) {
        // Destructure `self`
        // `self`をデストラクト
        let Pair(first, second) = self;

        println!("Destroying Pair({}, {})", first, second);

        // `first` and `second` go out of scope and get freed
        // `first`、`second`はスコープから抜け出すと同時に、解放される。
    }
}

fn main() {
    let rectangle = Rectangle {
        // Static methods are called using double colons
        // スタティックメソッドはコロンを2つ挟んで呼び出される。
        p1: Point::origin(),
        p2: Point::new(3.0, 4.0),
    };

    // Instance methods are called using the dot operator
    // Note that the first argument `&self` is implicitly passed, i.e.
    // インスタンスメソッドはドット演算子を用いて呼び出される。
    // 最初の引数`&self`は明示せずに受け渡されていることに注目。つまり
    // `rectangle.perimeter()` === `Rectangle::perimeter(&rectangle)`
    println!("Rectangle perimeter: {}", rectangle.perimeter());
    println!("Rectangle area: {}", rectangle.area());

    let mut square = Rectangle {
        p1: Point::origin(),
        p2: Point::new(1.0, 1.0),
    };

    // Error! `rectangle` is immutable, but this method requires a mutable
    // object
    // エラー！`rectangle`はイミュータブルだがこのメソッドはミュータブルなオブジェクトを
    // 必要とする。
    //rectangle.translate(1.0, 0.0);
    // TODO ^ Try uncommenting this line
    // TODO ^ この行をアンコメントしてみましょう。

    // Okay! Mutable objects can call mutable methods
    // OK! ミュータブルなオブジェクトはミュータブルなメソッドを呼び出せる。
    square.translate(1.0, 1.0);

    let pair = Pair(Box::new(1), Box::new(2));

    pair.destroy();

    // Error! Previous `destroy` call "consumed" `pair`
    // エラー！先ほどの`destroy`で`pair`はすでに消費されてしまっている。
    //pair.destroy();
    // TODO ^ Try uncommenting this line
    // TODO ^ この行をアンコメントしてみましょう。
}
```
