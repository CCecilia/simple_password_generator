/*!
# simple_password_generator

## CLI

Running from the binary file

```bash
$ ./spg --length 16
```

```bash
$ Password: D2sb9NV7@XjQQ&#
```
### Available Flags

| Short  | Long         | Description                         | Default |   |   |   |   |   |   |
|--------|--------------|-------------------------------------|---------|---|---|---|---|---|---|
|        | --length     | Length of password                  | 8       |   |   |   |   |   |   |
| -u     | --ucase-only | Only use uppercase aplha characters | false   |   |   |   |   |   |   |
| -l     | --lcase-only | Only use lowercase aplha characters | false   |   |   |   |   |   |   |
| -n     | --no-numbers | Don't include any number            | false   |   |   |   |   |   |   |
| -s     | --no-special | Dont include any special characters | false   |   |   |   |   |   |   |
| -h     |              | Help                                |         |   |   |   |   |   |   |
| -v     |              | version                             |         |   |   |   |   |   |   |
|        |              |                                     |         |   |   |   |   |   |   |
|        |              |                                     |         |   |   |   |   |   |   |
*/
use clap::Parser;

use simple_password_generator::{clipboard, generator::PasswordGenerator};
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
    #[arg(short = 'c', long, help = "Copy password to clipboard")]
    copy_to_clipboard: bool,
}

fn main() {
    let cli = Cli::parse();
    let password_length = cli.length;
    let lowercase_only = cli.lcase_only;
    let uppercase_only = cli.ucase_only;
    let no_numbers = cli.no_numbers;
    let no_special_chars = cli.no_special;
    let copy_to_clipboard_flag = cli.copy_to_clipboard;

    if lowercase_only && uppercase_only {
        eprintln!(
            "Lowercase only (-l) and Uppercase only (-u) flags cannot be used simultaneously"
        );
        process::exit(1);
    }

    let password = PasswordGenerator::new()
        .length(password_length)
        .uppercase_only(uppercase_only)
        .lowercase_only(lowercase_only)
        .exclude_numbers(no_numbers)
        .exclude_special_chars(no_special_chars)
        .generate();

    if copy_to_clipboard_flag {
        let copy = clipboard::copy_to_clipboard(&password);

        match copy {
            Ok(()) => {
                println!("got ok from copy")
            }
            Err(_clipboard_error) => {
                eprintln!("Failed to copy to clip board")
            }
        }
    }

    println!("Password: {}", password);
}
