use std::{
    env, fs,
    io::{self, BufRead},
    process, sync, thread,
};

fn main() -> anyhow::Result<()> {
    if let Some(arg) = env::args().nth(1) {
        match arg.as_str() {
            // "codegen" => codegen(env::args().nth(2)),
            // "full" => codegen(Some("")),
            "rust" => {
                codegen_cmd("rust")?;
                println!("Generated rust structures");
                for entry in fs::read_dir("./src/mmsdm")? {
                    let path = entry?.path();
                    cmd("cargo", &["fmt", "--", path.to_str().unwrap()])?;
                }
                println!("Formatted rust code");
                codegen_cmd("sql-server-tables")?;
                println!("Generated sql server tables");
                codegen_cmd("sql-server-rust-part")?;
                println!("Generated sql server - rust interaction");
                cmd("cargo", &["fmt", "--", "src/sql_server.rs"])?;
                println!("Formatted sql server - rust interaction");
                // codegen_cmd("clickhouse-tables")?;
                // codegen_cmd("clickhouse-rust-part")?;
                // codegen_cmd("parquet")?;
            }
            // cmd => {
            //     codegen_cmd(cmd)?;
            // },
            "python" => {
                codegen_cmd("python")?;
                // codegen_cmd("sql-server-tables")?;
                // codegen_cmd("clickhouse-tables")?;
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
"
    );
}

fn codegen_cmd(subcommand: &str) -> anyhow::Result<String> {
    cmd(
        "/usr/bin/cargo",
        &["run", "--package", "mmsdm-codegen", subcommand],
    )
}

fn cmd(cmd: &str, args: &[&str]) -> anyhow::Result<String> {
    let output = process::Command::new(cmd)
        .args(args)
        .stdout(process::Stdio::piped())
        .stderr(process::Stdio::piped())
        .output()?;
    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).into())
    } else {
        Err(anyhow::anyhow!(
            "Command failed with code {}: {}",
            &output.status,
            String::from_utf8_lossy(&output.stderr)
        ))
    }
}

fn codegen(arg: Option<String>) {
    println!("running codegen..");
    stream_output(
        "/usr/bin/cargo",
        &[
            "run",
            "--release",
            "--package",
            "mmsdm-codegen",
            &arg.unwrap_or("help".into()),
        ],
    );
}

fn stream_output(command: &str, args: &[&str]) {
    let mut out = process::Command::new(command)
        .args(args)
        .stdout(process::Stdio::piped())
        .stderr(process::Stdio::piped())
        .spawn()
        .expect(&format!("Failed to run command {}", command));

    let stdout = out.stdout.take().unwrap();
    let stderr = out.stderr.take().unwrap();
    let (shutdown_out, shutdown_out_rx) = sync::mpsc::channel();
    thread::spawn(move || {
        let mut buf = String::new();
        let mut bufreader = io::BufReader::new(stdout);
        while let Ok(_) = bufreader.read_line(&mut buf) {
            println!("{}", &buf);
            if let Ok(()) = shutdown_out_rx.try_recv() {
                break;
            }
        }
    });
    let (shutdown_err, shutdown_err_rx) = sync::mpsc::channel();
    thread::spawn(move || {
        let mut bufreader = io::BufReader::new(stderr);
        let mut buf = String::new();
        while let Ok(_) = bufreader.read_line(&mut buf) {
            eprintln!("{}", &buf);
            if let Ok(()) = shutdown_err_rx.try_recv() {
                break;
            }
        }
    });
    println!("Exit: {}", out.wait().unwrap());
    shutdown_err.send(()).unwrap();
    shutdown_out.send(()).unwrap();
}
