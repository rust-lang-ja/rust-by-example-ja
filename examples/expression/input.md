<!-- A Rust program is (mostly) made up of a series of statements: -->
Rustのプログラムは（ほとんどの場合）文(`statement`)の連続でできています


```
fn main() {
    // statement
    // statement
    // statement
}
```

<!-- There are a few kinds of statements in Rust. The most common two are declaring
a variable binding, and using a `;` with an expression: -->
宣言文にはいくつかの種類があります。最も一般的なのは変数の束縛(`variable binding`)と式文(`expression`)で、いずれも行末に`;`が付きます

```
fn main() {
    // 変数束縛
    let x = 5;

    // 式文
    x;
    x + 1;
    15;
}
```

<!-- Blocks are expressions too, so they can be used as [r-values][rvalue] in
assignments. The last expression in the block will be assigned to the
[l-value][lvalue]. However, if the last expression of the block ends with a
semicolon, the return value will be `()`. -->
コードブロックも文の一種です。よってブロックを丸ごと[右辺値][rvalue]として扱うことができます。その場合ブロック内の最後の式文が左辺値に代入されます。ただし、ブロック内の最後の式文が`;`で終わる場合は返り値は`()`になります。


{expression.play}

[rvalue]: https://en.wikipedia.org/wiki/Value_%28computer_science%29#lrvalue
[lvalue]: https://en.wikipedia.org/wiki/Value_%28computer_science%29#lrvalue
