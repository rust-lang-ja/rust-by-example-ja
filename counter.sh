#!/bin/bash
echo "counting total line number foreach chapters"
echo "output file is count_table"
echo "" > count_table

total_count=0
for example in `find examples -type d -d 1`; do
    total_count=$((`find $example -name "input.md" -o -name "*.rs"  | xargs cat | wc -l` + $total_count))
done

for example in `find examples -type d -d 1`; do
    printf  "%s\t" "$example" >> count_table
    count=$(find $example -name "input.md" -o -name "*.rs"  | xargs cat | wc -l)
    printf "%s\t" "$count" >> count_table
    echo "scale=3; $count / $total_count * 100" | bc >> count_table
done

