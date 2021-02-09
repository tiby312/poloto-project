mkdir assets
cargo run --example simple > assets/simple.svg &&
cargo run --example custom_colors_html > assets/custom_colors_html.svg
cargo run --example custom_style_html > assets/custom_style_html.html
cargo run --example custom_style > assets/custom_style.html
cargo run --example hover_html > assets/hover_html.html