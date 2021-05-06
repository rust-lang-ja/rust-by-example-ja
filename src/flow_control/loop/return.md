<!--
# Returning from loops
-->
# loopが返す値

<!--
One of the uses of a `loop` is to retry an operation until it succeeds. If the
operation returns a value though, you might need to pass it to the rest of the
code: put it after the `break`, and it will be returned by the `loop`
expression.
-->
`loop`の用途のひとつに「成功するまである処理を再試行する」ことがあります。もしその処理が値を返すならば、それをコードの他の部分に渡す必要があるでしょう。`break`の後に値を置くと、それが`loop`式の値として返されます。

```rust,editable
fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    assert_eq!(result, 20);
}
```
