use csv::{ReaderBuilder, StringRecord, WriterBuilder};
use serde::{Deserialize, Serialize};
use std::{env, error::Error, path::Path};

#[derive(Debug, Serialize, Deserialize)]
struct SdlMapping {
    guid: String,
    name: String,
    platform: String,
    mapping: String,
    source: String,
    verification: String,
}

fn main() {
    if let Err(error) = run() {
        eprintln!("error: {error}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    match args.get(1).map(String::as_str) {
        Some("validate") if args.len() == 3 => validate_dir(Path::new(&args[2])),
        Some("import-sdl") if args.len() == 4 => {
            let count = import_sdl(Path::new(&args[2]), Path::new(&args[3]))?;
            println!("Imported {count} upstream SDL mapping rows");
            Ok(())
        }
        _ => {
            eprintln!("ControllerParity spreadsheet tools\n\nUsage:\n  controller-parity validate <data-dir>\n  controller-parity import-sdl <gamecontrollerdb.txt> <sdl_mappings.csv>");
            std::process::exit(2);
        }
    }
}

fn import_sdl(input: &Path, output: &Path) -> Result<usize, Box<dyn Error>> {
    let text = std::fs::read_to_string(input)?;
    let mut writer = WriterBuilder::new().from_path(output)?;
    let mut count = 0;
    for line in text
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty() && !line.starts_with('#'))
    {
        let Some((guid, rest)) = line.split_once(',') else {
            continue;
        };
        let Some((name, bindings)) = rest.split_once(',') else {
            continue;
        };
        if guid.eq_ignore_ascii_case("guid") {
            continue;
        }
        if guid.len() != 32 || !guid.chars().all(|c| c.is_ascii_hexdigit()) {
            continue;
        }
        let mut platform = String::new();
        let mut parts = Vec::new();
        for field in bindings.split(',').filter(|field| !field.is_empty()) {
            if let Some(value) = field.strip_prefix("platform:") {
                platform = value.to_owned();
            } else {
                parts.push(field);
            }
        }
        writer.serialize(SdlMapping {
            guid: guid.to_ascii_lowercase(),
            name: name.to_owned(),
            platform,
            mapping: parts.join(","),
            source: "mdqinc/SDL_GameControllerDB (imported; see adjacent license)".into(),
            verification: "reported_upstream".into(),
        })?;
        count += 1;
    }
    writer.flush()?;
    Ok(count)
}

fn validate_dir(dir: &Path) -> Result<(), Box<dyn Error>> {
    let mut total = 0usize;
    for file in [
        "controllers.csv",
        "controls.csv",
        "games.csv",
        "game_bindings.csv",
    ] {
        let path = dir.join(file);
        let mut reader = ReaderBuilder::new().flexible(false).from_path(&path)?;
        let headers = reader.headers()?.clone();
        if headers.is_empty() {
            return Err(format!("{} has no header", path.display()).into());
        }
        for (index, row) in reader.records().enumerate() {
            let row = row?;
            if row.len() != headers.len() {
                return Err(format!(
                    "{} row {} has wrong column count",
                    path.display(),
                    index + 2
                )
                .into());
            }
            total += 1;
        }
    }
    let sdl = dir.join("sdl_mappings.csv");
    if sdl.exists() {
        let mut reader = ReaderBuilder::new().from_path(&sdl)?;
        let headers = reader.headers()?.clone();
        require_headers(
            &headers,
            &[
                "guid",
                "name",
                "platform",
                "mapping",
                "source",
                "verification",
            ],
        )?;
        let mut ids = std::collections::HashSet::new();
        for (index, row) in reader.deserialize::<SdlMapping>().enumerate() {
            let row = row?;
            if row.guid.len() != 32 || !row.guid.chars().all(|c| c.is_ascii_hexdigit()) {
                return Err(
                    format!("{} row {} has invalid SDL GUID", sdl.display(), index + 2).into(),
                );
            }
            let key = format!("{}:{}", row.guid.to_ascii_lowercase(), row.platform);
            if !ids.insert(key.clone()) {
                return Err(format!("duplicate SDL mapping {key}").into());
            }
            if row.name.trim().is_empty()
                || row.mapping.trim().is_empty()
                || row.source.trim().is_empty()
            {
                return Err(format!(
                    "{} row {} has required fields empty",
                    sdl.display(),
                    index + 2
                )
                .into());
            }
            total += 1;
        }
    }
    println!("Validated CSV structure and SDL mapping integrity ({total} data rows); custom raw-control profiles are not yet loaded by the runtime.");
    Ok(())
}

fn require_headers(headers: &StringRecord, expected: &[&str]) -> Result<(), Box<dyn Error>> {
    for name in expected {
        if !headers.iter().any(|actual| actual == *name) {
            return Err(format!("missing required CSV column: {name}").into());
        }
    }
    Ok(())
}
