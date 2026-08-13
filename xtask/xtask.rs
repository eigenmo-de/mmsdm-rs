use cargo::GlobalContext;
use cargo::core::Workspace;
use cargo::core::compiler::BuildConfig;
use cargo::core::compiler::UserIntent;
use cargo::ops;
use cargo::ops::CompileOptions;
use cargo::ops::Packages;
use cargo::util::context::JobsConfig;
use std::env;
use std::path::Path;
use std::process;

fn main() -> anyhow::Result<()> {
    let config = GlobalContext::default()?;
    let workspace = Workspace::new(&Path::new("./Cargo.toml").canonicalize()?, &config)?;

    if let Some(arg) = env::args().nth(1) {
        let x = arg.as_str();

        let base_compile_options = CompileOptions::new(&config, UserIntent::Build)?;
        let build_config = BuildConfig::new(
            &config,
            Some(JobsConfig::String("default".to_string())),
            false,
            &[],
            UserIntent::Build,
        )?;
        let local_build_options = CompileOptions {
            spec: Packages::Packages(Vec::from(["mmsdm-codegen".to_string()])),
            build_config,
            ..base_compile_options
        };

        // can't use ops::run as it subsubmes the whole process!
        ops::compile(&workspace, &local_build_options)?;

        process::Command::new("target/debug/mmsdm-codegen")
            .arg(x)
            .status()?;
        println!("Generated {x} mmsdm output");
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
