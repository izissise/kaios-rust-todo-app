extern crate failure;
extern crate sass_rs;
extern crate url;

use failure::Fallible;
use sass_rs::{compile_file, Options, OutputStyle};
use std::{
    env,
    fs::{copy, write},
    path::{Path, PathBuf},
    process::Command,
};

const CSS_FILE: &str = "style.css";
const SCSS_FILE: &str = "style.scss";

pub fn main() -> Fallible<()> {
    // Prepeare the complete style
    prepare_style()?;

    Ok(())
}

// fn run<F>(name: &str, mut configure: F) -> Fallible<()>
// where
//     F: FnMut(&mut Command) -> &mut Command,
// {
//     let mut command = Command::new(name);
//     let configured = configure(&mut command);
//     if !configured.status()?.success() {
//         panic!("failed to execute {:?}", configured);
//     }
//     Ok(())
// }

fn prepare_style() -> Fallible<()> {
    // Prepare the directory
    let out_dir = env::var("OUT_DIR")?;
    let mut target = PathBuf::from(out_dir);

    // Copy the scss file into the output directory
    target.push(SCSS_FILE);
    copy(format!("src/{}", SCSS_FILE), &target)?;
    // Build the file
    let mut options = Options::default();
    options.output_style = OutputStyle::Compressed;
    match compile_file(&target, options) {
        Err(error) => panic!(error),
        Ok(content) => {
            // Copy the file into the static directory
            target.pop();
            target.push(CSS_FILE);
            write(&target, content)?;
        }
    }

    Ok(())
}
