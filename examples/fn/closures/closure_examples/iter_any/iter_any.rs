fn main() {
    let vec1 = vec![1, 2, 3];
    let vec2 = vec![4, 5, 6];

    // ベクトル型に対する`iter`は`&i32`を`yield`するので、`i32`へとデストラクト
    println!("2 in vec1: {}", vec1.iter()     .any(|&x| x == 2));
    // `into_iter()`の場合は`i32`を`yield`するので、デストラクトする必要はない。
    println!("2 in vec2: {}", vec2.into_iter().any(| x| x == 2));

    let array1 = [1, 2, 3];
    let array2 = [4, 5, 6];

    // 配列に対する`iter()`は`&i32`をyieldする。
    println!("2 in array1: {}", array1.iter()     .any(|&x| x == 2));
    // 配列に`into_iter()`を使うと例外的に`&i32`を`yield`する。
    println!("2 in array2: {}", array2.into_iter().any(|&x| x == 2));
}
