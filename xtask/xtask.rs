use cargo::core::compiler::BuildConfig;
use cargo::core::Workspace;
use cargo::ops;
use cargo::ops::CompileOptions;
use cargo::ops::Packages;
use cargo::util::command_prelude::CompileMode;
use cargo::Config;
use std::env;
use std::path::Path;
use std::thread;
use std::process;

fn main() -> anyhow::Result<()> {
    let config = Config::default()?;
    let workspace = Workspace::new(&Path::new("./Cargo.toml").canonicalize()?, &config)?;

    if let Some(arg) = env::args().nth(1) {
        match arg.as_str() {
            "rust" => {
                let base_compile_options = CompileOptions::new(&config, CompileMode::Build)?;
                let build_config = BuildConfig::new(
                    &config,
                    Some(thread::available_parallelism()?.get().try_into()?),
                    false,
                    &[],
                    CompileMode::Build,
                )?;
                let local_build_options = CompileOptions {
                    spec: Packages::Packages(Vec::from(["mmsdm-codegen".to_string()])),
                    build_config,
                    ..base_compile_options
                };

                // can't use ops::run as it subsubmes the whole process!
                ops::compile(&workspace, &local_build_options)?;

                process::Command::new("target/debug/mmsdm-codegen")
                    .arg("rust")
                    .status()?;
                println!("Generated rust structures");

                process::Command::new("target/debug/mmsdm-codegen")
                    .arg("sql-server-rust-part")
                    .status()?;
                println!("Generated sql server - rust interaction");

                process::Command::new("target/debug/mmsdm-codegen")
                    .arg("sql-server-tables")
                    .status()?;
                println!("Generated sql server tables");
            }
            // "python" => {
            //     codegen_cmd("python")?;
            // }
            "example" => {
                if let Some(_arg) = env::args().nth(2) {
                    // match arg.as_str() {
                    //     "load" => {
                    //         xshell::cmd!("cargo run --example load --release --features sql_server,dispatch,settlement_data")
                    //         .env("RUST_LOG", "INFO")
                    //         .run().unwrap()
                    //     }
                    //     "parquet" => {
                    //         xshell::cmd!("cargo run --example parquet --release --features save_as_parquet,dispatch")
                    //         .env("RUST_LOG", "INFO")
                    //         .env("RUST_BACKTRACE", "1")
                    //         .run().unwrap()
                    //     }
                    //     "download" => {
                    //         xshell::cmd!("cargo run --example download_files --release")
                    //         .env("RUST_LOG", "INFO")
                    //         .run().unwrap()
                    //     }
                    //     other => {
                    //         println!("Argument {} is not recognised as an example", other);
                    //     }
                    // }
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
