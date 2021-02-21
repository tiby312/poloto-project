mkdir assets
cargo run --example simple > assets/simple.svg &&
cargo run --example custom_colors_html > assets/custom_colors.html
cargo run --example custom_style_html > assets/custom_style.html
cargo run --example custom_style > assets/custom_style.svg
cargo run --example hover_shadow_html > assets/hover_shadow.html
cargo run --example no_legend > assets/no_legend.svg