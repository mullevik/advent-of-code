#!/bin/bash

[ $# -ne 1 ] && printf "Provide a day name (eg. '02')\n" && exit 2

set -e

cp ./src/template.rs "./src/.tmp.rs"

sed s/DAY/${1}/g "./src/.tmp.rs" > "./src/day_${1}.rs"

rm ./src/.tmp.rs

printf "Created ./src/day_${1}.rs\n"

