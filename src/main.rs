use clap::Parser;
use simple_password_generator::PasswordGenerator;
use std::process;

#[derive(Parser)]
#[command(name = "Password Generator")]
#[command(author = "Christian Cecilia <christian.cecilia1@gmail.com")]
#[command(version = "1.0")]
#[command(about = "Generates a password", long_about = None)]
struct Cli {
    #[arg(long, default_value_t = 8, help = "Length of password")]
    length: u8,
    #[arg(short, long, help = "Only use uppercase aplha characters")]
    ucase_only: bool,
    #[arg(short, long, help = "Only use lowercase aplha characters")]
    lcase_only: bool,
    #[arg(short, long, help = "Dont include any number")]
    no_numbers: bool,
    #[arg(short = 's', long, help = "Dont include any special characters")]
    no_special: bool,
}

fn main() {
    let cli = Cli::parse();
    let password_length = cli.length;
    let lowercase_only = cli.lcase_only;
    let uppercase_only = cli.ucase_only;
    let no_numbers = cli.no_numbers;
    let no_special_chars = cli.no_special;

    if lowercase_only {
        if uppercase_only {
            eprintln!(
                "Lowercase only (-l) and Uppercase only (-u) flags cannot be used simultaneously"
            );
            process::exit(1);
        }
    }

    let password = PasswordGenerator::new()
        .length(password_length)
        .uppercase_only(uppercase_only)
        .lowercase_only(lowercase_only)
        .exclude_numbers(no_numbers)
        .exclude_special_chars(no_special_chars)
        .generate();

    println!("Password: {}", password);
}
