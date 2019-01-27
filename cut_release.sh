#!/bin/bash

set -eu

cd $(dirname $0)

rm -rf Release
mkdir Release

cargo build --release

cp target/release/ggj19 Release/dear_hunter
cp -r resources Release/resources

# Delete Krita files
find Release/resources -name "*.kra" -delete


tar czf dear_hunter_$(uname).tar.gz Release
