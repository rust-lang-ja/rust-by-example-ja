// 使用されていないコードよる警告を隠すアトリビュート
#![allow(dead_code)]

enum Status {
    Rich,
    Poor,
}

enum Work {
    Civilian,
    Soldier,
}

fn main() {
    // `use`することで絶対名でなくとも使用可能になる。
    use Status::{Poor, Rich};
    // `Work`の中の名前をすべて`use`する
    use Work::*;

    // `use`しているため、`Status::Poor`と書いていることに等しい
    let status = Poor;
    // `Work::Civilian`に等しい
    let work = Civilian;

    match status {
        // `use`しているのでスコープを明示していない
        Rich => println!("The rich have lots of money!"),
        Poor => println!("The poor have no money..."),
    }

    match work {
        // こちらも同じ
        Civilian => println!("Civilians work!"),
        Soldier  => println!("Soldiers fight!"),
    }
}
