pub fn create_test_file(filename: &str) -> hypermelon::tools::Adaptor<std::fs::File> {
    std::fs::create_dir_all("../target/assets/test/").unwrap();
    let file = std::fs::File::create(format!("../target/assets/test/{}", filename)).unwrap();
    hypermelon::tools::upgrade_write(file)
}
