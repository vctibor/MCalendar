#!/usr/bin/env sh 

echo $DATABASE_URL

echo "BUILDING SHARED"
cd shared
cargo build
cd ..

echo "BUILDING CLIENT"
cd client
rm -rf dist
cargo make build_release
mkdir dist
cp index.html dist/
cp style.css dist/
cp favicon.svg dist/
cp -r pkg dist/
cd ..

echo "BUILDING SERVER"
cd server
cargo build
cargo run