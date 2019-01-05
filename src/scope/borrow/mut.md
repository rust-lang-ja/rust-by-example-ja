<!-- Mutable data can be mutably borrowed using `&mut T`. This is called
a *mutable reference* and gives read/write access to the borrower.
In contrast, `&T` borrows the data via an immutable reference, and
the borrower can read the data but not modify it: -->
ミュータブルなデータは`&mut T`でミュータブルに(変更可能な形で)借用することができます。これは*ミュータブルな参照*と呼ばれ、読み込み・書き込みの権限を借用者に与えます。対照的に`&T`はデータをイミュータブルな参照を介して借用し、借用した側はデータを読み込みはできますが書き込みはできません。

``` rust,editable,ignore,mdbook-runnable
#[allow(dead_code)]
#[derive(Clone, Copy)]
struct Book {
    // `&'static str`はread-onlyメモリ上の文字列への参照
    author: &'static str,
    title: &'static str,
    year: u32,
}

// この関数はBook型へのリファレンスを取る。
fn borrow_book(book: &Book) {
    println!("I immutably borrowed {} - {} edition", book.title, book.year);
}

// この関数はミュータブルなBook型へのミュータブルなリファレンスを取り、
// `year`を2014へ変更する。
fn new_edition(book: &mut Book) {
    book.year = 2014;
    println!("I mutably borrowed {} - {} edition", book.title, book.year);
}

fn main() {
    // `immutabook`という名のイミュータブルなBookを作成
    let immutabook = Book {
        // 「"」で囲まれた部分は`&'static str`型になる。
        author: "Douglas Hofstadter",
        title: "Gödel, Escher, Bach",
        year: 1979,
    };

    // `immutabook`のミュータブルなコピーを作成し、`mutabook`と名付ける
    let mut mutabook = immutabook;

    // イミュータブルなオブジェクトをイミュータブルに借用する
    borrow_book(&immutabook);

    // ミュータブルなオブジェクトをイミュータブルに借用する
    borrow_book(&mutabook);

    // ミュータブルなオブジェクトをミュータブルに借用する
    new_edition(&mut mutabook);

    // エラー！イミュータブルなオブジェクトをミュータブルに借用することはできない
    new_edition(&mut immutabook);
    // FIXME ^ この行をコメントアウトしましょう
}

```

### See Also
[`static`][static]

[static]: scope/lifetime/static_lifetime.html
