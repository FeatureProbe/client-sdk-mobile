// build.rs

use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("target_os.rs");
    let mut f = File::create(dest_path).unwrap();
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap_or_else(|_| "uniffi".to_owned());

    let s = format!(
        "pub fn target_os() -> String {{
            \"{}\".to_owned()
        }}",
        target_os
    );

    f.write_all(&s.into_bytes()).unwrap();
}
