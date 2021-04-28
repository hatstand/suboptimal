use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Serialize, Deserialize)]
struct OptimizelyFile {
    version: String,
}

fn main() {
    let data = r#"
    {
        "version": "42"
    }"#;

    let p: Result<OptimizelyFile> = serde_json::from_str(data);
    match p {
        Ok(f) => println!("{}", f.version),
        Err(e) => println!(":-( {}", e),
    }
}
