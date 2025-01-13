// build.rs
use std::fs;
use std::path::Path;

fn main() {
    let strategies_dir = "src/strategies";
    let mod_rs_path = Path::new(&strategies_dir).join("mod.rs");

    // Scan the strategies directory for .rs files
    let mut module_lines = Vec::new();
    let mut registration_lines = Vec::new();
    for entry in fs::read_dir(strategies_dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_file() && path.extension().unwrap() == "rs" {
            let file_name = path.file_stem().unwrap().to_str().unwrap();
            // Exclude mod.rs and registry.rs from the module list
            if file_name != "mod" && file_name != "registry" {
                module_lines.push(format!("pub mod {};", file_name));
                registration_lines.push(format!(
                    "registry::register_strategy(Box::new({}::{}::new()));",
                    file_name, file_name
                ));
            }
        }
    }

    // Generate the module references
    let modules_content = module_lines.join("\n");

    // Generate the registration code
    let registration_content = registration_lines.join("\n    ");

    // Add the registry module and the register_strategies function
    let mod_rs_content = format!(
        r#"
pub mod registry;

{}

pub fn register_strategies() {{
    {}
}}
"#,
        modules_content, registration_content
    );

    // Write the updated content back to mod.rs
    fs::write(mod_rs_path, mod_rs_content).unwrap();
}