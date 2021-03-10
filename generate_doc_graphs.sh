mkdir assets
cargo run --example custom_color > assets/custom_color.svg &&
cargo run --example custom_colors_html > assets/custom_colors.html &&
cargo run --example custom_style > assets/custom_style.svg &&
cargo run --example hover_shadow_html > assets/hover_shadow.html &&
cargo run --example magnitude > assets/magnitude.svg &&
cargo run --example simple > assets/simple.svg &&
cargo run --example test > assets/test.html
cargo run --example write_to_file
cargo run --example file_iter > assets/file_iter.svg
cargo run --example from_scratch > assets/from_scratch.svg


