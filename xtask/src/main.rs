
use std::{process, env, io::{self, BufRead}, thread, sync};

fn main() {
    if let Some(arg) = env::args().nth(1) {
        match arg.as_str() {
            "codegen" => codegen(),
            _ => help(),
        }
    } else {
        help()
    }
}

fn help() {
    println!(
"available options are:
    
    `cargo xtask codegen`
");
}


fn codegen() {
    println!("running codegen..");
    stream_output(
        "/usr/bin/cargo",
        &["run", "--release", "--package", "aemo-codegen", &env::args().nth(2).unwrap_or("help".into())],
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
