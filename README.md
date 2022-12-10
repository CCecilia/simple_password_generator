# simple_password_generator

## CLI

Running from the binary file

`$ ./simple_password_generator --length 16`

`$ Password: D2sb9NV7@XjQQ&#`


## Lib

```rust
use simple_password_generator::PasswordGenerator;

fn main() {
    let password = PasswordGenerator::new().length(password_length).generate();

    println!("Password: {}", password);
}
```