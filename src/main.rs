use clap::Parser;
use colored::Colorize;

#[derive(Parser)]
struct Options {
    #[clap(default_value = "Meow!")]
    /// What does the cat say?
    message: String,
    #[clap(short = 'd', long = "dead")]
    /// Make the cat appear dead
    dead: bool,
}

fn main() {
    let options = Options::parse();
    let message = options.message;
    
    if message.to_lowercase() == "woof" {
        eprintln!("A cat shouldn't bark like a dog.")
    }
    
    let eye = if options.dead {"x"} else {"o"};
    
    println!("{}", message.bright_purple().underline().on_purple());
    println!(" \\");
    println!("  \\");
    println!("     /\\_/\\");
    println!("    ( {eye} {eye} )");
    println!("    =( I )=");
}
