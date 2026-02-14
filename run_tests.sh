#!/bin/sh
echo building...
cargo build
echo -e "\nrunning scripts...\n"
for file in tests/*; do
    echo -e "\033[1m$file\033[0m"
    ./target/debug/semmel $file
    echo
done
echo "done."
