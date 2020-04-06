use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    // ひとつ目の引数はプログラムを呼び出す際に使用したパス
    println!("My path is {}.", args[0]);

    // 残りはプログラムに渡されたコマンドラインパラメータ。
    // プログラムはこんなふうに呼び出す。
    //   $ ./args arg1 arg2
    println!("I got {:?} arguments: {:?}.", args.len() - 1, &args[1..]);
}
