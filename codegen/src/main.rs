use structopt::StructOpt;

mod rust;
mod json;
//mod python;
//mod parquet;
mod sql_server_tables;
//mod sql_server_rust;
mod mms;
mod pdr;
mod clickhouse_rust;
mod clickhouse_tables;

#[derive(structopt::StructOpt)]
#[structopt(about = "Code generation on the MMS Data Model")]
enum AemoCodegen {
    Json,
    Rust,
    SqlServerTables,
    SqlServerRustPart,
    Python,
    ClickhouseTables,
    ClickhouseRustPart,
    Parquet,
}
#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    match AemoCodegen::from_args() {
        AemoCodegen::SqlServerRustPart => {
            todo!();
            //sql_server_rust::run();
        }
        AemoCodegen::SqlServerTables => {
            sql_server_tables::run()?;
        }
        AemoCodegen::Rust => {
            rust::run()?;
        }
        AemoCodegen::Json => {
            json::run().await?;
        }
        AemoCodegen::Python => {
            todo!();
            //python::run();
        }
        AemoCodegen::ClickhouseTables => {
            clickhouse_tables::run()?;
        }
        AemoCodegen::ClickhouseRustPart => {
            todo!();
            //clickhouse_rust::run();
        }
        AemoCodegen::Parquet => {
            todo!();
            //parquet::run();
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
