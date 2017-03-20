<!-- Data can be immutably borrowed any number of times, but while immutably
borrowed, the original data can't be mutably borrowed. On the other hand,
only *one* mutable borrow is allowed at a time. The original data can be
borrowed again only *after* the mutable reference goes out of scope. -->
データは一度にいくつでもイミュータブルに借用することができますが、その間オリジナルのデータをミュータブルに借用することはできません。一方でミュータブルな借用は一度に*一つ*しか借用することができず、オリジナルのデータをもう一度借用するためには、ミュータブルな参照がスコープを抜けた*あとで*ないといけません。

{alias.play}
