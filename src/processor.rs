use std::{
    fs,
    path::Path,
};
use webx_api::{parse_file, Prose};

fn main() {
    let prose_dir = Path::new("src/prose");
    let mut entries: Vec<Prose> = Vec::new();

    for entry in fs::read_dir(prose_dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();

        if path.extension().unwrap_or_default() == "org" {
            let prose = parse_file(&path); // Your existing conversion
            if let Ok(prose) = prose {
                entries.push(prose);
            }
        }
    }

    // Write to dist/prose.json
    let json = serde_json::to_string(&entries).unwrap();
    fs::write("src/prose.json", json).unwrap();
}
