set -ex

cargo test --release
cargo run --release --example collatz > target/assets/collatz.svg
cargo run --release --example gaussian > target/assets/gaussian.svg
cargo run --release --example custom_ticks > target/assets/custom_ticks.svg
cargo run --release --example timestamp > target/assets/timestamp.svg
cargo run --release --example bar > target/assets/bar.svg

cd target/assets
python3 -m http.server
