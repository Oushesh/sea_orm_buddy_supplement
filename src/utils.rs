use std::fs;
use std::path::PathBuf;

pub fn get_output_path(filename: &str) -> String {
    // 1. Get the root directory of your project
    let root = env!("CARGO_MANIFEST_DIR");

    // 2. Build the path to a "plots" folder
    let mut path = PathBuf::from(root);
    path.push("plots"); // folder name

    // 3. Ensure the folder exists (os.makedirs(exist_ok=True))
    fs::create_dir_all(&path).unwrap();

    // 4. Add the filename
    path.push(filename);

    // 5. Convert to string for your visualizer
    path.to_str().unwrap().to_string()
}