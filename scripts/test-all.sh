#!/bin/bash
set -e

for solution in day{01..09}{a,b}; do
    echo "Testing $solution..."
    (cd "$solution" && cargo test --quiet)
done

echo "All tests passed!"
