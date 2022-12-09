use clap::Parser;
use rand::Rng;
use std::{char, u32};

#[derive(Parser)]
#[command(name = "Password Generator")]
#[command(author = "Christian Cecilia <christian.cecilia1@gmail.com")]
#[command(version = "1.0")]
#[command(about = "Generates a password", long_about = None)]
struct Cli {
    #[arg(long)]
    length: u8,
}

struct CharacterSet {
    lowercase_char_set: [char; 26],
    uppercase_char_set: [char; 26],
    number_set: [i32; 9],
    spec_char_set: [char; 11],
    composition_codes: [char; 4],
}

impl CharacterSet {
    pub fn new() -> CharacterSet {
        let lowercase_char_set = [
            'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q',
            'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
        ];
        let uppercase_char_set = lowercase_char_set.map(|lower_char| {
            let uppered: Vec<char> = lower_char.to_uppercase().collect();
            uppered[0]
        });
        CharacterSet {
            lowercase_char_set,
            uppercase_char_set,
            number_set: [0; 9],
            spec_char_set: ['!', '@', '#', '$', '%', '^', '&', '*', '(', ')', '~'],
            composition_codes: ['L', 'U', 'N', 'S'],
        }
    }

    pub fn get_composition(&self, length: &u8) -> Vec<char> {
        let mut result: Vec<char> = Vec::new();
        let mut rng = rand::thread_rng();

        for _i in 0..length.clone() - 1 {
            let rnd_index = rng.gen_range(0..=self.composition_codes.len() - 1);
            let comp_char = self.composition_codes[rnd_index];

            result.push(comp_char)
        }

        result
    }

    fn get_random_lowercase_char(&self) -> char {
        let mut rng = rand::thread_rng();
        let rnd_index = rng.gen_range(0..=self.lowercase_char_set.len() - 1);

        self.lowercase_char_set[rnd_index]
    }

    fn get_random_uppercase_char(&self) -> char {
        let mut rng = rand::thread_rng();
        let rnd_index = rng.gen_range(0..=self.uppercase_char_set.len() - 1);

        self.uppercase_char_set[rnd_index]
    }

    fn get_random_number(&self) -> i32 {
        let mut rng = rand::thread_rng();
        let rnd_index = rng.gen_range(0..=self.number_set.len() - 1);

        self.number_set[rnd_index]
    }

    fn get_random_special_character(&self) -> char {
        let mut rng = rand::thread_rng();
        let rnd_index = rng.gen_range(0..=self.spec_char_set.len() - 1);

        self.spec_char_set[rnd_index]
    }

    pub fn generate_password(&self, composition: &Vec<char>) -> String {
        let comp_code = composition;
        let mut password: Vec<char> = Vec::new();

        for code in comp_code {
            match code {
                'L' => {
                    println!("got lowercase char");
                    let value = self.get_random_lowercase_char();
                    password.push(value);
                }
                'U' => {
                    println!("got uppercase char");
                    let value = self.get_random_uppercase_char();
                    password.push(value);
                }
                'N' => {
                    println!("got number char");
                    let rand_num = self.get_random_number();
                    let rand_num_u32 = rand_num as u32;
                    let rand_num_char = char::from_u32(rand_num_u32).unwrap();
                    password.push(rand_num_char);
                }
                'S' => {
                    println!("got special char");
                    let value = self.get_random_special_character();
                    password.push(value);
                }
                _ => {
                    println!("unknown special char")
                }
            }
        }

        password.into_iter().collect()
    }
}

fn main() {
    let cli = Cli::parse();
    let password_length = cli.length;
    let character_set = CharacterSet::new();
    let composition = character_set.get_composition(&password_length);
    let password = character_set.generate_password(&composition);

    println!(
        "password length: {}\ncomposition: {:?}",
        password_length, composition
    );
    println!("Password {:?}", password);
}
