<!-- Most of the time, we'd like to access data without taking ownership over
it. To accomplish this, Rust uses a *borrowing* mechanism. Instead of
passing objects by-value (`T`), objects can be passed by reference (`&T`).  -->
実際には、データの所有権を完全に受け渡すことなく一時的にアクセスしたいという場合がほとんどです。そのために、Rustでは*借用(`borrowing`)*という仕組みを用います。値そのもの(`T`)を受け渡すのではなく、そのリファレンス(`&T`)を渡すのです。

<!-- The compiler statically guarantees (via its borrow checker) that references  -->
<!-- *always* point to valid objects. That is, while references to an object
exist, the object cannot be destroyed. -->
コンパイラは借用チェッカを用いてリファレンスが*常に*有効なオブジェクトへの参照であることを、コンパイル時に保証します。つまり、あるオブジェクトへのリファレンスが存在しているならば、そのオブジェクトを破壊することはできないということです。

{borrow.play}
