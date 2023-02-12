use clap::Parser;
use colored::Colorize;
use home;
use std::env;
use std::path::Path;
use std::process;

#[derive(Parser, Debug)]
#[command(
    bin_name = "sdk home",
    about = "sdk subcommand to output the path of a specific candidate version"
)]
struct Args {
    #[arg(required(true))]
    candidate: String,

    #[arg(required(true))]
    version: String,
}

fn main() {
    let args = Args::parse();
    let candidate = args.candidate;
    let version = args.version;
    let sdkman_dir = match env::var("SDKMAN_DIR") {
        Ok(dir) => dir,
        Err(_) => home::home_dir().unwrap().to_str().unwrap().to_string(),
    };

    let candidates_file = format!("{}/var/candidates", sdkman_dir);
    let valid_candidates = std::fs::read_to_string(candidates_file.as_str())
        .expect("panic! the candidates file is missing");
    if !valid_candidates.contains(candidate.as_str()) {
        eprint!("{} is not a valid candidate!", candidate.bold());
        process::exit(1);
    }

    let candidate_home = format!("{}/candidates/{}/{}", sdkman_dir, candidate, version);
    let candidate_path = Path::new(candidate_home.as_str());
    if candidate_path.is_dir() {
        println!("{}", candidate_home);
    } else {
        eprintln!(
            "{} {} is not installed on your system",
            candidate.bold(),
            version.bold()
        );
        process::exit(1);
    }
}
