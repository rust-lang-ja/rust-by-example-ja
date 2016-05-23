struct Point {
    x: f64,
    y: f64,
}

// メソッドの実装のためのブロック。`Point`の持つメソッドを全て定義する。
impl Point {
    // スタティックメソッド。つまり、インスタンスからでなくても
    // 呼び出せるメソッド。以下のようにコンストラクタとして使用されることが多い。
    fn origin() -> Point {
        Point { x: 0.0, y: 0.0 }
    }

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
    // こちらはインスタンスメソッド。`&self`は`self: &Self`の糖衣構文。
    // `Self`は呼び出し元オブジェクトの型。この場合は`Rectangle`。
    fn area(&self) -> f64 {
        // `self`はドット演算子によって構造体のfieldを参照できる。
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        // `abs`は`f64`のメソッドで、呼び出し元の値の絶対値を返す。
        ((x1 - x2) * (y1 - y2)).abs()
    }

    fn perimeter(&self) -> f64 {
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        2.0 * ((x1 - x2).abs() + (y1 - y2).abs())
    }

    // このメソッドは呼び出し元オブジェクトがミュータブルであることを
    // 必要とする。`&mut self`は`self: &mut Self`の糖衣構文である。
    fn translate(&mut self, x: f64, y: f64) {
        self.p1.x += x;
        self.p2.x += x;

        self.p1.y += y;
        self.p2.y += y;
    }
}

// `Pair`はヒープ上の整数を2つ保持する。
struct Pair(Box<i32>, Box<i32>);

impl Pair {
    // このメソッドは呼び出し元オブジェクトの持つ要素を「消費」する。
    // `self`は`self: Self`の糖衣構文である。
    fn destroy(self) {
        // `self`をデストラクト
        let Pair(first, second) = self;

        println!("Destroying Pair({}, {})", first, second);

        // `first`、`second`はスコープから抜け出すと同時に、解放される。
    }
}

fn main() {
    let rectangle = Rectangle {
        // スタティックメソッドはコロンを2つ挟んで呼び出される。
        p1: Point::origin(),
        p2: Point::new(3.0, 4.0),
    };

    // インスタンスメソッドはドット演算子を用いて呼び出される。
    // 最初の引数`&self`は明示せずに受け渡されていることに注目。つまり
    // `rectangle.perimeter()` === `perimeter(&rectangle)`
    println!("Rectangle perimeter: {}", rectangle.perimeter());
    println!("Rectangle area: {}", rectangle.area());

    let mut square = Rectangle {
        p1: Point::origin(),
        p2: Point::new(1.0, 1.0),
    };

    // エラー！`rectangle`はイミュータブルだがこのメソッドはミュータブルなオブジェクトを
    // 必要とする。
    //rectangle.translate(1.0, 0.0);
    // TODO ^ この行をアンコメントしてみましょう。

    // Okay! Mutable objects can call mutable methods
    // OK! ミュータブルなオブジェクトはミュータブルなメソッドを呼び出せる。
    square.translate(1.0, 1.0);

    let pair = Pair(Box::new(1), Box::new(2));

    pair.destroy();

    // エラー！先ほどの`destroy`で`pair`はすでに消費されてしまっている。
    //pair.destroy();
    // TODO ^ この行をアンコメントしてみましょう。
}
