use std::io::Write;

fn main() {
    println!("cargo:rerun-if-changed=Cargo.toml");

    let cargo_version = std::env::var("CARGO_PKG_VERSION").unwrap();
    let mut config = std::fs::File::create("src/config.rs").unwrap();
    config
        .write_all(
            format!(
                "#[allow(dead_code)]\n\
                 pub const CARGO_VERSION: &'static str = \"{}\";\n\
                 #[allow(dead_code)]\n\
                 pub const UI_FRAMEWORK: &'static str = \"dioxus\";\n",
                cargo_version
            )
            .as_bytes(),
        )
        .unwrap();

    println!("cargo:rerun-if-changed=../../SourceHanSansCN-Normal.otf");

    println!("cargo:warning=build.rs: skipping font copy (fonts bundled separately)");
}
