#[allow(dead_code)]
pub fn public_function() {
    println!("called `my::inaccessible::public_function()`");
}

// 訳注: この関数は`my/mod.rs`中で`pub mod`されていないため、
// `split.rs`からは呼び出すことができない。
