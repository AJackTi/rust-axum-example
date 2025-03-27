use std::env;
use std::fs::{ self, File };
use std::io::{ Write, Read };
use std::path::{ Path, PathBuf };
use std::borrow::Cow;
use zip::read::ZipArchive;
use zip::result::ZipError;

const SWAGGER_UI_VERSION: &str = "5.1.0";
const SWAGGER_DIST_URL: &str =
    "https://github.com/swagger-api/swagger-ui/archive/refs/tags/v5.1.0.zip";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out_dir = env::var("OUT_DIR")?;
    let dest_path = Path::new(&out_dir).join("swagger-ui");

    if !dest_path.exists() {
        fs::create_dir_all(&dest_path)?;

        // Download swagger UI zip
        let response = reqwest::blocking::get(SWAGGER_DIST_URL)?;
        let zip_content = response.bytes()?;

        // Create temporary zip file
        let temp_zip_path = Path::new(&out_dir).join("swagger-ui.zip");
        let mut temp_zip_file = File::create(&temp_zip_path)?;
        temp_zip_file.write_all(&zip_content)?;

        // Open zip archive
        let mut archive = ZipArchive::new(File::open(&temp_zip_path)?)?;

        // Generate Rust file to include Swagger UI files
        let swagger_ui_rs_path = Path::new(&out_dir).join("swagger_ui_files.rs");
        let mut swagger_ui_rs = File::create(&swagger_ui_rs_path)?;

        // Write initial content to Rust file
        writeln!(swagger_ui_rs, "// Auto-generated Swagger UI files\n")?;
        writeln!(swagger_ui_rs, "const SWAGGER_UI_FILES: &[(&str, &str)] = &[")?;

        // Extract and process files
        for i in 0..archive.len() {
            let mut file = archive.by_index(i)?;
            let file_path_str = file
                .name()
                .strip_prefix(&format!("swagger-ui-{SWAGGER_UI_VERSION}/dist/"))
                .ok_or(ZipError::InvalidArchive(Cow::Borrowed("invalid path file")))?;

            // Only extract specific files we want
            let allowed_files = [
                "index.html",
                "swagger-ui.css",
                "swagger-ui-bundle.js",
                "swagger-ui-standalone-preset.js",
            ];

            if allowed_files.contains(&file_path_str) {
                let output_path = dest_path.join(file_path_str);

                if file.is_file() {
                    // Read file contents
                    let mut contents = String::new();
                    file.read_to_string(&mut contents)?;

                    // Write file path and contents to Rust file
                    writeln!(swagger_ui_rs, "    (\"{}\", r#\"{}\"#),", file_path_str, contents)?;

                    // Write actual file to destination
                    let mut output_file = File::create(&output_path)?;
                    output_file.write_all(contents.as_bytes())?;
                }
            }
        }

        // Close the array
        writeln!(swagger_ui_rs, "];")?;

        // Clean up temporary zip file
        fs::remove_file(temp_zip_path)?;
    }

    // Tell cargo to rerun this script if dependencies change
    println!("cargo:rerun-if-changed=build.rs");

    Ok(())
}
