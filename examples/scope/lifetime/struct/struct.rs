// `i32`への参照をメンバに持つ`Borrowed`型。
// 参照は`Borrowed`自体よりも長生きでなくてはならない。
#[derive(Debug)]
struct Borrowed<'a>(&'a i32);

// 同様に、ここでも参照は構造体よりも長生きでなくてはならない。
#[derive(Debug)]
struct NamedBorrowed<'a> {
    x: &'a i32,
    y: &'a i32,
}

// `i32`、あるいは`i32`への参照のいずれかとなる列挙型
#[derive(Debug)]
enum Either<'a> {
    Num(i32),
    Ref(&'a i32),
}

fn main() {
    let x = 18;
    let y = 15;

    let single = Borrowed(&x);
    let double = NamedBorrowed { x: &x, y: &y };
    let reference = Either::Ref(&x);
    let number    = Either::Num(y);

    println!("x is borrowed in {:?}", single);
    println!("x and y are borrowed in {:?}", double);
    println!("x is borrowed in {:?}", reference);
    println!("y is *not* borrowed in {:?}", number);
}
