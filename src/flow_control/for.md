<!--- The `for in` construct can be used to iterate through an `Iterator`.  --->
<!--- One of the easiest ways to create an iterator is to use the range --->
<!--- notation `a..b`. This yields values from `a` (inclusive) to `b`  --->
<!--- (exclusive) in steps of one. --->
`for in`文を用いることで、イテレータ(`Iterator`)のそれぞれの要素に対して処理をすることが可能です。イテレータを作る最も単純な方法は`a..b`のような書き方をすることです。これは「`a`」から「`b`のひとつ前」までの要素を順に産出(`yield`)するというものです。

<!--- Let's write FizzBuzz using `for` instead of `while`. --->
では`for`と`while`を用いてFizzBuzzを書いてみましょう。

``` rust,editable
fn main() {
    // `n`は1, 2, ...., 100のそれぞれの値を取ります。
    for n in 1..101 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
    }
}

```

###See also

[イテレータ][iter]

[iter]: ../trait/iter.html
