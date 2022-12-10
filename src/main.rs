use clap::Parser;

use simple_password_generator::PasswordGenerator;

#[derive(Parser)]
#[command(name = "Password Generator")]
#[command(author = "Christian Cecilia <christian.cecilia1@gmail.com")]
#[command(version = "1.0")]
#[command(about = "Generates a password", long_about = None)]
struct Cli {
    #[arg(long)]
    length: u8,
}

fn main() {
    let cli = Cli::parse();
    let password_length = cli.length;
    // let pw_generator = PasswordGenerator::new();
    let password = PasswordGenerator::new().length(password_length).generate();

    println!("Password {:?}", password);
}
