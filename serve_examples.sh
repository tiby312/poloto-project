set -ex

cargo test --release --features timestamp,chrono/std
cargo run --release --example collatz > target/assets/collatz.svg
cargo run --release --example gaussian > target/assets/gaussian.svg
cargo run --release --example custom_ticks > target/assets/custom_ticks.svg
cargo run --release  --example timestamp > target/assets/timestamp.svg
cargo run --release --example bar > target/assets/bar.svg
cargo bloat  --test plots --features timestamp,chrono/std
cd target/assets
python3 -m http.server
