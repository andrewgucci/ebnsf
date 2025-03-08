use std::path::PathBuf;

pub fn save_svg(name: &str, svg: &str) {
    let mut path = PathBuf::from("tests/images");
    std::fs::create_dir_all(&path).expect("create test svg dir");

    path.push(name);
    path.set_extension("svg");

    std::fs::write(path, svg).expect("write svg output");
}
