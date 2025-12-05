use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("days.rs");
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    let day_dir = Path::new("src/day");

    let mut days = Vec::new();

    if let Ok(entries) = fs::read_dir(day_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if let Some(filename) = path.file_name().and_then(|n| n.to_str()) {
                if filename.starts_with("day_") && filename.ends_with(".rs") {
                    let day_name = filename.strip_suffix(".rs").unwrap();
                    days.push(day_name.to_string());
                }
            }
        }
    }

    days.sort();

    let mut generated = String::new();

    for day in &days {
        let day_path = format!("{}/src/day/{}.rs", manifest_dir, day);
        generated.push_str(&format!("#[path = \"{}\"]\n", day_path));
        generated.push_str(&format!("mod {};\n", day));
    }

    generated.push_str("\npub fn get_days() -> Vec<Box<dyn Day>> {\n    vec![\n");

    for day in &days {
        generated.push_str(&format!("        {}::day(),\n", day));
    }

    generated.push_str("    ]\n}\n");

    fs::write(dest_path, generated).unwrap();

    println!("cargo:rerun-if-changed=src/day");
}
