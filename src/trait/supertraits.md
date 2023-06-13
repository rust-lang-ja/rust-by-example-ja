<!--
# Supertraits
-->
# スーパートレイト

<!--
Rust doesn't have "inheritance", but you can define a trait as being a superset
of another trait. For example:
-->
Rustには"継承"はありませんが、あるトレイトを別のトレイトの上位集合として定義できます。
例えば：

```rust,editable
trait Person {
    fn name(&self) -> String;
}

// Person is a supertrait of Student.
// Implementing Student requires you to also impl Person.
// PersonはStudentの上位集合です。
// Studentを実装するにはPersonも実装する必要があります。
trait Student: Person {
    fn university(&self) -> String;
}

trait Programmer {
    fn fav_language(&self) -> String;
}

// CompSciStudent (computer science student) is a subtrait of both Programmer 
// and Student. Implementing CompSciStudent requires you to impl both supertraits.
// CompSciStudent（コンピュータサイエンスの学生）はProgrammerとStudent両方のサブトレイトです。
// CompSciStudentを実装するには、両方のスーパートレイトを実装する必要があります。
trait CompSciStudent: Programmer + Student {
    fn git_username(&self) -> String;
}

fn comp_sci_student_greeting(student: &dyn CompSciStudent) -> String {
    format!(
        "My name is {} and I attend {}. My favorite language is {}. My Git username is {}",
        student.name(),
        student.university(),
        student.fav_language(),
        student.git_username()
    )
}

fn main() {}
```

<!--
### See also:
-->
### 参照

[The Rust Programming Language chapter on supertraits][trpl_supertraits]

[trpl_supertraits]: https://doc.rust-lang.org/book/ch19-03-advanced-traits.html#using-supertraits-to-require-one-traits-functionality-within-another-trait
