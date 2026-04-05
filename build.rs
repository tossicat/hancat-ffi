use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=cbindgen.toml");

    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let output_path = PathBuf::from(&crate_dir).join("include").join("hancat.h");

    // include 디렉토리가 없으면 생성
    std::fs::create_dir_all(PathBuf::from(&crate_dir).join("include")).unwrap();

    let config = cbindgen::Config::from_file("cbindgen.toml").unwrap_or_default();

    cbindgen::Builder::new()
        .with_crate(crate_dir)
        .with_config(config)
        .generate()
        .expect("cbindgen으로 헤더 파일을 생성할 수 없습니다.")
        .write_to_file(output_path);
}
