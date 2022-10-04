set -ex

cargo test --release --features timestamp_full
cargo run --release --example collatz > target/assets/collatz.svg
cargo run --release --example gaussian > target/assets/gaussian.svg
cargo run --release --example custom_ticks > target/assets/custom_ticks.svg
cargo run --release  --example timestamp > target/assets/timestamp.svg
cargo run --release --example bar > target/assets/bar.svg
cargo run --release --example hello_world > target/assets/hello_world.svg

cargo bloat  --test plots --features timestamp_full
cd target/assets
python3 -m http.server
