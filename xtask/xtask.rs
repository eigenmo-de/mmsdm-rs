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
            // cmd => {
            //     codegen_cmd(cmd)?;
            // },
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
"
    );
}

fn codegen_cmd(subcommand: &str) -> anyhow::Result<String> {
    let res = xshell::cmd!("cargo run --package mmsdm-codegen -- {subcommand}").read()?;
    Ok(res)
}
