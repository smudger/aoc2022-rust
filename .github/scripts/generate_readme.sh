#!/bin/bash

cp .github/data/README.adoc README.adoc

# Replace `include` directive` with file contents, as GitHub does not support AsciiDoc `include` directives.
sed -i "" "s/include\:\:benchmark\.csv\[\]/$(sed -e 's/[\&/]/\\&/g' -e 's/$/\\n/' .github/data/benchmark.csv | tr -d '\n')/" README.adoc
