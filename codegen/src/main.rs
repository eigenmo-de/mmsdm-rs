use structopt::StructOpt;

mod analyse;
mod json;
mod mms;
mod pdr;
mod python;
mod rust;
mod sql_server_tables;

pub const VERSION: &str = "5.3";

#[derive(structopt::StructOpt)]
#[structopt(about = "Code generation on the MMS Data Model")]
enum AemoCodegen {
    Json,
    Rust,
    SqlServerTables,
    Python,
    Analyse,
}
fn main() {
    if let Err(e) = run() {
        eprintln!("{e:#?}");
    }
}

#[tokio::main]
async fn run() -> Result<(), anyhow::Error> {
    match AemoCodegen::from_args() {
        AemoCodegen::SqlServerTables => {
            sql_server_tables::run()?;
        }
        AemoCodegen::Rust => {
            rust::run()?;
        }
        AemoCodegen::Analyse => {
            analyse::run()?;
        }
        AemoCodegen::Json => {
            json::run().await?;
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
