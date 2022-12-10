use rand::Rng;
use std::{char, u32};

pub struct PasswordGenerator {
    lowercase_char_set: [char; 26],
    uppercase_char_set: [char; 26],
    number_set: [i32; 9],
    spec_char_set: [char; 11],
    composition_codes: [CompositionCodes; 4],
    length: u8,
}

pub enum CompositionCodes {
    Lowercase,
    Uppercase,
    Number,
    SpecialCharacter,
}

impl CompositionCodes {
    pub fn all_to_array() -> [CompositionCodes; 4] {
        [
            CompositionCodes::Lowercase,
            CompositionCodes::Uppercase,
            CompositionCodes::Number,
            CompositionCodes::SpecialCharacter,
        ]
    }
}

impl PasswordGenerator {
    pub fn new() -> PasswordGenerator {
        let lowercase_char_set = [
            'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q',
            'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
        ];
        let uppercase_char_set = lowercase_char_set.map(|lower_char| {
            let uppered: Vec<char> = lower_char.to_uppercase().collect();
            uppered[0]
        });
        PasswordGenerator {
            lowercase_char_set,
            uppercase_char_set,
            number_set: [0; 9],
            spec_char_set: ['!', '@', '#', '$', '%', '^', '&', '*', '(', ')', '~'],
            composition_codes: CompositionCodes::all_to_array(),
            length: 8,
        }
    }

    pub fn length(mut self, length: u8) -> Self {
        self.length = length;

        self
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

    pub fn generate_random_composition(&self) -> Vec<&CompositionCodes> {
        let mut result: Vec<&CompositionCodes> = Vec::new();
        let mut rng = rand::thread_rng();

        for _i in 0..self.length - 1 {
            let rnd_index = rng.gen_range(0..=self.composition_codes.len() - 1);
            let comp_char = &self.composition_codes[rnd_index];

            result.push(comp_char)
        }

        result
    }

    pub fn generate_random_string_from_composition(
        &self,
        composition: Vec<&CompositionCodes>,
    ) -> String {
        let mut password: Vec<char> = Vec::new();

        for code in composition {
            match code {
                CompositionCodes::Lowercase => {
                    println!("got lowercase char");
                    let value = self.get_random_lowercase_char();
                    password.push(value);
                }
                CompositionCodes::Uppercase => {
                    println!("got uppercase char");
                    let value = self.get_random_uppercase_char();
                    password.push(value);
                }
                CompositionCodes::Number => {
                    println!("got number char");
                    let rand_num = self.get_random_number();
                    let rand_num_u32 = rand_num as u32;
                    let rand_num_char = char::from_u32(rand_num_u32).unwrap();
                    password.push(rand_num_char);
                }
                CompositionCodes::SpecialCharacter => {
                    println!("got special char");
                    let value = self.get_random_special_character();
                    password.push(value);
                }
            }
        }

        password.into_iter().collect()
    }

    pub fn generate(&self) -> String {
        let comp_code = self.generate_random_composition();
        let password = self.generate_random_string_from_composition(comp_code);

        password
    }
}
