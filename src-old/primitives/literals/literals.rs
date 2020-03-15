fn main() {
    // 整数の足し算
    println!("1 + 2 = {}", 1u32 + 2);

    // 整数の引き算
    println!("1 - 2 = {}", 1i32 - 2);
    // TODO ^ 型が重要であることを実感するため`1i32`を`1u32`に変更してみましょう。

    // 単純な論理演算子
    println!("true AND false is {}", true && false);
    println!("true OR false is {}", true || false);
    println!("NOT true is {}", !true);

    // ビットワイズ演算
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);

    // 可読性のための`_`（アンダースコア）の使用
    println!("One million is written as {}", 1_000_000u32);
}
