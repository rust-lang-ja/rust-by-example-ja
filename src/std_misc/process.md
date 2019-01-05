<!-- The `ProcessOutput` struct represents the output of a finished child process,
and the `Command` struct is a process builder. -->
`ProcessOutput`構造体は終了したプロセスのアウトプットを表し、`Command`構造体はプロセスの作成を行います。

``` rust,editable
use std::process::Command;

fn main() {
    let output = Command::new("rustc")
        .arg("--version")
        .output().unwrap_or_else(|e| {
            panic!("failed to execute process: {}", e)
    });

    if output.status.success() {
        let s = String::from_utf8_lossy(&output.stdout);

        print!("rustc succeeded and stdout was:\n{}", s);
    } else {
        let s = String::from_utf8_lossy(&output.stderr);

        print!("rustc failed and stderr was:\n{}", s);
    }
}

```

<!-- (You are encouraged to try the previous example with an incorrect flag passed
to `rustc`) -->
(余裕があれば、上の例で`rustc`に不正なフラグを渡し、どうなるか見てみましょう)
