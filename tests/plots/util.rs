pub fn create_test_file(filename: &str) -> tagger::Adaptor<std::fs::File> {
    std::fs::create_dir_all("target/assets").unwrap();
    let file = std::fs::File::create(format!("target/assets/{}", filename)).unwrap();
    tagger::upgrade_write(file)
}
