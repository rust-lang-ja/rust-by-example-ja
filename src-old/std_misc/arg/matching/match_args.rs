use std::env;

fn increase(number: i32) {
    println!("{}", number + 1);
}

fn decrease(number: i32) {
    println!("{}", number - 1);
}

fn help() {
    println!("usage:
match_args <string>
    Check whether given string is the answer.
match_args {{increase|decrease}} <integer>
    Increase or decrease given integer by one.");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        // 引数がない場合
        1 => {
            println!("My name is 'match_args'. Try passing some arguments!");
        },
        // 引数が1つの場合
        2 => {
            match args[1].parse() {
                Ok(42) => println!("This is the answer!"),
                _ => println!("This is not the answer."),
            }
        },
        // コマンドが一つと引数が一つの場合
        3 => {
            let cmd = &args[1];
            let num = &args[2];
            // 数字をパース
            let number: i32 = match num.parse() {
                Ok(n) => {
                    n
                },
                Err(_) => {
                    println!("error: second argument not an integer");
                    help();
                    return;
                },
            };
            // コマンドをパース
            match &cmd[..] {
                "increase" => increase(number),
                "decrease" => decrease(number),
                _ => {
                    println!("error: invalid command");
                    help();
                },
            }
        },
        // その他の場合
        _ => {
            // ヘルプメッセージを表示
            help();
        }
    }
}
