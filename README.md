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

| short  | long                | description                         | default |   |   |   |   |   |   |
|--------|---------------------|-------------------------------------|---------|---|---|---|---|---|---|
|        | --length            | Length of password                  | 8       |   |   |   |   |   |   |
| -u     | --ucase-only        | Only use uppercase aplha characters | false   |   |   |   |   |   |   |
| -l     | --lcase-only        | Only use lowercase aplha characters | false   |   |   |   |   |   |   |
| -n     | --no-numbers        | Don't include any number            | false   |   |   |   |   |   |   |
| -s     | --no-special        | Dont include any special characters | false   |   |   |   |   |   |   |
| -c     | --copy-to-clipboard | Copies password to system clipboard | false   |   |   |   |   |   |   |
| -h     |                     | Help                                |         |   |   |   |   |   |   |
| -v     |                     | version                             |         |   |   |   |   |   |   |
|        |                     |                                     |         |   |   |   |   |   |   |
|        |                     |                                     |         |   |   |   |   |   |   |


**Note**
The `--copy-to-clipboard` use on linux systems requires that [xclip](https://github.com/astrand/xclip) be installed.


## Lib

```rust
use simple_password_generator::PasswordGenerator;

fn main() {
    let password = PasswordGenerator::new().length(password_length).generate();

    println!("Password: {}", password);
}
```