struct Container(i32, i32);

// ２つの要素がコンテナ型の中にあることをチェックするトレイト
// また、最初と最後の値を取得することもできる
trait Contains<A, B> {
    fn contains(&self, &A, &B) -> bool; // `A`と`B`の両方を明示的に要求する
    fn first(&self) -> i32; // `A`、`B`いずれも要求しない
    fn last(&self) -> i32;  // `A`、`B`いずれも要求しない

}

impl Contains<i32, i32> for Container {
    // コンテナ内の２つの要素が等しければTrueを返す
    fn contains(&self, number_1: &i32, number_2: &i32) -> bool {
        (&self.0 == number_1) && (&self.1 == number_2)
    }

    // ひとつ目の値を取得
    fn first(&self) -> i32 { self.0 }

    // 最後(2つめ)の値を取得
    fn last(&self) -> i32 { self.1 }
}

// `A`と`B`は`C`に保持されていることを考慮すると、`A`と`B`を
// ２度も書くのは面倒
fn difference<A, B, C>(container: &C) -> i32 where
    C: Contains<A, B> {
    container.last() - container.first()
}

fn main() {
    let number_1 = 3;
    let number_2 = 10;

    let container = Container(number_1, number_2);

    println!("Does container contain {} and {}: {}",
        &number_1, &number_2,
        container.contains(&number_1, &number_2));
    println!("First number: {}", container.first());
    println!("Last number: {}", container.last());

    println!("The difference is: {}", difference(&container));
}
