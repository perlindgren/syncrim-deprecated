fn main() {
    let path = std::env::current_dir().unwrap();

    let res = rfd::FileDialog::new()
        .add_filter("text", &["txt", "rs"])
        .add_filter("rust", &["rs", "toml"])
        .set_directory(&path)
        .pick_files();

    println!("The user choose: {:#?}", res);
}
