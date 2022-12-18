# simple_password_generator


[![ci](https://github.com/CCecilia/simple_password_generator/actions/workflows/ci.yml/badge.svg)](https://github.com/CCecilia/simple_password_generator/actions/workflows/ci.yml)

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



## Lib

```rust
use simple_password_generator::PasswordGenerator;

fn main() {
    let password = PasswordGenerator::new().length(password_length).generate();

    println!("Password: {}", password);
}
```