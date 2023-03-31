set -ex


mkdir -p target/assets
cargo run --release --example collatz > target/assets/collatz.svg
cargo run --release --example gaussian > target/assets/gaussian.svg
cargo run --release --example custom_ticks > target/assets/custom_ticks.svg
cargo run --release --example custom_tick_fmt > target/assets/custom_tick_fmt.svg
cargo run --release --example bar > target/assets/bar.svg
cargo run --release --example hello_world > target/assets/hello_world.svg
cargo run --release --example styling > target/assets/styling.svg
cargo run --release --example styling > target/assets/styling.svg

cargo run --release --example timestamp > target/assets/timestamp.svg

cargo test --release

resvg -w 1200 target/assets/collatz.svg target/assets/collatz.png



cargo bloat  --test plots
cd target/assets
python3 -m http.server
