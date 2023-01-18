<!--
# Partial moves
-->
# 部分的ムーブ

<!--
Within the [destructuring] of a single variable, both `by-move` and 
`by-reference` pattern bindings can be used at the same time. Doing 
this will result in a _partial move_ of the variable, which means 
that parts of the variable will be moved while other parts stay. In 
such a case, the parent variable cannot be used afterwards as a 
whole, however the parts that are only referenced (and not moved) 
can still be used.
-->
1つの変数の [デストラクト] の中で、 `ムーブ` と `参照` の両方のパターンバインディングを同時に使用することができます。両方を使用すると、変数の一部がムーブされ、他の部分が参照として残るという変数の部分的ムーブが発生した状態になります。変数の部分的ムーブが発生すると親変数はその後使用できませんが、参照されているだけの部分（ムーブされていない部分）は使用することができます。

[デストラクト]: ../../flow_control/match/destructuring.md

```rust,editable
fn main() {
    #[derive(Debug)]
    struct Person {
        name: String,
        age: Box<u8>,
    }

    let person = Person {
        name: String::from("Alice"),
        age: Box::new(20),
    };

    // `name` is moved out of person, but `age` is referenced
    // `name` は person からムーブしたが、 `age` は参照されている
    let Person { name, ref age } = person;

    println!("The person's age is {}", age);

    println!("The person's name is {}", name);

    // Error! borrow of partially moved value: `person` partial move occurs
    // エラー！部分的ムーブした値の借用：`person` では部分的ムーブが発生している
    //println!("The person struct is {:?}", person);

    // `person` cannot be used but `person.age` can be used as it is not moved
    // `person` は使用できないが、 `person.age` はムーブしていないので使用できる
    println!("The person's age from person struct is {}", person.age);
}
```
(In this example, we store the `age` variable on the heap to 
illustrate the partial move: deleting `ref` in the above code would 
give an error as the ownership of `person.age` would be moved to the 
variable `age`. If `Person.age` were stored on the stack, `ref` would 
not be required as the definition of `age` would copy the data from 
`person.age` without moving it.)

<!--
### See also:
-->
### 参照

<!--
[destructuring][destructuring]
-->
[デストラクト][destructuring]

[destructuring]: ../../flow_control/match/destructuring.md
