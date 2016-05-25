fn main() {
    let vec1 = vec![1, 2, 3];
    let vec2 = vec![4, 5, 6];

    // ベクトル型に対する`iter`は`&i32`を`yield`する。
    let mut iter = vec1.iter();
    // `inter_iter()`の場合は`i32`を`yield`する。
    let mut into_iter = vec2.into_iter();

    // `yield`された要素へのリファレンスは`&&i32`となる。`i32`へとデストラクトする。
    println!("Find 2 in vec1: {:?}", iter     .find(|&&x| x == 2));
    // `into_iter`の場合は`&i32`が要素のリファレンス。
    println!("Find 2 in vec2: {:?}", into_iter.find(| &x| x == 2));

    let array1 = [1, 2, 3];
    let array2 = [4, 5, 6];

    // 配列に対する`iter`も`&i32`を`yield`する。
    println!("Find 2 in array1: {:?}", array1.iter()     .find(|&&x| x == 2));
    // 配列に`into_iter()`を使うと例外的に`&i32`を`yield`する。
    println!("Find 2 in array2: {:?}", array2.into_iter().find(|&&x| x == 2));
}

