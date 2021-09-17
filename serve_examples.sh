set -ex

mkdir assets -p
cargo run --example custom_color > assets/custom_color.svg
cargo run --example custom_colors_html > assets/custom_colors.html
cargo run --example custom_style > assets/custom_style.svg
cargo run --example custom_style_html > assets/custom_style.html
cargo run --example hover_shadow_html > assets/hover_shadow.html
cargo run --example magnitude > assets/magnitude.svg
cargo run --example trig > assets/trig.svg
cargo run --example simple > assets/simple.svg
cargo run --example test > assets/test.html
cargo run --example write_to_file
cargo run --example dark > assets/dark.svg
cargo run --example gaussian > assets/gaussian.svg
cargo run --example collatz > assets/collatz.svg
cargo run --example heart > assets/heart.svg

cd assets
python3 -m http.server
