set -ex

mkdir assets -p

cargo run --example base_color > assets/base_color.svg
cargo run --example large_scatter > assets/large_scatter.svg
cargo run --example custom_colors_html > assets/custom_colors.html
cargo run --example custom_style > assets/custom_style.svg
cargo run --example custom_style_html > assets/custom_style.html
cargo run --example hover_shadow_html > assets/hover_shadow.html
cargo run --example magnitude > assets/magnitude.svg
cargo run --example trig > assets/trig.svg
cargo run --example test > assets/test.html
cargo run --example write_to_file
cargo run --example dark > assets/dark.svg
cargo run --example gaussian > assets/gaussian.svg
cargo run --example collatz > assets/collatz.svg
cargo run --example heart > assets/heart.svg
cargo run --example line_fill_fmt > assets/line_fill.svg
cargo run --example years_fmt > assets/years.svg
cargo run --example months > assets/months.svg
cargo run --example days > assets/days.svg
cargo run --example hours > assets/hours.svg
cargo run --example minutes > assets/minutes.svg
cargo run --example seconds > assets/seconds.svg
cd assets
python3 -m http.server
