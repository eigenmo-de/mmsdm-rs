#![allow(unused)]
use std::env;

use anyhow::Context;
use log::info;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

mod analyse;
mod download;
mod html_tree;
mod json;
mod mms;
mod pdr;
mod rust;

pub const VERSION: &str = "5.7";

fn main() {
    if let Err(e) = run() {
        eprintln!("\n=====\nError running command:\n=====\n{e:#?}");
    }
}

#[tokio::main]
async fn run() -> Result<(), anyhow::Error> {
    env_logger::Builder::new()
        .filter_level(log::LevelFilter::Info)
        .init();

    let file_name = format!("mmsdm_v{VERSION}.json");
    match env::args().nth(1).as_deref() {
        Some("rust") => {
            let mut data = String::new();
            let mut f = tokio::fs::OpenOptions::new()
                .read(true)
                .open(&file_name)
                .await
                .with_context(|| format!("Opening file: `{file_name}`"))?;

            f.read_to_string(&mut data).await?;
            let de = serde_json::from_str(&data)?;

            info!("Generating rust code");
            rust::run(de)?;
        }
        Some("analyse") => {
            let mut data = String::new();
            let mut f = tokio::fs::OpenOptions::new()
                .read(true)
                .open(&file_name)
                .await
                .with_context(|| format!("Opening file: `{file_name}`"))?;

            f.read_to_string(&mut data).await?;
            let de = serde_json::from_str(&data)?;

            analyse::run(de)?;
        }
        Some("json") => {
            let dm = json::run().await.context("generate json")?;

            let ser = serde_json::to_string_pretty(&dm)?;
            let mut f = tokio::fs::OpenOptions::new()
                .create(true)
                .truncate(true)
                .write(true)
                .open(&file_name)
                .await
                .with_context(|| format!("Opening file: `{file_name}`"))?;

            f.write_all(ser.as_bytes()).await?;

            f.flush().await?;
        }
        Some("download") => {
            download::run().await?;
        }
        _ => unimplemented!("not yet"),
    }
    Ok(())
}

const KW: [&str; 51] = [
    "as", "break", "const", "continue", "crate", "else", "enum", "extern", "false", "fn", "for",
    "if", "impl", "in", "let", "loop", "match", "mod", "move", "mut", "pub", "ref", "return",
    "self", "Self", "static", "struct", "super", "trait", "true", "type", "unsafe", "use", "where",
    "while", "async", "await", "dyn", "abstract", "become", "box", "do", "final", "macro",
    "override", "priv", "typeof", "unsized", "virtual", "yield", "try",
];
