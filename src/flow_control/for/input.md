<!--- The `for in` construct can be used to iterate through an `Iterator`.  --->
<!--- One of the easiest ways to create an iterator is to use the range --->
<!--- notation `a..b`. This yields values from `a` (inclusive) to `b`  --->
<!--- (exclusive) in steps of one. --->
`for in`文を用いることで、イテレータ(`Iterator`)のそれぞれの要素に対して処理をすることが可能です。イテレータを作る最も単純な方法は`a..b`のような書き方をすることです。これは「`a`」から「`b`のひとつ前」までの要素を順に産出(`yield`)するというものです。

<!--- Let's write FizzBuzz using `for` instead of `while`. --->
では`for`と`while`を用いてFizzBuzzを書いてみましょう。

{for.play}

###See also

[イテレータ][iter]

[iter]: ../trait/iter.html
