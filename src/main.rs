use std::io::Read;

use colored::*;
use exitfailure::ExitFailure;
use failure::ResultExt;
use structopt::StructOpt;
#[derive(StructOpt)]
struct Options {
    #[structopt(default_value = "Hello, I'm a cat!")]
    /// What does the cat say?
    message: String,
    #[structopt(short, long)]
    dead: bool,
    #[structopt(short = "f", long = "file", parse(from_os_str))]
    /// Load the cat picture from the specified file
    catfile: Option<std::path::PathBuf>,
    #[structopt(short = "i", long = "stdin")]
    /// Read the message from standard input
    stdin: bool,
}

fn main() -> Result<(), ExitFailure> {
    let options = Options::from_args();
    let mut message = String::new();
    if options.stdin {
        std::io::stdin().read_to_string(&mut message)?;
    } else {
        message = options.message;
    }
    if message.to_lowercase() == "woof" {
        eprintln!("Error: Cats don't bark!");
    }
    println!();
    let eye = if options.dead { "X" } else { "O" };
    println!("{}", message.bright_yellow().underline().on_purple());
    match &options.catfile {
        Some(path) => {
            let cat_template = std::fs::read_to_string(path)
                .with_context(|_| format!("could not read file {:?}", path))?;
            let cat_picture = cat_template.replace("{eye}", &eye);
            println!("{}", cat_picture);
        }
        None => {
            println!(" \\");
            println!("  \\");
            println!("     /\\_/\\");
            println!("    ( {eye}w{eye} )", eye = eye.bright_red().bold());
            println!("    =( O )=");
        }
    }
    println!();

    Ok(())
}
