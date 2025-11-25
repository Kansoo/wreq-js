use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let emulation_file = locate_wreq_util_emulation_file();
    let emulation_source =
        fs::read_to_string(&emulation_file).expect("Failed to read wreq-util emulation/mod.rs");

    // Dynamically extract all browser profiles from wreq-util by reading the source
    let profiles = extract_profiles_from_source(&emulation_source);
    let operating_systems = extract_operating_systems_from_source(&emulation_source);

    println!("cargo:warning=Found {} browser profiles", profiles.len());
    println!(
        "cargo:warning=Found {} operating systems",
        operating_systems.len()
    );

    // Generate TypeScript type definition
    let ts_type = generate_typescript_types(&profiles, &operating_systems);

    // Generate Rust profiles array
    let rust_profiles = generate_rust_profiles(&profiles, &operating_systems);

    // Write to src directory (going up one level from rust/)
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    // Write TypeScript types
    let ts_dest = Path::new(&manifest_dir)
        .parent()
        .unwrap()
        .join("src")
        .join("generated-types.ts");
    fs::write(&ts_dest, ts_type).unwrap();

    // Write Rust profiles array
    let rust_dest = Path::new(&manifest_dir)
        .join("src")
        .join("generated_profiles.rs");
    fs::write(&rust_dest, rust_profiles).unwrap();

    println!("cargo:rerun-if-changed=build.rs");
}

fn generate_typescript_types(profiles: &[String], operating_systems: &[String]) -> String {
    let mut ts_content = String::from(
        "/**\n * Auto-generated from Rust build script\n * DO NOT EDIT MANUALLY\n */\n\n",
    );

    ts_content.push_str("/**\n * Browser profile names supported\n */\n");
    ts_content.push_str("export type BrowserProfile =\n");

    for (i, profile) in profiles.iter().enumerate() {
        if i == profiles.len() - 1 {
            // Last profile - put semicolon on same line
            ts_content.push_str(&format!("  | '{}';\n", profile));
        } else {
            ts_content.push_str(&format!("  | '{}'\n", profile));
        }
    }

    ts_content.push_str("\n/**\n * Operating systems supported for emulation\n */\n");
    ts_content.push_str("export type EmulationOS =\n");

    for (i, os) in operating_systems.iter().enumerate() {
        if i == operating_systems.len() - 1 {
            ts_content.push_str(&format!("  | '{}';\n", os));
        } else {
            ts_content.push_str(&format!("  | '{}'\n", os));
        }
    }

    ts_content
}

fn generate_rust_profiles(profiles: &[String], operating_systems: &[String]) -> String {
    let mut rust_content =
        String::from("// Auto-generated from build script\n// DO NOT EDIT MANUALLY\n\n");

    rust_content.push_str("pub const BROWSER_PROFILES: &[&str] = &[\n");

    for profile in profiles {
        rust_content.push_str(&format!("    \"{}\",\n", profile));
    }

    rust_content.push_str("];\n");

    rust_content.push_str("\npub const OPERATING_SYSTEMS: &[&str] = &[\n");

    for os in operating_systems {
        rust_content.push_str(&format!("    \"{}\",\n", os));
    }

    rust_content.push_str("];\n");

    rust_content
}

fn locate_wreq_util_emulation_file() -> std::path::PathBuf {
    let metadata = std::process::Command::new("cargo")
        .args(["metadata", "--format-version", "1"])
        .output()
        .expect("Failed to run cargo metadata");

    let metadata_str = String::from_utf8(metadata.stdout).expect("Failed to parse cargo metadata");

    // Parse JSON to find wreq-util package
    let metadata_json: serde_json::Value =
        serde_json::from_str(&metadata_str).expect("Failed to parse metadata JSON");

    let packages = metadata_json["packages"]
        .as_array()
        .expect("No packages in metadata");

    let wreq_util_pkg = packages
        .iter()
        .find(|p| p["name"].as_str() == Some("wreq-util"))
        .expect("wreq-util package not found");

    let manifest_path = wreq_util_pkg["manifest_path"]
        .as_str()
        .expect("No manifest_path for wreq-util");

    Path::new(manifest_path)
        .parent()
        .expect("Failed to get wreq-util directory")
        .join("src")
        .join("emulation")
        .join("mod.rs")
}

fn extract_profiles_from_source(content: &str) -> Vec<String> {
    // Extract serde rename values from the file
    // Look for patterns like: => ("profile_name", ...)
    let mut profiles = Vec::new();

    for line in content.lines() {
        // Match lines like: Chrome100 => ("chrome_100", v100::emulation),
        if let Some(start) = line.find("=> (\"")
            && let Some(end) = line[start + 5..].find('"')
        {
            let profile = &line[start + 5..start + 5 + end];
            profiles.push(profile.to_string());
        }
    }

    if profiles.is_empty() {
        panic!("No profiles found in wreq-util source!");
    }

    profiles
}

fn extract_operating_systems_from_source(content: &str) -> Vec<String> {
    let mut operating_systems = Vec::new();
    let mut in_os_block = false;
    let mut saw_plain_marker = false;

    for line in content.lines() {
        if !in_os_block {
            if line.contains("plain,") {
                saw_plain_marker = true;
                continue;
            }

            if saw_plain_marker && line.contains("EmulationOS") {
                in_os_block = true;
                continue;
            }

            continue;
        }

        if in_os_block {
            if line.contains(");") {
                break;
            }

            if let Some(start) = line.find("=> \"")
                && let Some(end) = line[start + 4..].find('"')
            {
                let os = &line[start + 4..start + 4 + end];
                operating_systems.push(os.to_string());
            }
        }
    }

    if operating_systems.is_empty() {
        panic!("No operating systems found in wreq-util source!");
    }

    operating_systems
}
