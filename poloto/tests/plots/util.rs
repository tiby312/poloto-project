pub fn create_test_file(filename: &str) -> tagu::tools::Adaptor<std::fs::File> {
    std::fs::create_dir_all("../target/assets/test/").unwrap();
    let file = std::fs::File::create(format!("../target/assets/test/{}", filename)).unwrap();
    tagu::tools::upgrade_write(file)
}
