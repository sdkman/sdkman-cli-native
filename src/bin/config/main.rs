use std::env;
use std::path::{PathBuf};

use edit::edit_file;

fn main() {
    let config = {
        let sdkman_dir = env::var("SDKMAN_DIR").expect("The environment variable SDKMAN_DIR is not set.");
        let mut sdkman_dir = PathBuf::from(sdkman_dir);

        sdkman_dir.push("etc");
        sdkman_dir.push("config");

        sdkman_dir
    };

    let _result = edit_file(config);
}
