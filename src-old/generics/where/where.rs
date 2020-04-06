use std::fmt::Debug;

trait PrintInOption {
    fn print_in_option(self);
}

// `where`句を用いない場合、以下と等価な機能を実装するには、
// `<T: Debug>`という形で表現するか、別の直接的でない方法
// を使用するかしなくてはならない。
impl<T> PrintInOption for T where
    Option<T>: Debug {
    // プリントされるのが`Some(self)`であるため、この関数の
    // ジェネリック境界として`Option<T>: Debug`を使用したい。
    fn print_in_option(self) {
        println!("{:?}", Some(self));
    }
}

fn main() {
    let vec = vec![1, 2, 3];

    vec.print_in_option();
}
