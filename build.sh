rm -rf output
mkdir -p output
cargo build --release
cp ./target/release/zkcli ./output/zkcli