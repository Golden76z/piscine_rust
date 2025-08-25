use chrono::prelude::*;

// This will be the structure that will handle the errors
#[derive(Debug, Eq, PartialEq)]
pub struct FormError {
    pub form_values: (&'static str, String),
    pub date: String,
    pub err: &'static str,
}

impl FormError {
    pub fn new(field_name: &'static str, field_value: String, err: &'static str) -> Self {
        let now = Local::now();
        let date_str = now.format("%Y-%m-%d %H:%M:%S").to_string();

        FormError {
            form_values: (field_name, field_value),
            date: date_str,
            err,
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Form {
    pub name: String,
    pub password: String,
}

impl Form {
    pub fn validate(&self) -> Result<(), FormError> {
        // Validate name (first name must not be empty)
        if self.name.is_empty() {
            return Err(FormError::new(
                "name",
                self.name.clone(),
                "Username is empty",
            ));
        }

        // Validate password length (at least 8 characters)
        if self.password.len() < 8 {
            return Err(FormError::new(
                "password",
                self.password.clone(),
                "Password should be at least 8 characters long",
            ));
        }

        // Validate password composition (must have ASCII alphanumeric + symbols)
        let has_letters = self.password.chars().any(|c| c.is_ascii_alphabetic());
        let has_numbers = self.password.chars().any(|c| c.is_ascii_digit());
        let has_symbols = self.password.chars().any(|c| {
            c.is_ascii_punctuation() || c.is_ascii_graphic() && !c.is_ascii_alphanumeric()
        });

        if !has_letters || !has_numbers || !has_symbols {
            return Err(FormError::new(
                "password",
                self.password.clone(),
                "Password should be a combination of ASCII numbers, letters and symbols",
            ));
        }

        Ok(())
    }
}
