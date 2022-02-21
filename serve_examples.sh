set -ex

mkdir target/assets -p

cargo run --example base_color > target/assets/base_color.svg
cargo run --example large_scatter > target/assets/large_scatter.svg
cargo run --example custom_colors_html > target/assets/custom_colors.html
cargo run --example custom_style > target/assets/custom_style.svg
cargo run --example custom_style_html > target/assets/custom_style.html
cargo run --example hover_shadow_html > target/assets/hover_shadow.html
cargo run --example magnitude > target/assets/magnitude.svg
cargo run --example trig > target/assets/trig.svg
cargo run --example test > target/assets/test.html
cargo run --example write_to_file
cargo run --example dark > target/assets/dark.svg
cargo run --example gaussian > target/assets/gaussian.svg
cargo run --example collatz > target/assets/collatz.svg
cargo run --example heart > target/assets/heart.svg
cargo run --example line_fill_fmt > target/assets/line_fill.svg
cargo run --example years > target/assets/years.svg
cargo run --example months > target/assets/months.svg
cargo run --example days > target/assets/days.svg
cargo run --example hours > target/assets/hours.svg
cargo run --example minutes_local_time > target/assets/minutes_local_time.svg
cargo run --example seconds > target/assets/seconds.svg
cargo run --example marathon > target/assets/marathon.svg
cargo run --example thread_needle > target/assets/thread_needle.svg
cargo run --example long_label > target/assets/long_label.svg
cargo run --example custom_dim > target/assets/custom_dim.svg

cd target/assets
python3 -m http.server
