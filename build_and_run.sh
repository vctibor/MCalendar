cd client
rm -rf dist
cargo make build_release
mkdir dist
cp index.html dist/
cp style.css dist/
cp -r pkg dist/
cd ..

cd server
cargo build
cargo run