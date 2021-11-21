use std::{env, fs};

fn main() -> anyhow::Result<()> {
    if let Some(arg) = env::args().nth(1) {
        match arg.as_str() {
            "rust" => {
                codegen_cmd("rust")?;
                println!("Generated rust structures");
                for entry in fs::read_dir("./src/data_model")? {
                    let path = entry?.path();
                    let str_path = path.to_str().unwrap();
                    xshell::cmd!("cargo fmt -- {str_path}").run()?;
                }
                println!("Formatted rust code");
                codegen_cmd("sql-server-tables")?;
                println!("Generated sql server tables");
                codegen_cmd("sql-server-rust-part")?;
                println!("Generated sql server - rust interaction");
                xshell::cmd!("cargo fmt -- src/sql_server.rs").run()?;
                println!("Formatted sql server - rust interaction");
            }
            "python" => {
                codegen_cmd("python")?;
            }
            "example" => {
                if let Some(arg) = env::args().nth(2) {
                    match arg.as_str() {
                        "load" => {
                            xshell::cmd!("cargo run --example load --release --features sql_server,dispatch,settlement_data")                            
                            .env("RUST_LOG", "INFO")
                            .run().unwrap()
                        }
                        "parquet" => {
                            xshell::cmd!("cargo run --example parquet --release --features save_as_parquet,dispatch")
                            .env("RUST_LOG", "INFO")
                            .env("RUST_BACKTRACE", "1")
                            .run().unwrap()
                        }
                        "download" => {
                            xshell::cmd!("cargo run --example download_files --release")
                            .env("RUST_LOG", "INFO")
                            .run().unwrap()
                        }
                        other => {
                            println!("Argument {} is not recognised as an example", other);
                        }
                    }
                } else {
                    println!(
                        "Second arg is required to run an example, options are
        `cargo xtask example load`
        `cargo xtask example parquet`
"
                    )
                }
            }
            _ => help(),
        }
    } else {
        help()
    };
    Ok(())
}

fn help() {
    println!(
        "available options are:
    
        `cargo xtask rust`
        `cargo xtask python`
        `cargo xtask example`
"
    );
}

fn codegen_cmd(subcommand: &str) -> anyhow::Result<String> {
    let res = xshell::cmd!("cargo run --package mmsdm-codegen -- {subcommand}").read()?;
    Ok(res)
}
