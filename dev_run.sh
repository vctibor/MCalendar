#!/usr/bin/env sh 

echo $DATABASE_URL

cd shared
cargo build
cd ..

cd client
rm -rf dist
cargo make build_release
mkdir dist
cp index.html dist/
cp style.css dist/
cp favicon.svg dist/
cp -r pkg dist/
cd ..

cd server
cargo build
cargo run