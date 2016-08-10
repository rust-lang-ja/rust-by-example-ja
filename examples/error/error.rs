fn give_princess(gift: &str) {
    // お姫様はヘビが大嫌いです。拒絶の意思をお示しに
    // なられたなら、直ちにストップしなくてはなりません。
    if gift == "snake" { panic!("AAAaaaaa!!!!"); }

    println!("I love {}s!!!!!", gift);
}

fn main() {
    give_princess("teddy bear");
    give_princess("snake");
}
