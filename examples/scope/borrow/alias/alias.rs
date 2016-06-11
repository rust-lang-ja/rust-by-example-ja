struct Point { x: i32, y: i32, z: i32 }

fn main() {
    let mut point = Point { x: 0, y: 0, z: 0 };

    {
        let borrowed_point = &point;
        let another_borrow = &point;

        // データは元々の持ち主と参照の両方からアクセスすることができます。
        println!("Point has coordinates: ({}, {}, {})",
                 borrowed_point.x, another_borrow.y, point.z);

        // エラー！pointはすでにイミュータブルに借用されているため、
        // ミュータブルに借用することができない。
        //let mutable_borrow = &mut point;
        // TODO ^ この行をアンコメントしてみましょう。

        // ここでイミュータブルな参照がスコープを抜ける。
    }

    {
        let mutable_borrow = &mut point;

        // ミュータブルなリファレンスを介してデータを変更する
        mutable_borrow.x = 5;
        mutable_borrow.y = 2;
        mutable_borrow.z = 1;

        // エラー！`point`はすでにミュータブルに借用されているため、
        // イミュータブルに借用することはできない。
        //let y = &point.y;
        // TODO ^ この行をアンコメントしてみましょう。

        // エラー！`println!`はイミュータブルなリファレンスを取るため、printできません。
        //println!("Point Z coordinate is {}", point.z);
        // TODO ^ この行をアンコメントしてみましょう。

        // ここでミュータブルな参照がスコープを抜ける。
    }

    // pointへのイミュータブルな参照を使うことが再び許される。
    println!("Point now has coordinates: ({}, {}, {})",
             point.x, point.y, point.z);
}
