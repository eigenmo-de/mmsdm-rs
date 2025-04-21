use anyhow::Context;
use log::info;
use structopt::StructOpt;
use tokio::io::{AsyncReadExt, AsyncSeekExt, AsyncWriteExt};

mod analyse;
mod download;
mod html_tree;
mod json;
mod mms;
mod pdr;
mod python;
mod rust;
mod sql_server_tables;

pub const VERSION: &str = "5.4";

#[derive(structopt::StructOpt)]
#[structopt(about = "Code generation on the MMS Data Model")]
enum AemoCodegen {
    Json,
    Rust,
    SqlServerTables,
    Python,
    Analyse,
    Download,
}
fn main() {
    if let Err(e) = run() {
        eprintln!("{e:#?}");
    }
}

#[tokio::main]
async fn run() -> Result<(), anyhow::Error> {
    env_logger::Builder::new()
        .filter_level(log::LevelFilter::Info)
        .init();

    let file_name = format!("mmsdm_v{VERSION}.json");
    match AemoCodegen::from_args() {
        AemoCodegen::SqlServerTables => {
            sql_server_tables::run()?;
        }
        AemoCodegen::Rust => {
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
        AemoCodegen::Analyse => {
            analyse::run()?;
        }
        AemoCodegen::Json => {
            let dm = json::run().await?;

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
        AemoCodegen::Download => {
            download::run().await?;
        }
        AemoCodegen::Python => {
            python::run()?;
        }
    }
    Ok(())
}

fn swap_nonreq<T>(or: Option<anyhow::Result<T>>) -> anyhow::Result<Option<T>> {
    match or {
        Some(Ok(o)) => Ok(Some(o)),
        Some(Err(e)) => Err(e),
        None => Ok(None),
    }
}

const KW: [&str; 51] = [
    "as", "break", "const", "continue", "crate", "else", "enum", "extern", "false", "fn", "for",
    "if", "impl", "in", "let", "loop", "match", "mod", "move", "mut", "pub", "ref", "return",
    "self", "Self", "static", "struct", "super", "trait", "true", "type", "unsafe", "use", "where",
    "while", "async", "await", "dyn", "abstract", "become", "box", "do", "final", "macro",
    "override", "priv", "typeof", "unsized", "virtual", "yield", "try",
];
