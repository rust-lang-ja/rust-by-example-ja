<!-- Vectors are re-sizable arrays. Like slices, their size is not known at compile
time, but they can grow or shrink at any time. A vector is represented using
3 words: a pointer to the data, its length, and its capacity. The capacity
indicates how much memory is reserved for the vector. The vector can grow as
long as the length is smaller than the capacity. When this threshold needs to
be surpassed, the vector is reallocated with a larger capacity. -->
「ベクタ」はサイズを変更可能な配列です。スライスと同様、そのサイズはコンパイル時には不定ですが、いつでも要素を追加したり削除したりすることができます。ベクタは3つの要素で、その特徴が完全に決まります。

1. データへのポインタ
2. 長さ
3. 容量 ... あらかじめメモリ上にベクタのために確保された領域

ベクタはその容量を超えない限りにおいて長くしていくことができます。超えた場合には、より大きな容量を持つように割り当てなおされます。


{vec.play}

<!-- More `Vec` methods can be found under the
[std::vec][vec] module -->
`Vec`型のメソッドの一覧は[std::vec][vec]モジュールを見てください。

[vec]: http://doc.rust-lang.org/std/vec/
