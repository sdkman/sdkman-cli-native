use crate::utils::{
    constants::VAR_DIR,
    directory_utils::infer_sdkman_dir,
    file_utils::{check_file_exists, read_file_content},
};
use colored::Colorize;
use std::env::consts::{ARCH, OS};

const CLI_VERSION_FILE: &str = "version";
const NATIVE_VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(clap::Args, Debug)]
#[command(about = "Display the installed SDKMAN! version (script + native)")]
pub struct Args {
    /// Print only the native binary version
    #[arg(long)]
    pub native_only: bool,
}

pub fn run(args: Args) -> Result<(), i32> {
    if args.native_only {
        println!("{NATIVE_VERSION}");
        return Ok(());
    }

    let sdkman_dir = infer_sdkman_dir().map_err(|e| {
        eprintln!("failed to infer SDKMAN_DIR: {e}");
        1
    })?;

    let cli_version_path = sdkman_dir.join(VAR_DIR).join(CLI_VERSION_FILE);

    let cli_version = check_file_exists(&cli_version_path)
        .and_then(read_file_content)
        .map_err(|e| {
            eprintln!(
                "failed to read SDKMAN! script version at {}: {}",
                cli_version_path.display(),
                e
            );
            1
        })?;

    println!(
        "\n{}\nscript: {}\nnative: {} ({} {})\n",
        "SDKMAN!".bold().yellow(),
        cli_version,
        NATIVE_VERSION,
        OS,
        ARCH
    );

    Ok(())
}
