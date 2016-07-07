use std::collections::HashSet;

fn main() {
    let mut a: HashSet<i32> = vec!(1i32, 2, 3).into_iter().collect();
    let mut b: HashSet<i32> = vec!(2i32, 3, 4).into_iter().collect();

    assert!(a.insert(4));
    assert!(a.contains(&4));

    // 既に存在する値を追加しようとすると
    // `HashSet::insert()`はfalseを返す。
    assert!(b.insert(4), "Value 4 is already in set B!");
    // FIXME ^ この行をコメントアウトしましょう。

    b.insert(5);

    // 集合の要素が、`Debug`を実装している型の場合、
    // 集合そのものも`Debug`を実装する。
    // 通常は`[elem1, elem2, ...]`のように要素をプリントする。
    println!("A: {:?}", a);
    println!("B: {:?}", b);

    // [1, 2, 3, 4, 5]を順不同にプリント
    println!("Union: {:?}", a.union(&b).collect::<Vec<&i32>>());

    // これは[1]をプリント
    println!("Difference: {:?}", a.difference(&b).collect::<Vec<&i32>>());

    // [2, 3, 4]を順不同にプリント
    println!("Intersection: {:?}", a.intersection(&b).collect::<Vec<&i32>>());

    // [1, 5]をプリント
    println!("Symmetric Difference: {:?}",
             a.symmetric_difference(&b).collect::<Vec<&i32>>());
}

