use std::fs;
use std::path::Path;

fn main() {
    let source_icon = Path::new("../assets/icons/dark/icon16x16.png");
    let target_icon = Path::new("icons/16x16.png");

    println!("cargo:rerun-if-changed={}", source_icon.display());

    if let Some(parent) = target_icon.parent() {
        fs::create_dir_all(parent).expect("failed to create src-tauri/icons directory");
    }

    fs::copy(source_icon, target_icon).expect("failed to copy dark build icon");

    tauri_build::build()
}
