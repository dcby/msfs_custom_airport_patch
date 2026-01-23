use crate::args::Args;
use anyhow::{Context, Result, anyhow};
use clap::Parser;
use serde_json::{Value, json};
use std::path::Path;

mod args;

const BAD_JSON: &str = "Bad JSON";
const KEY: &str = "package_order_hint";

fn main() -> Result<()> {
    let args = Args::parse();

    do_path(&args.path)?;

    for path in &args.next_path {
        do_path(&path)?;
    }

    Ok(())
}

fn do_path<P: AsRef<Path>>(path: P) -> Result<()> {
    fn inner(path: &Path) -> Result<()> {
        return if path.is_dir() {
            do_dir(path)
        } else {
            do_file(path)
        };
    }
    inner(path.as_ref())
}

fn do_dir(path: &Path) -> Result<()> {
    let mut p = path.to_path_buf();
    p.push("manifest.json");
    do_file(p.as_path())
}

fn do_file(path: &Path) -> Result<()> {
    let data = std::fs::read_to_string(path)
        .with_context(|| format!("Failed to read the file `{}`", path.to_string_lossy()))?;
    let mut doc: Value = serde_json::from_str(data.as_str()).with_context(|| BAD_JSON)?;
    let root = match doc.as_object_mut() {
        Some(x) => x,
        None => return Err(anyhow!(BAD_JSON)),
    };

    if let Some(v) = root.get("content_type")
        && v == "SCENERY"
    {
        if !root.contains_key(KEY) {
            root.insert(KEY.to_string(), json!("CUSTOM_AIRPORT_PATCH"));
            let data = serde_json::to_string_pretty(&doc)?;
            std::fs::copy(path, path.with_added_extension("bak"))?;
            std::fs::write(path, data)?;
        }
    }

    Ok(())
}
