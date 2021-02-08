mkdir assets
cargo run --example simple > assets/simple.svg &&
cargo run --example custom_style > assets/custom_style.svg
cargo run --example custom_style_embed > assets/custom_style_embed.html
cargo run --example custom_style_embed2 > assets/custom_style_embed2.html