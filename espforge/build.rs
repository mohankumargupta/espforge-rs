use std::{
    env,
    fs::{self, File},
    io::Write,
    path::Path,
};

pub fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let examples_dir = Path::new("examples");

    // Collect subdirectories containing a config.rs
    let mut entries = Vec::new();

    if examples_dir.exists() {
        for entry in fs::read_dir(examples_dir).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.is_dir() && path.join("config.rs").exists() {
                if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                    entries.push(name.to_string());
                }
            }
        }
    }

    // Sort for stable builds
    entries.sort();

    // Generate import lines and macro invocation
    let mut output = String::new();
    for name in &entries {
        let alias = format!("{}Config", to_pascal_case(name));
        output.push_str(&format!(
            "use crate::examples::{name}::config::{alias};\n"
        ));
    }

    output.push_str("\n");
    output.push_str("generate_example_enum! {\n    ParsedExample {\n");

    for name in &entries {
        let alias = format!("{}Config", to_pascal_case(name));
        output.push_str(&format!("        {name} => {alias},\n"));
    }

    output.push_str("    }\n}\n");

    let out_path = Path::new(&out_dir).join("examples_generated.rs");
    File::create(&out_path)
        .unwrap()
        .write_all(output.as_bytes())
        .unwrap();

    println!("cargo:rerun-if-changed=examples");
}

// Helper: snake_case -> PascalCase
fn to_pascal_case(name: &str) -> String {
    let mut out = String::new();
    let mut uppercase = true;
    for c in name.chars() {
        if c == '_' {
            uppercase = true;
        } else if uppercase {
            out.push(c.to_ascii_uppercase());
            uppercase = false;
        } else {
            out.push(c);
        }
    }
    out
}

