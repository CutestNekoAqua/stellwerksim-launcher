use clap::Parser;
use std::fs::{self, File};
use std::io::{stdin, stdout, Read, Write};
use std::path::PathBuf;
use std::process::Command;

use platform_dirs::AppDirs;

#[derive(Parser, Debug)]
#[clap(author = "April John", version, about)]
/// Application configuration
struct Args {
    /// whether to be verbose
    #[arg(short = 'v')]
    verbose: bool,

    /// an optional name to greet
    #[arg()]
    login_file: Option<String>,
}

fn main() {
    let args = Args::parse();
    if args.verbose {
        println!("DEBUG {args:?}");
    }
    println!("Welcome to the StellwerkSim Launcher!");
    println!("Beware! A working internet connection is required at all times!");

    let app_dirs = AppDirs::new(Some("stellwerksim-launcher"), true).unwrap();
    let login_file_path = PathBuf::from(
        args.login_file
            .unwrap_or(app_dirs.data_dir.join("sts.jnlp").display().to_string()),
    );

    println!("Searching directory..");
    fs::create_dir_all(&app_dirs.data_dir).unwrap();
    println!("Opening config file..");

    let file = if login_file_path.exists() {
        File::open(login_file_path.clone()).unwrap()
    } else {
        eprintln!("ERROR! Could not find {}, StellwerkSim Launcher will exit. Please download your specific launch file from stellwerksim.de and put it in the directory just opened!", login_file_path.to_string_lossy());
        open::that(&app_dirs.data_dir);
        pause();
        return;
    };

    println!("Starting Stellwerk Sim..");

    let mut command = Command::new("javaws");
    command.arg("-headless");
    command.arg(login_file_path);
    if let Ok(mut child) = command.spawn() {
        println!("Child's ID is {}", &child.id());
        child.wait();
    } else {
        eprintln!("STS startup process didn't start!");
    }
}

fn pause() {
    let mut stdout = stdout();
    stdout.write(b"Press Enter to continue...").unwrap();
    stdout.flush().unwrap();
    stdin().read(&mut [0]).unwrap();
}
