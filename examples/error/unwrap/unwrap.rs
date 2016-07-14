// 庶民(commoner)は経験豊富なので、大体どんな状況にも対処できます。
// あらゆる贈り物は`match`を用いて手動で処理されます。
fn give_commoner(gift: Option<&str>) {
    // Specify a course of action for each case.
    match gift {
        Some("snake") => println!("Yuck! I'm throwing that snake in a fire."),
        Some(inner)   => println!("{}? How nice.", inner),
        None          => println!("No gift? Oh well."),
    }
}

// 温室育ちのお姫様はヘビを見ると`panic`します。
fn give_princess(gift: Option<&str>) {
    // `unwrap`を使用すると値が`None`だった際に`panic`を返します。。
    let inside = gift.unwrap();
    if inside == "snake" { panic!("AAAaaaaa!!!!"); }

    println!("I love {}s!!!!!", inside);
}

fn main() {
    let food  = Some("chicken");
    let snake = Some("snake");
    let void  = None;

    give_commoner(food);
    give_commoner(snake);
    give_commoner(void);

    let bird = Some("robin");
    let nothing = None;

    give_princess(bird);
    give_princess(nothing);
}
