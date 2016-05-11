fn main() {
    let x = 5u32;

    let y = {
        let x_squared = x * x;
        let x_cube = x_squared * x;

        // この式文は`y`に代入されます。
        x_cube + x_squared + x
    };

    let z = {
        // セミコロンがあるので`z`には`()`が入ります。
        2 * x;
    };

    println!("x is {:?}", x);
    println!("y is {:?}", y);
    println!("z is {:?}", z);
}
