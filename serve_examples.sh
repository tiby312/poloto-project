set -ex

cargo test
cargo run --example collatz > target/assets/collatz.svg
cargo run --example gaussian > target/assets/gaussian.svg
cargo run --example custom_ticks > target/assets/custom_ticks.svg
cargo run --example timestamp > target/assets/timestamp.svg
cargo run --example bar > target/assets/bar.svg

cd target/assets
python3 -m http.server
