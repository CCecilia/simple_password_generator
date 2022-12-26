//! # Simple Password Generator
//!
//! A library for generating passwords
use rand::Rng;
use std::char;

static DEFAULT_LENGTH: u8 = 8;
static ALPHABET: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z',
];
static NUMBERS: [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
static SPECIAL_CHARS: [char; 11] = ['!', '@', '#', '$', '%', '^', '&', '*', '(', ')', '~'];

/// PasswordGenerator struct for generatoring passwords
pub struct PasswordGenerator {
    /// Array of lowercase chars used when generating a password.
    pub lowercase_char_set: [char; 26],
    /// Array of uppercase chars used when generating a password.
    pub uppercase_char_set: [char; 26],
    /// Array of number chars used when generating a password.
    pub number_set: [char; 10],
    /// Array of special chars used when generating a password.
    pub spec_char_set: [char; 11],
    /// Array of composition codes used when generating a password.
    pub composition_codes: [CompositionCodes; 4],
    /// Length of password to be generated.
    ///
    /// Default `8`
    pub length: u8,
    /// Bool used to exclude uppercase composition code when generating a password.
    ///
    /// Default `false`
    pub lowercase_only: bool,
    /// Bool used to exclude lowercase composition code when generating a password.
    ///
    /// Default `false`
    pub uppercase_only: bool,
    /// Bool used to exclude number composition code when generating a password.
    ///
    /// Default `false`
    pub exclude_numbers: bool,
    /// Bool used to special character composition code when generating a password.
    ///
    /// Default `false`
    pub exclude_special_chars: bool,
}

/// The primary character types used when generating the composition of the password
pub enum CompositionCodes {
    Lowercase,
    Uppercase,
    Number,
    SpecialCharacter,
}

impl CompositionCodes {
    /// Outputs all compositions code in a array
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
    /// PasswordGenerator Constructor
    pub fn new() -> PasswordGenerator {
        let lowercase_char_set = ALPHABET;
        let uppercase_char_set = lowercase_char_set.map(|lower_char| {
            let uppered: Vec<char> = lower_char.to_uppercase().collect();
            uppered[0]
        });

        PasswordGenerator {
            lowercase_char_set,
            uppercase_char_set,
            number_set: NUMBERS,
            spec_char_set: SPECIAL_CHARS,
            composition_codes: CompositionCodes::all_to_array(),
            length: DEFAULT_LENGTH,
            lowercase_only: false,
            uppercase_only: false,
            exclude_numbers: false,
            exclude_special_chars: false,
        }
    }

    /// Builder function for setting password length
    ///
    /// # Examples
    ///
    /// ```
    /// use simple_password_generator::generator::PasswordGenerator;
    ///
    /// let expected_length = 16;
    /// let result = PasswordGenerator::new().length(expected_length).generate();
    ///
    /// assert_eq!(expected_length as usize, result.chars().count());
    /// ```
    pub fn length(mut self, length: u8) -> Self {
        self.length = length;

        self
    }

    /// Builder function for setting lowercase characters only
    ///
    /// # Examples
    ///
    /// ```
    /// use simple_password_generator::generator::PasswordGenerator;
    ///
    /// let test_password = PasswordGenerator::new().lowercase_only(true).generate();
    /// let mut contains_uppercase = false;
    ///
    ///	for c in test_password.chars() {
    /// 	if c.is_alphabetic() {
    /// 		if c.is_uppercase() {
    /// 			contains_uppercase = true
    /// 		}
    /// 	}
    /// }
    ///
    /// assert_eq!(false, contains_uppercase);
    /// ```
    pub fn lowercase_only(mut self, lowercase_only: bool) -> Self {
        self.lowercase_only = lowercase_only;

        self
    }

    /// Builder function for setting uppercase characters only
    ///
    /// # Examples
    ///
    /// ```
    /// use simple_password_generator::generator::PasswordGenerator;
    ///
    /// let test_password = PasswordGenerator::new().uppercase_only(true).generate();
    /// let mut contains_lowercase = false;
    ///
    /// for c in test_password.chars() {
    /// 	if c.is_alphabetic() {
    /// 		if c.is_lowercase() {
    /// 			contains_lowercase = true
    /// 		}
    /// 	}
    /// }
    ///
    /// assert_eq!(false, contains_lowercase);
    /// ```
    pub fn uppercase_only(mut self, uppercase_only: bool) -> Self {
        self.uppercase_only = uppercase_only;

        self
    }

    /// Builder function for excluding any numbers from password
    ///
    /// # Examples
    ///
    /// ```
    /// use simple_password_generator::generator::PasswordGenerator;
    ///
    /// let test_password = PasswordGenerator::new().exclude_numbers(true).generate();
    /// let mut contains_numbers = false;
    ///
    /// for c in test_password.chars() {
    /// 	if c.is_numeric() {
    /// 		contains_numbers = true
    /// 	}
    /// }
    ///
    /// assert_eq!(false, contains_numbers);
    /// ```
    pub fn exclude_numbers(mut self, exclude_numbers: bool) -> Self {
        self.exclude_numbers = exclude_numbers;

        self
    }

    /// Builder function for excluding any special characters from password
    ///
    /// # Examples
    ///
    /// ```
    /// use simple_password_generator::generator::PasswordGenerator;
    ///
    /// let test_password = PasswordGenerator::new()
    /// 	.exclude_special_chars(true)
    /// 	.generate();
    /// let spec_char_vec: Vec<char> = vec!['!', '@', '#', '$', '%', '^', '&', '*', '(', ')', '~'];
    /// let mut contains_special_chars = false;
    ///
    /// for c in test_password.chars() {
    /// 	if spec_char_vec.contains(&c) {
    /// 		contains_special_chars = true;
    /// 	}
    /// }
    ///
    /// assert_eq!(false, contains_special_chars);
    /// ```
    pub fn exclude_special_chars(mut self, exclude_special_chars: bool) -> Self {
        self.exclude_special_chars = exclude_special_chars;

        self
    }

    /// Returns a random lowercase char
    fn get_random_lowercase_char(&self) -> char {
        let mut rng = rand::thread_rng();
        let rnd_index = rng.gen_range(0..=self.lowercase_char_set.len() - 1);

        self.lowercase_char_set[rnd_index]
    }

    /// Returns a random uppercase char
    fn get_random_uppercase_char(&self) -> char {
        let mut rng = rand::thread_rng();
        let rnd_index = rng.gen_range(0..=self.uppercase_char_set.len() - 1);

        self.uppercase_char_set[rnd_index]
    }

    /// Returns a random number char
    fn get_random_number(&self) -> char {
        let mut rng = rand::thread_rng();
        let rnd_index = rng.gen_range(0..=self.number_set.len() - 1);

        self.number_set[rnd_index]
    }

    /// Returns a random special character char
    fn get_random_special_character(&self) -> char {
        let mut rng = rand::thread_rng();
        let rnd_index = rng.gen_range(0..=self.spec_char_set.len() - 1);

        self.spec_char_set[rnd_index]
    }

    /// Generates a random composition vec
    fn generate_random_composition(&self) -> Vec<&CompositionCodes> {
        let mut result: Vec<&CompositionCodes> = Vec::new();
        let mut rng = rand::thread_rng();
        let filtered_comp_codes = self
            .composition_codes
            .iter()
            .filter(|code| match code {
                CompositionCodes::Lowercase => !self.uppercase_only,
                CompositionCodes::Uppercase => !self.lowercase_only,
                CompositionCodes::Number => !self.exclude_numbers,
                CompositionCodes::SpecialCharacter => !self.exclude_special_chars,
            })
            .collect::<Vec<_>>();

        for _i in 0..self.length {
            let rnd_index = rng.gen_range(0..=filtered_comp_codes.len() - 1);
            let comp_char = filtered_comp_codes[rnd_index];

            result.push(comp_char)
        }

        result
    }

    /// Generates a random string following the input composition vec
    fn generate_random_string_from_composition(
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

    /// Generates a password
    ///
    /// # Examples
    ///
    /// ```
    /// use simple_password_generator::generator::PasswordGenerator;
    ///
    /// let password = PasswordGenerator::new().generate();
    /// ```
    pub fn generate(&self) -> String {
        let comp_code = self.generate_random_composition();

        self.generate_random_string_from_composition(comp_code)
    }
}

impl Default for PasswordGenerator {
    fn default() -> Self {
        Self::new()
    }
}
