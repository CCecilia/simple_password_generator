use rand::Rng;
use std::char;

static DEFAULT_LENGTH: u8 = 8;

pub struct PasswordGenerator {
    lowercase_char_set: [char; 26],
    uppercase_char_set: [char; 26],
    number_set: [char; 10],
    spec_char_set: [char; 11],
    composition_codes: [CompositionCodes; 4],
    length: u8,
    lowercase_only: bool,
    uppercase_only: bool,
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
            number_set: ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'],
            spec_char_set: ['!', '@', '#', '$', '%', '^', '&', '*', '(', ')', '~'],
            composition_codes: CompositionCodes::all_to_array(),
            length: DEFAULT_LENGTH,
            lowercase_only: false,
            uppercase_only: false,
        }
    }

    pub fn length(mut self, length: u8) -> Self {
        self.length = length;

        self
    }

    pub fn lowercase_only(mut self, lowercase_only: bool) -> Self {
        self.lowercase_only = lowercase_only;

        self
    }

    pub fn uppercase_only(mut self, uppercase_only: bool) -> Self {
        self.uppercase_only = uppercase_only;

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

    fn get_random_number(&self) -> char {
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
        let filtered_comp_codes = self
            .composition_codes
            .iter()
            .filter(|code| match code {
                CompositionCodes::Lowercase => {
                    if self.uppercase_only {
                        false
                    } else {
                        true
                    }
                }
                CompositionCodes::Uppercase => {
                    if self.lowercase_only {
                        false
                    } else {
                        true
                    }
                }
                _ => true,
            })
            .collect::<Vec<_>>();

        for _i in 0..self.length {
            let rnd_index = rng.gen_range(0..=filtered_comp_codes.len() - 1);
            let comp_char = filtered_comp_codes[rnd_index];

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
                    let value = self.get_random_lowercase_char();
                    password.push(value);
                }
                CompositionCodes::Uppercase => {
                    let value = self.get_random_uppercase_char();
                    password.push(value);
                }
                CompositionCodes::Number => {
                    let value = self.get_random_number();
                    password.push(value);
                }
                CompositionCodes::SpecialCharacter => {
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

#[cfg(test)]
mod tests {
    use crate::PasswordGenerator;

    #[test]
    fn generates_password_to_expected_length() {
        let expected_length = 16;
        let result = PasswordGenerator::new().length(expected_length).generate();

        assert_eq!(expected_length as usize, result.chars().count());
    }

    #[test]
    fn generates_password_with_no_lowercase() {
        let test_password = PasswordGenerator::new().uppercase_only(true).generate();
        let mut contains_lowercase = false;

        for c in test_password.chars() {
            if c.is_alphabetic() {
                if c.is_lowercase() {
                    contains_lowercase = true
                }
            }
        }

        assert_eq!(false, contains_lowercase);
    }

    #[test]
    fn generates_password_with_no_uppercase() {
        let test_password = PasswordGenerator::new().lowercase_only(true).generate();
        let mut contains_uppercase = false;

        for c in test_password.chars() {
            if c.is_alphabetic() {
                if c.is_uppercase() {
                    contains_uppercase = true
                }
            }
        }

        assert_eq!(false, contains_uppercase);
    }
}
