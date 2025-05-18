use std::{
    io::{Read, Seek},
    iter,
};

use anyhow::Context;
use log::info;
use rc_zip::fsm::{ArchiveFsm, DecompressOutcome, EntryFsm, FsmResult};
use structopt::StructOpt;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

mod analyse;
mod download;
mod html_tree;
mod json;
mod mms;
mod pdr;
mod rust;
mod sql_server_tables;

pub const VERSION: &str = "5.5";

#[derive(structopt::StructOpt)]
#[structopt(about = "Code generation on the MMS Data Model")]
enum AemoCodegen {
    Json,
    Rust,
    SqlServerTables,
    Analyse,
    Download,
    Unzip,
}
fn main() {
    if let Err(e) = run() {
        eprintln!("\n=====\nError running command:\n=====\n    {e:#?}");
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
            let mut data = String::new();
            let mut f = tokio::fs::OpenOptions::new()
                .read(true)
                .open(&file_name)
                .await
                .with_context(|| format!("Opening file: `{file_name}`"))?;

            f.read_to_string(&mut data).await?;
            let de = serde_json::from_str(&data)?;

            sql_server_tables::run(de)?;
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
        AemoCodegen::Unzip => unzip()?,
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

fn unzip() -> anyhow::Result<()> {
    use std::io::Write;

    let mut f = std::fs::File::open("./PUBLIC_ARCHIVE#BIDOFFERPERIOD#FILE42#202503010000.zip")?;

    let md = f.metadata()?;

    dbg!(&md);

    let mut fsm = ArchiveFsm::new(md.len());

    let mut buf = [0; 8192];

    let mut total_bytes = 0;
    // let mut n = 0;
    let archive = loop {
        // dbg!(fsm.wants_read());
        let current = f.stream_position()?;
        if let Some(pos) = fsm.wants_read() {
            if pos != current {
                f.seek(std::io::SeekFrom::Start(pos))?;
            }

            let len_read = f.read(&mut buf[..])?;
            total_bytes += len_read;
            if u64::try_from(total_bytes)? > 2 * md.len() {
                anyhow::bail!("boh");
            }
            // dbg!(len_read, &buf[0..5]);
            let mut sp = fsm.space();
            // dbg!(sp.len());
            sp.write_all(&buf[0..len_read])?;

            let wr = fsm.fill(len_read);
            // dbg!(wr);
        }

        fsm = match fsm.process()? {
            FsmResult::Continue(x) => x,
            FsmResult::Done(archive) => break archive,
        };
        // n += 1;
        // if n > 5 {
        //     anyhow::bail!("donneee");
        // }
    };

    dbg!(archive.size(), archive.entries().count());

    let first = archive.entries().next().unwrap();

    dbg!(
        &first.name,
        first.sanitized_name(),
        first.method,
        &first.comment,
        first.modified,
        first.header_offset,
        first.compressed_size,
        first.uncompressed_size,
        md.len()
    );
    f.seek(std::io::SeekFrom::Start(first.header_offset))?;

    let mut entry_fsm = EntryFsm::new(Some(first.clone()), None);

    let mut out_buf = iter::repeat_n(0, 100_000).collect::<Vec<_>>();

    let mut out_buf_start = 0;

    let mut outcome = DecompressOutcome::default();

    let mut total_read = 0;
    let mut total_decompress = 0;
    loop {
        if entry_fsm.wants_read() {
            //  let len_read = f.read(&mut buf[..]).context("read from file")?;
            // total_read += len_read;
            //  dbg!(len_read, total_read);
            //  if len_read > 0 {
            //      match entry_fsm.space().write_all(&buf[0..len_read]).context("write to space") {
            //         Ok(()) => {
            //             entry_fsm.fill(len_read);

            //         }
            //         Err(_) => {
            //             // do nothing
            //         }
            //      }

            //  }

            let len_read = f.read(entry_fsm.space())?;
            total_read += len_read;
            entry_fsm.fill(len_read);
        }

        entry_fsm = match entry_fsm.process(&mut out_buf[..]).context("process")? {
            FsmResult::Continue((fsm, x)) => {
                // dbg!(outcome, first.compressed_size, first.uncompressed_size);

                outcome = x;
                fsm
            }
            FsmResult::Done(_) => {
                break;
            }
        }
    }

    dbg!(outcome, total_read);

    Ok(())
}
