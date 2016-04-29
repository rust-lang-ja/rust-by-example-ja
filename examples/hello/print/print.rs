fn main() {
    // 一般的に `{} `はどんな引数であろうと自動的に置き換えられます。
    // 例えば以下は文字列に変換されます
    println!("{} days", 31);

    // サフィックスで型を指定しなければ31はi32として扱われます。
    // サフィックスの指定により、31の型を自由に変換することができます。

    // 引数の位置から埋め込まれる場所を指定することができます。
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    // 名前での指定も可能です。
    println!("{subject} {verb} {object}",
             object="the lazy dog",
             subject="the quick brown fox",
             verb="jumps over");

    // `:` のあとにフォーマット型を指定することによる特殊なフォーマットも可能です.
    println!("{} of {:b} people know binary, the other half don't", 1, 2);

    // 指定した幅の中に、右寄せで文字列を挿入することができます。
    // 以下の例では"     1". というように、５つの半角空白のあとに"1"が入ります.
    println!("{number:>width$}", number=1, width=6);

    // 空白の代わりに0を使うこともできます. このアウトプットは "000001" になります.
    println!("{number:>0width$}", number=1, width=6);

    // 引数の数が正しいかのチェックも行ってくれます。
    println!("My name is {0}, {1} {0}", "Bond");
    // FIXME ^ Add the missing argument: "James"

    // `i32`保持する `Structure` という名の構造体を定義します.
    struct Structure(i32);

    // このようにカスタム型を用いる場合、少々扱いが複雑になります。
    // 以下は動作しません。
    println!("This struct `{}` won't print...", Structure(3));
    // FIXME ^ Comment out this line.
}
